use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::domain::aggregate::BankAccount;
use crate::queries::{AccountQuery, SimpleLoggingQuery};

pub fn cqrs_framework(pool: Pool<Postgres>) -> (Arc<PostgresCqrs<BankAccount>>, Arc<AccountQuery>) {
    // A very simple query that simply writes each event to stdout.
    let simple_query = SimpleLoggingQuery {};

    // A query that stores the current state of an individual account.
    let account_view_repo = PostgresViewRepository::new("account_query", pool.clone());
    let mut account_query = AccountQuery::new(account_view_repo);
    // It's essential to add an error handler. Without one the user will have no indication if an
    // error occurs (e.g., database connection failure, missing columns or table).
    account_query.use_error_handler(Box::new(|e| println!("{}", e)));

    // Create and return an event-sourced `CqrsFramework`.
    let account_query = Arc::new(account_query);
    let queries: Vec<Arc<dyn Query<BankAccount>>> =
        vec![Arc::new(simple_query), account_query.clone()];
    (
        Arc::new(postgres_es::postgres_cqrs(pool, queries)),
        account_query,
    )
}
