use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BankAccountCommandPayload {
    OpenAccount { account_id: String },
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64, atm_id: String },
    WriteCheck { check_number: String, amount: f64 },
}

pub struct BankAccountCommand {
    pub payload: BankAccountCommandPayload,
    pub services: Box<dyn BankAccountServices>,
}

// External services must be called during the processing of the command.
#[async_trait]
pub trait BankAccountServices: Sync + Send {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError>;
    async fn validate_check(&self, account_id: &str, check: &str) -> Result<(), CheckingError>;
}

pub struct AtmError;
pub struct CheckingError;
