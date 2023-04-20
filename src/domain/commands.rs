use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BankAccountCommand {
    OpenAccount { account_id: String },
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64, atm_id: String },
    WriteCheck { check_number: String, amount: f64 },
}

impl BankAccountCommand {
    pub(crate) fn command_type(&self) -> &'static str {
        match self {
            BankAccountCommand::OpenAccount { .. } => "OpenAccount",
            BankAccountCommand::DepositMoney { .. } => "DepositMoney",
            BankAccountCommand::WithdrawMoney { .. } => "WithdrawMoney",
            BankAccountCommand::WriteCheck { .. } => "WriteCheck",
        }
    }
}
