use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::aggregate::BankAccount;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened(AccountOpened),
    CustomerDepositedMoney(CustomerDepositedMoney),
    CustomerWithdrewCash(CustomerWithdrewCash),
    CustomerWroteCheck(CustomerWroteCheck),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountOpened {
    pub account_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerDepositedMoney {
    pub amount: f64,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerWithdrewCash {
    pub amount: f64,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerWroteCheck {
    pub check_number: String,
    pub amount: f64,
    pub balance: f64,
}

impl DomainEvent<BankAccount> for BankAccountEvent {
    fn apply(self, account: &mut BankAccount) {
        match self {
            BankAccountEvent::AccountOpened(e) => { e.apply(account) }
            BankAccountEvent::CustomerDepositedMoney(e) => { e.apply(account) }
            BankAccountEvent::CustomerWithdrewCash(e) => { e.apply(account) }
            BankAccountEvent::CustomerWroteCheck(e) => { e.apply(account) }
        }
    }
}

impl DomainEvent<BankAccount> for AccountOpened {
    fn apply(self, _account: &mut BankAccount) {    }
}
impl DomainEvent<BankAccount> for CustomerDepositedMoney {
    fn apply(self, account: &mut BankAccount) {
        account.balance = self.balance;
    }
}

impl DomainEvent<BankAccount> for CustomerWithdrewCash {
    fn apply(self, account: &mut BankAccount) {
        account.balance = self.balance;
    }
}

impl DomainEvent<BankAccount> for CustomerWroteCheck {
    fn apply(self, account: &mut BankAccount) {
        account.balance = self.balance;
    }
}