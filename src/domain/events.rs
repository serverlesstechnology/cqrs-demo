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

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        match self {
            BankAccountEvent::AccountOpened { .. } => "AccountOpened".to_string(),
            BankAccountEvent::CustomerDepositedMoney { .. } => "CustomerDepositedMoney".to_string(),
            BankAccountEvent::CustomerWithdrewCash { .. } => "CustomerWithdrewCash".to_string(),
            BankAccountEvent::CustomerWroteCheck { .. } => "CustomerWroteCheck".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
