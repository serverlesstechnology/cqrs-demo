use cqrs_es::{AggregateError, Command};
use serde::{Deserialize, Serialize};

use crate::aggregate::BankAccount;
use crate::events::{AccountOpened, BankAccountEvent, CustomerDepositedMoney, CustomerWithdrewCash, CustomerWroteCheck};

#[derive(Serialize, Deserialize)]
pub struct OpenAccount {
    pub account_id: String
}

#[derive(Serialize, Deserialize)]
pub struct DepositMoney {
    pub amount: f64
}

#[derive(Serialize, Deserialize)]
pub struct WithdrawMoney {
    pub amount: f64
}

#[derive(Serialize, Deserialize)]
pub struct WriteCheck {
    pub check_number: String,
    pub amount: f64,
}

impl Command<BankAccount, BankAccountEvent> for OpenAccount {
    fn handle(self, _account: &BankAccount) -> Result<Vec<BankAccountEvent>, AggregateError> {
        let event_payload = AccountOpened  {
            account_id: self.account_id
        };
        Ok(vec![BankAccountEvent::AccountOpened(event_payload)])
    }
}

impl Command<BankAccount, BankAccountEvent> for DepositMoney {
    fn handle(self, account: &BankAccount) -> Result<Vec<BankAccountEvent>, AggregateError> {
        let balance = account.balance + self.amount;
        let event_payload = CustomerDepositedMoney {
            amount: self.amount,
            balance,
        };
        Ok(vec![BankAccountEvent::CustomerDepositedMoney(event_payload)])
    }
}

impl Command<BankAccount, BankAccountEvent> for WithdrawMoney {
    fn handle(self, account: &BankAccount) -> Result<Vec<BankAccountEvent>, AggregateError> {
        let balance = account.balance - self.amount;
        if balance < 0_f64 {
            return Err(AggregateError::new("funds not available"));
        }
        let event_payload = CustomerWithdrewCash {
            amount: self.amount,
            balance,
        };
        Ok(vec![BankAccountEvent::CustomerWithdrewCash(event_payload)])
    }
}

impl Command<BankAccount, BankAccountEvent> for WriteCheck {
    fn handle(self, account: &BankAccount) -> Result<Vec<BankAccountEvent>, AggregateError> {
        let balance = account.balance - self.amount;
        if balance < 0_f64 {
            return Err(AggregateError::new("funds not available"));
        }
        let event_payload = CustomerWroteCheck {
            check_number: self.check_number,
            amount: self.amount,
            balance,
        };
        Ok(vec![BankAccountEvent::CustomerWroteCheck(event_payload)])
    }
}