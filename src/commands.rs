use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BankAccountCommand {
    OpenAccount(OpenAccount),
    DepositMoney(DepositMoney),
    WithdrawMoney(WithdrawMoney),
    WriteCheck(WriteCheck),
}

#[derive(Serialize, Deserialize)]
pub struct OpenAccount {
    pub account_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DepositMoney {
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawMoney {
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WriteCheck {
    pub check_number: String,
    pub amount: f64,
}
