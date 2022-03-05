#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{AddExtensionLayer, Json, Router};
use postgres_es::{default_postgress_pool, PostgresCqrs};

use crate::config::cqrs_framework;
use crate::domain::aggregate::BankAccount;
use crate::domain::commands::BankAccountCommand;
use crate::metadata_extension::MetadataExtension;
use crate::queries::AccountQuery;
use crate::services::HappyPathServicesFactory;

mod config;
mod domain;
mod metadata_extension;
mod queries;
mod services;

#[tokio::main]
async fn main() {
    // Configure the CQRS framework, backed by a Postgres database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let pool = default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    // Packaging all of the needed service calls into a single services factory.
    let services_factory = Arc::new(HappyPathServicesFactory);

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .layer(AddExtensionLayer::new(cqrs))
        .layer(AddExtensionLayer::new(services_factory))
        .layer(AddExtensionLayer::new(account_query));

    // Start the Axum server.
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested account.
async fn query_handler(
    Path(account_id): Path<String>,
    Extension(account_query): Extension<Arc<AccountQuery>>,
) -> Response {
    match account_query.load(&account_id).await {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(account_view) => (StatusCode::OK, Json(account_view)).into_response(),
    }
}

// Serves as our command endpoint to make changes in a `BankAccount` aggregate.
async fn command_handler(
    Path(account_id): Path<String>,
    Json(command): Json<BankAccountCommand>,
    Extension(services_factory): Extension<Arc<HappyPathServicesFactory>>,
    Extension(cqrs): Extension<Arc<PostgresCqrs<BankAccount>>>,
    MetadataExtension(metadata): MetadataExtension,
) -> Response {
    let command = services_factory.wrap_bank_account_command(command);
    match cqrs
        .execute_with_metadata(&account_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
