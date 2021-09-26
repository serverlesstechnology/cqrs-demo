use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query, View};
use serde::{Deserialize, Serialize};

use crate::aggregate::BankAccount;
use crate::events::BankAccountEvent;
use postgres_es::GenericQuery;

pub struct SimpleLoggingQuery {}

// Our simplest query, this is great for debugging but absolutely useless in production.
// This query just pretty prints the events as they are processed.
#[async_trait]
impl Query<BankAccount> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

// Our second query, this one will be handled with Postgres `GenericQuery`
// which will serialize and persist our view after it is updated. It also
// provides a `load` method to deserialize the view on request.
pub type AccountQuery = GenericQuery<BankAccountView, BankAccount>;

// The view for a BankAccount query, for a standard http query this should
// be designed to reflect the dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BankAccountView {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
}

// This is implemented to update the view with events as they are committed.
// The logic should be minimal here, e.g., don't calculate the account balance
// here, instead design the events to carry that information.
impl View<BankAccount> for BankAccountView {
    fn update(&mut self, event: &EventEnvelope<BankAccount>) {
        match &event.payload {
            BankAccountEvent::AccountOpened { account_id } => {
                self.account_id = Some(account_id.clone());
            }
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = *balance;
            }
            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = *balance;
            }
            BankAccountEvent::CustomerWroteCheck {
                check_number,
                amount: _,
                balance,
            } => {
                self.balance = *balance;
                self.written_checks.push(check_number.clone())
            }
        }
    }
}
