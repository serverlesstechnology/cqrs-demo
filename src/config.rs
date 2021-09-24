use crate::aggregate::BankAccount;
use crate::queries::{BankAccountQuery, SimpleLoggingQueryProcessor};
use crate::service::CommandService;
use iron::{typemap, BeforeMiddleware, IronResult, Request};
use postgres_es::{default_postgress_pool, GenericQueryRepository, PostgresCqrs};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub type AccountQuery = GenericQueryRepository<BankAccountQuery, BankAccount>;

fn cqrs_framework(pool: Pool<Postgres>) -> PostgresCqrs<BankAccount> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut account_query_processor = AccountQuery::new("account_query", pool.clone());
    account_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(
        pool,
        vec![Box::new(simple_query), Box::new(account_query_processor)],
    )
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

impl BeforeMiddleware for ServiceInjector {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions
            .insert::<CommandServiceKey>(self.command_service());
        req.extensions
            .insert::<AccountQueryKey>(self.query_service());
        Ok(())
    }
}

pub struct CommandServiceKey {}
impl typemap::Key for CommandServiceKey {
    type Value = Arc<CommandService>;
}

pub struct AccountQueryKey {}
impl typemap::Key for AccountQueryKey {
    type Value = Arc<AccountQuery>;
}
