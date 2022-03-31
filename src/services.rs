use crate::domain::commands::{AtmError, BankAccountCommand, BankAccountServices, CheckingError};
use crate::BankAccountCommandPayload;
use async_trait::async_trait;

// A helper class that is used to wrap commands with a command wrapper that provides
// access to all the services that will be needed for command processing.
pub struct HappyPathServicesFactory;

impl HappyPathServicesFactory {
    pub fn wrap_bank_account_command(
        &self,
        command: BankAccountCommandPayload,
    ) -> BankAccountCommand {
        BankAccountCommand {
            payload: command,
            services: Box::new(HappyPathBankAccountServices),
        }
    }
}

// A very simple "happy path" set of services that always succeed.
pub struct HappyPathBankAccountServices;

#[async_trait]
impl BankAccountServices for HappyPathBankAccountServices {
    async fn atm_withdrawal(&self, _atm_id: &str, _amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    async fn validate_check(
        &self,
        _account_id: &str,
        _check_number: &str,
    ) -> Result<(), CheckingError> {
        Ok(())
    }
}
