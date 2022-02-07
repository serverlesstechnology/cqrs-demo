use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{default_postgress_pool, PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::aggregate::BankAccount;
use crate::queries::{AccountQuery, SimpleLoggingQuery};
use crate::service::CommandService;

pub fn cqrs_framework(pool: Pool<Postgres>) -> (PostgresCqrs<BankAccount>, Arc<AccountQuery>) {
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
    (postgres_es::postgres_cqrs(pool, queries), account_query)
}

pub struct ServiceInjector {
    command_service: Arc<CommandService>,
    account_query: Arc<AccountQuery>,
}

impl ServiceInjector {
    pub async fn configured() -> Self {
        let pool =
            default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
        let (cqrs, account_query) = cqrs_framework(pool);
        let command_service = Arc::new(CommandService::new(cqrs));
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
