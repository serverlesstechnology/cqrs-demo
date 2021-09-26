use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BankAccountCommand {
    OpenAccount(OpenAccount),
    DepositMoney(DepositMoney),
    WithdrawMoney(WithdrawMoney),
    WriteCheck(WriteCheck),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAccount {
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositMoney {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawMoney {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteCheck {
    pub check_number: String,
    pub amount: f64,
}
