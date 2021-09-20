#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::io::Read;
use warp::{http::Response, Reply, Rejection};
use cqrs_es::AggregateError;
// use iron::{status, Chain, Headers, Iron, IronResult, Request, Response};
// use router::Router;

use crate::config::{AccountQueryKey, CommandServiceKey, CqrsMiddleware, AccountQuery};
use warp::Filter;
use warp::hyper::{StatusCode, Body};
use std::convert::Infallible;
use std::sync::Arc;
use crate::queries::BankAccountQuery;
use crate::service::CommandService;
use warp::hyper::body::Bytes;

mod aggregate;
mod application;
mod commands;
mod config;
mod events;
mod queries;
mod service;

#[tokio::main]
async fn main() {
    let cqrs = CqrsMiddleware::configured();
    let query = warp::get()
        .and(warp::path("account"))
        .and(warp::path::param())
        .and(with_query(cqrs.query_service()))
        .and_then(query_handler);
    let query = warp::post()
        .and(warp::path("account"))
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::body::bytes())
        .and(with_command(cqrs.command_service()))
        .and_then(command_handler);

    warp::serve(query).run(([127, 0, 0, 1], 3030)).await

    // let mut router = Router::new();
    // router.get("/account/:query_id", account_query, "account_query");
    // router.post(
    //     "/account/:command_type/:aggregate_id",
    //     account_command,
    //     "account_command",
    // );
    // let mut chain = Chain::new(router);
    // chain.link_before(cqrs);
    // println!("Starting server at http://localhost:3030");
    // Iron::new(chain).http("localhost:3030").unwrap();
}
fn with_query(query: Arc<AccountQuery>) -> impl Filter<Extract = (Arc<AccountQuery>,), Error = Infallible> + Clone {
    warp::any().map(move || query.clone())
}

fn with_command(command_service: Arc<CommandService>) -> impl Filter<Extract = (Arc<CommandService>,), Error = Infallible> + Clone {
    warp::any().map(move || command_service.clone())
}

async fn query_handler(query_id: String, query_repo: Arc<AccountQuery>) -> std::result::Result<impl Reply, Rejection> {
    let response = match query_repo.load(query_id) {
        None => Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()),
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            Response::builder().body(Body::from(body))
        }
    };
    Ok(response)
}
async fn command_handler(command_type: String, aggregate_id: String, payload: Bytes, command_service: Arc<CommandService>) -> std::result::Result<impl Reply, Rejection> {
    let payload = std::str::from_utf8(payload.as_ref()).unwrap().to_string();
    let result = match command_type.as_str() {
        "openAccount" => command_service.process_command("OpenAccount", aggregate_id.as_str(), payload),
        "depositMoney" => command_service.process_command("DepositMoney", aggregate_id.as_str(), payload),
        "withdrawMoney" => command_service.process_command("WithdrawMoney", aggregate_id.as_str(), payload),
        "writeCheck" => command_service.process_command("WriteCheck", aggregate_id.as_str(), payload),
        _ => return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty())),
    };
    match result {
        Ok(_) => Ok(Response::builder().status(StatusCode::NO_CONTENT).body(Body::empty())),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e) => e.clone(),
            };
            let mut response = Response::builder().status(StatusCode::NOT_FOUND).body(Body::from(err_payload));
            // response.headers = std_headers();
            Ok(response)
        }
    }
}
// pub fn account_command(req: &mut Request) -> IronResult<Response> {
//     let command_service = req.extensions.get::<CommandServiceKey>().unwrap();
//     let params = req.extensions.get::<Router>().unwrap();
//     let command_type = params.find("command_type").unwrap_or("");
//     let aggregate_id = params.find("aggregate_id").unwrap_or("");
//     let mut payload = String::new();
//     req.body.read_to_string(&mut payload).unwrap();
//     let result = match command_type {
//         "openAccount" => command_service.process_command("OpenAccount", aggregate_id, payload),
//         "depositMoney" => command_service.process_command("DepositMoney", aggregate_id, payload),
//         "withdrawMoney" => command_service.process_command("WithdrawMoney", aggregate_id, payload),
//         "writeCheck" => command_service.process_command("WriteCheck", aggregate_id, payload),
//         _ => return Ok(Response::with(status::NotFound)),
//     };
//     match result {
//         Ok(_) => Ok(Response::with(status::NoContent)),
//         Err(err) => {
//             let err_payload = match &err {
//                 AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
//                 AggregateError::TechnicalError(e) => e.clone(),
//             };
//             let mut response = Response::with((status::BadRequest, err_payload));
//             response.headers = std_headers();
//             Ok(response)
//         }
//     }
// }

// pub fn account_query(req: &mut Request) -> IronResult<Response> {
//     let query_id = req
//         .extensions
//         .get::<Router>()
//         .unwrap()
//         .find("query_id")
//         .unwrap_or("")
//         .to_string();
//
//     let query_repo = req.extensions.get::<AccountQueryKey>().unwrap();
//     match query_repo.load(query_id) {
//         None => Ok(Response::with(status::NotFound)),
//         Some(query) => {
//             let body = serde_json::to_string(&query).unwrap();
//             let mut response = Response::with((status::Ok, body));
//             response.headers = std_headers();
//             Ok(response)
//         }
//     }
// }

// fn std_headers() -> Headers {
//     let mut headers = Headers::new();
//     let content_type = iron::headers::ContentType::json();
//     headers.set(content_type);
//     headers
// }
