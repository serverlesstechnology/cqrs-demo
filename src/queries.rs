use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};

use crate::aggregate::BankAccount;
use crate::events::BankAccountEvent;

pub struct SimpleLoggingQueryProcessor {}

impl QueryProcessor<BankAccount> for SimpleLoggingQueryProcessor {
    fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountQuery {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
}

impl Query<BankAccount> for BankAccountQuery {
    fn update(&mut self, event: &EventEnvelope<BankAccount>) {
        match &event.payload {
            BankAccountEvent::AccountOpened(payload) => {
                self.account_id = Some(payload.account_id.clone());
            }
            BankAccountEvent::CustomerDepositedMoney(payload) => {
                self.balance = payload.balance;
            }
            BankAccountEvent::CustomerWithdrewCash(payload) => {
                self.balance = payload.balance;
            }
            BankAccountEvent::CustomerWroteCheck(payload) => {
                self.balance = payload.balance;
                self.written_checks.push(payload.check_number.clone())
            }
        }
    }
}

impl Default for BankAccountQuery {
    fn default() -> Self {
        BankAccountQuery {
            account_id: None,
            balance: 0_f64,
            written_checks: Default::default(),
        }
    }
}
