use std::sync::Arc;

use cqrs_es::Query;
use dynamo_es::{DynamoCqrs, dynamodb_cqrs, DynamoViewRepository};

use crate::domain::aggregate::BankAccount;
use crate::queries::{AccountQuery, BankAccountView, SimpleLoggingQuery};
use crate::services::{BankAccountServices, HappyPathBankAccountServices};

pub fn cqrs_framework(
    client: aws_sdk_dynamodb::client::Client,
) -> (
    Arc<DynamoCqrs<BankAccount>>,
    Arc<DynamoViewRepository<BankAccountView, BankAccount>>,
) {
    // A very simple query that writes each event to stdout.
    let simple_query = SimpleLoggingQuery {};

    // A query that stores the current state of an individual account.
    let account_view_repo = Arc::new(DynamoViewRepository::new("TestViewTable", client.clone()));
    let mut account_query = AccountQuery::new(account_view_repo.clone());

    // Without a query error handler there will be no indication if an
    // error occurs (e.g., database connection failure, missing columns or table).
    // Consider logging an error or panicking in your own application.
    account_query.use_error_handler(Box::new(|e| println!("Error in writing query: {:#?}", e)));

    // Create and return an event-sourced `CqrsFramework`.
    let queries: Vec<Box<dyn Query<BankAccount>>> =
        vec![Box::new(simple_query), Box::new(account_query)];
    let services = BankAccountServices::new(Box::new(HappyPathBankAccountServices));
    (
        Arc::new(dynamodb_cqrs(client, queries, services)),
        account_view_repo,
    )
}
