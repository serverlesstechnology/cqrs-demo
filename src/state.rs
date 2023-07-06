use crate::config::cqrs_framework;
use crate::domain::aggregate::BankAccount;
use crate::queries::BankAccountView;
use std::sync::Arc;
use dynamo_es::{DynamoCqrs, DynamoViewRepository};

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<DynamoCqrs<BankAccount>>,
    pub account_query: Arc<DynamoViewRepository<BankAccountView, BankAccount>>,
}

pub async fn new_application_state() -> ApplicationState {
    // Configure the CQRS framework, backed by a Postgres database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    let region = aws_sdk_dynamodb::config::Region::new("us-west-2");
    let credentials = aws_sdk_dynamodb::config::Credentials::new("TESTAWSID", "TESTAWSKEY", None, None, "");
    let config = aws_sdk_dynamodb::config::Config::builder()
        .region(region)
        .endpoint_url("http://localhost:8000")
        .credentials_provider(credentials)
        .build();
    let client = aws_sdk_dynamodb::client::Client::from_conf(config);

    // Example configuration for building a test client calling the real AWS DynamoDb.
    // Requires credentials to be configured (e.g., `aws configure`).
    //
    // let mut config = ::aws_config::load_from_env().await;
    // let client = aws_sdk_dynamodb::Client::new(&config);
    let (cqrs, account_query) = cqrs_framework(client);
    ApplicationState {
        cqrs,
        account_query,
    }
}
