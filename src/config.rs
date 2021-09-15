use crate::aggregate::BankAccount;
use crate::queries::{BankAccountQuery, SimpleLoggingQueryProcessor};
use crate::service::CommandService;
use iron::{typemap, BeforeMiddleware, IronResult, Request};
use postgres_es::{Connection, GenericQueryRepository, PostgresCqrs};
use std::sync::Arc;

type AccountQuery = GenericQueryRepository<BankAccountQuery, BankAccount>;

fn cqrs_framework() -> PostgresCqrs<BankAccount> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut account_query_processor = AccountQuery::new("account_query", db_connection());
    account_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(
        db_connection(),
        vec![Box::new(simple_query), Box::new(account_query_processor)],
    )
}

fn db_connection() -> Connection {
    Connection::new("postgresql://demo_user:demo_pass@localhost:5432/demo")
}

pub struct CqrsMiddleware {
    command_service: Arc<CommandService>,
    account_query: Arc<AccountQuery>,
}

impl CqrsMiddleware {
    pub fn configured() -> Self {
        let command_service = Arc::new(CommandService::new(cqrs_framework()));
        let account_query = Arc::new(AccountQuery::new("account_query", db_connection()));
        Self {
            command_service,
            account_query,
        }
    }
    fn command_service(&self) -> Arc<CommandService> {
        self.command_service.clone()
    }
    fn query_service(&self) -> Arc<AccountQuery> {
        self.account_query.clone()
    }
}

impl BeforeMiddleware for CqrsMiddleware {
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
