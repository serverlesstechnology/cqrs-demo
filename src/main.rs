#![forbid(unsafe_code)]
#![deny(clippy::all)]

use cqrs_es::AggregateError;
use warp::{http::Response, Rejection, Reply};

use crate::config::ServiceInjector;
use crate::queries::AccountQuery;
use crate::service::CommandService;
use std::convert::Infallible;
use std::sync::Arc;
use warp::hyper::body::Bytes;
use warp::hyper::{Body, StatusCode};
use warp::Filter;

mod aggregate;
mod commands;
mod config;
mod events;
mod queries;
mod service;

#[tokio::main]
async fn main() {
    // Configure the CQRS framework using a Postgres database and two queries.
    // Database should automatically configure with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let services = ServiceInjector::configured().await;

    // Configure the query endpoint at `GET /account/{{accountId}}
    // This will load a view for the sumbitted `accountId` to add to the respoonse.
    let query = warp::get()
        .and(warp::path("account"))
        .and(warp::path::param())
        .and(with_query(services.query_service()))
        .and_then(query_handler);

    // Configure the command endpoint at `POST /account/:accountId`
    // Response is a 204 status if successful or a 400 with error message if the command fails.
    // For a failure example, try withdrawing more money than is available.
    let command = warp::post()
        .and(warp::path("account"))
        .and(warp::path::param())
        .and(warp::body::bytes())
        .and(with_command(services.command_service()))
        .and_then(command_handler);

    // Combines the command and query routes and starts a warp server.
    let routes = warp::any().and(query).or(command);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}

fn with_query(
    query: Arc<AccountQuery>,
) -> impl Filter<Extract = (Arc<AccountQuery>,), Error = Infallible> + Clone {
    warp::any().map(move || query.clone())
}

fn with_command(
    command_service: Arc<CommandService>,
) -> impl Filter<Extract = (Arc<CommandService>,), Error = Infallible> + Clone {
    warp::any().map(move || command_service.clone())
}

// Serves as our query endpoint to respond with the materialized BankAccountView
// for the requested account.
async fn query_handler(
    // The requested account id, injected by warp from the path parameter.
    account_id: String,
    // The account query repository, injected by warp from the configured `with_query` method.
    query_repo: Arc<AccountQuery>,
) -> std::result::Result<impl Reply, Rejection> {
    let response = match query_repo.load(&account_id).await {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty()),
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            Response::builder()
                .header(warp::http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
        }
    };
    Ok(response)
}

// Serves as our command endpoint to make changes in our `BankAccount` aggregate
// for the requested account.
async fn command_handler(
    // The requested account id, injected by warp from the path parameter.
    account_id: String,
    // The body of the request, injected by warp.
    payload: Bytes,
    // A command service for handling commands, injected by warp from the configured `with_command` method.
    command_service: Arc<CommandService>,
) -> std::result::Result<impl Reply, Rejection> {
    let payload = std::str::from_utf8(payload.as_ref()).unwrap().to_string();
    let result = command_service.process_command(account_id.as_str(), payload.as_bytes());
    match result.await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e) => e.to_string(),
                AggregateError::AggregateConflict => {
                    "command collision encountered, please try again".to_string()
                }
                AggregateError::DatabaseConnectionError(e) => e.to_string(),
                AggregateError::DeserializationError(e) => e.to_string(),
            };
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(warp::http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(err_payload)))
        }
    }
}
