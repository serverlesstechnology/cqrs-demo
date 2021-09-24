#![forbid(unsafe_code)]
#![deny(clippy::all)]

use cqrs_es::AggregateError;
use warp::{http::Response, Rejection, Reply};

use crate::config::{AccountQuery, ServiceInjector};
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
    let services = ServiceInjector::configured().await;

    let query = warp::get()
        .and(warp::path("account"))
        .and(warp::path::param())
        .and(with_query(services.query_service()))
        .and_then(query_handler);

    let command = warp::post()
        .and(warp::path("account"))
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::body::bytes())
        .and(with_command(services.command_service()))
        .and_then(command_handler);

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

async fn query_handler(
    query_id: String,
    query_repo: Arc<AccountQuery>,
) -> std::result::Result<impl Reply, Rejection> {
    let response = match query_repo.load(query_id).await {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty()),
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            Response::builder().body(Body::from(body))
        }
    };
    Ok(response)
}
async fn command_handler(
    command_type: String,
    aggregate_id: String,
    payload: Bytes,
    command_service: Arc<CommandService>,
) -> std::result::Result<impl Reply, Rejection> {
    let payload = std::str::from_utf8(payload.as_ref()).unwrap().to_string();
    let result = match command_type.as_str() {
        "openAccount" => {
            command_service.process_command("OpenAccount", aggregate_id.as_str(), payload)
        }
        "depositMoney" => {
            command_service.process_command("DepositMoney", aggregate_id.as_str(), payload)
        }
        "withdrawMoney" => {
            command_service.process_command("WithdrawMoney", aggregate_id.as_str(), payload)
        }
        "writeCheck" => {
            command_service.process_command("WriteCheck", aggregate_id.as_str(), payload)
        }
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty()))
        }
    };
    match result.await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e) => e.clone(),
            };
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(err_payload)))
        }
    }
}

// fn std_headers() -> Headers {
//     let mut headers = Headers::new();
//     let content_type = iron::headers::ContentType::json();
//     headers.set(content_type);
//     headers
// }
