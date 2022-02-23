use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BankAccountCommand {
    OpenAccount { account_id: String },
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64, atm_id: String },
    WriteCheck { check_number: String, amount: f64 },
}

pub struct BankAccountCommandWrapper {
    pub payload: BankAccountCommand,
    pub services: Box<dyn BankAccountServices>,
}

// External services must be called during the processing of the command.
#[async_trait]
pub trait BankAccountServices: Sync + Send {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmClientError>;
    async fn validate_check(
        &self,
        account_id: &str,
        check_number: &str,
    ) -> Result<(), CheckingClientError>;
}

pub struct AtmClientError;
pub struct CheckingClientError;
