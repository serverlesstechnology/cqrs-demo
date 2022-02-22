#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::{Extension, FromRequest, Path, RequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{AddExtensionLayer, Json, Router};
use postgres_es::{default_postgress_pool, PostgresCqrs};

use crate::config::cqrs_framework;
use crate::domain::aggregate::BankAccount;
use crate::domain::commands::BankAccountCommand;
use crate::queries::AccountQuery;

mod config;
mod domain;
mod queries;

#[tokio::main]
async fn main() {
    // Configure the CQRS framework using a Postgres database and two queries.
    // Database should automatically configure with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let pool = default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    let router = Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .layer(AddExtensionLayer::new(cqrs))
        .layer(AddExtensionLayer::new(account_query));

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

// Serves as our query endpoint to respond with the materialized BankAccountView
// for the requested account.
async fn query_handler(
    Path(account_id): Path<String>,
    Extension(query_repo): Extension<Arc<AccountQuery>>,
) -> Response {
    match query_repo.load(&account_id).await {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(view) => (StatusCode::OK, Json(view)).into_response(),
    }
}

// Serves as our command endpoint to make changes in our `BankAccount` aggregate
// for the requested account.
async fn command_handler(
    Path(account_id): Path<String>,
    Json(command): Json<BankAccountCommand>,
    Extension(cqrs): Extension<Arc<PostgresCqrs<BankAccount>>>,
    MetadataExtension(metadata): MetadataExtension,
) -> Response {
    match cqrs
        .execute_with_metadata(&account_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

struct MetadataExtension(HashMap<String, String>);

#[async_trait]
impl<B: Send> FromRequest<B> for MetadataExtension {
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("uri".to_string(), req.uri().to_string());
        let headers = match req.headers() {
            None => return Ok(MetadataExtension(metadata)),
            Some(headers) => headers,
        };
        if let Some(user_agent) = headers.get("User-Agent") {
            if let Ok(value) = user_agent.to_str() {
                metadata.insert("User-Agent".to_string(), value.to_string());
            }
        }
        Ok(MetadataExtension(metadata))
    }
}
