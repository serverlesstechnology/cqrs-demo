use crate::domain::commands::{
    AtmClientError, BankAccountCommandWrapper, BankAccountServices, CheckingClientError,
};
use crate::BankAccountCommand;
use async_trait::async_trait;

pub struct HappyPathBankAccountServices;

pub struct HappyPathServicesFactory;

impl HappyPathServicesFactory {
    pub fn wrap_bank_account_command(
        &self,
        command: BankAccountCommand,
    ) -> BankAccountCommandWrapper {
        BankAccountCommandWrapper {
            payload: command,
            services: Box::new(HappyPathBankAccountServices),
        }
    }
}

#[async_trait]
impl BankAccountServices for HappyPathBankAccountServices {
    async fn atm_withdrawal(&self, _atm_id: &str, _amount: f64) -> Result<(), AtmClientError> {
        Ok(())
    }

    async fn validate_check(
        &self,
        _account_id: &str,
        _check_number: &str,
    ) -> Result<(), CheckingClientError> {
        Ok(())
    }
}
