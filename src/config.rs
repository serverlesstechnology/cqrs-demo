use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{default_postgress_pool, GenericQuery, PostgresCqrs};
use sqlx::{Pool, Postgres};

use crate::aggregate::BankAccount;
use crate::queries::{BankAccountView, SimpleLoggingQuery};
use crate::service::CommandService;

pub type AccountQuery = GenericQuery<BankAccountView, BankAccount>;

pub fn cqrs_framework(pool: Pool<Postgres>) -> PostgresCqrs<BankAccount> {
    // A very simple query that simply writes each event to stdout.
    let simple_query = SimpleLoggingQuery {};

    // A query that stores the current state of an individual account.
    let mut account_query = AccountQuery::new("account_query", pool.clone());
    // It's essential to add an error handler. Without one the user will have no indication if an
    // error occurs (e.g., database connection failure, missing columns or table).
    account_query.use_error_handler(Box::new(|e| println!("{}", e)));

    // Create and return an event-sourced `CqrsFramework`.
    let queries: Vec<Box<dyn Query<BankAccount>>> =
        vec![Box::new(simple_query), Box::new(account_query)];
    postgres_es::postgres_cqrs(pool, queries)
}

pub struct ServiceInjector {
    command_service: Arc<CommandService>,
    account_query: Arc<AccountQuery>,
}

impl ServiceInjector {
    pub async fn configured() -> Self {
        let pool =
            default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
        let command_service = Arc::new(CommandService::new(cqrs_framework(pool.clone())));
        let account_query = Arc::new(AccountQuery::new("account_query", pool));
        Self {
            command_service,
            account_query,
        }
    }
    pub fn command_service(&self) -> Arc<CommandService> {
        self.command_service.clone()
    }
    pub fn query_service(&self) -> Arc<AccountQuery> {
        self.account_query.clone()
    }
}
