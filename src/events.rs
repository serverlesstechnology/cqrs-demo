use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {}
