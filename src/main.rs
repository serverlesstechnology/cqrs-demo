#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::HashMap;
use std::io::Read;

use cqrs_es::{AggregateError, Command};
use iron::{Headers, Iron, IronResult, Request, Response, status};
use postgres::{Connection, TlsMode};
use postgres_es::{GenericQueryRepository, PostgresCqrs};
use router::Router;
use serde::de::DeserializeOwned;

use crate::aggregate::BankAccount;
use crate::commands::{DepositMoney, OpenAccount, WithdrawMoney, WriteCheck};
use crate::events::BankAccountEvent;
use crate::queries::{BankAccountQuery, SimpleLoggingQueryProcessor};

mod application;
mod aggregate;
mod events;
mod commands;
mod queries;

fn main() {
    let mut router = Router::new();
    router.get("/account/:query_id", account_query, "account_query");
    router.post("/account/:command_type/:aggregate_id", account_command, "account_command");
    Iron::new(router).http("localhost:3030").unwrap();
}

pub fn account_command(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let command_type = params.find("command_type").unwrap_or("");
    let aggregate_id = params.find("aggregate_id").unwrap_or("");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let result = match command_type {
        "openAccount" => process_command::<OpenAccount>(aggregate_id, payload),
        "depositMoney" => process_command::<DepositMoney>(aggregate_id, payload),
        "withdrawMoney" => process_command::<WithdrawMoney>(aggregate_id, payload),
        "writeCheck" => process_command::<WriteCheck>(aggregate_id, payload),
        _ => return Ok(Response::with(status::NotFound))
    };
    match result {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e) => e.clone(),
            };
            let mut response = Response::with((status::BadRequest, err_payload));
            response.headers = std_headers();
            Ok(response)
        }
    }
}

fn process_command<T>(aggregate_id: &str, payload: String) -> Result<(), AggregateError>
    where T: Command<BankAccount, BankAccountEvent> + DeserializeOwned
{
    let payload = match serde_json::from_str::<T>(payload.as_str()) {
        Ok(payload) => { payload }
        Err(err) => {
            return Err(AggregateError::TechnicalError(err.to_string()));
        }
    };
    let cqrs = cqrs_framework();
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    cqrs.execute_with_metadata(aggregate_id, payload, metadata)
}

pub fn account_query(req: &mut Request) -> IronResult<Response> {
    let query_id = req.extensions.get::<Router>().unwrap().find("query_id").unwrap_or("").to_string();

    let query_repo = AccountQuery::new("account_query", db_connection());
    match query_repo.load(query_id) {
        None => {
            Ok(Response::with(status::NotFound))
        }
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            let mut response = Response::with((status::Ok, body));
            response.headers = std_headers();
            Ok(response)
        }
    }
}

fn std_headers() -> Headers {
    let mut headers = Headers::new();
    let content_type = iron::headers::ContentType::json();
    headers.set(content_type);
    headers
}

type AccountQuery = GenericQueryRepository::<BankAccountQuery, BankAccount, BankAccountEvent>;

fn cqrs_framework() -> PostgresCqrs<BankAccount, BankAccountEvent> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut account_query_processor = AccountQuery::new("account_query", db_connection());
    account_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_query), Box::new(account_query_processor)])
}

fn db_connection() -> Connection {
    Connection::connect("postgresql://demo_user:demo_pass@localhost:5432/demo", TlsMode::None).unwrap()
}