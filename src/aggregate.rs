use cqrs_es::{Aggregate, AggregateError};
use serde::{Deserialize, Serialize};

use crate::commands::BankAccountCommand;
use crate::events::{AccountOpened, BankAccountEvent, CustomerDepositedMoney, CustomerWithdrewCash, CustomerWroteCheck};

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    account_id: String,
    balance: f64,
}

impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;

    fn aggregate_type() -> &'static str {
        "account"
    }

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, AggregateError> {
        match command {
            BankAccountCommand::OpenAccount(payload) => {
                let event_payload = AccountOpened {
                    account_id: payload.account_id
                };
                Ok(vec![BankAccountEvent::AccountOpened(event_payload)])
            }
            BankAccountCommand::DepositMoney(payload) => {
                let balance = self.balance + payload.amount;
                let event_payload = CustomerDepositedMoney {
                    amount: payload.amount,
                    balance,
                };
                Ok(vec![BankAccountEvent::CustomerDepositedMoney(event_payload)])
            }
            BankAccountCommand::WithdrawMoney(payload) => {
                let balance = self.balance - payload.amount;
                if balance < 0_f64 {
                    return Err(AggregateError::new("funds not available"));
                }
                let event_payload = CustomerWithdrewCash {
                    amount: payload.amount,
                    balance,
                };
                Ok(vec![BankAccountEvent::CustomerWithdrewCash(event_payload)])
            }
            BankAccountCommand::WriteCheck(payload) => {
                let balance = self.balance - payload.amount;
                if balance < 0_f64 {
                    return Err(AggregateError::new("funds not available"));
                }
                let event_payload = CustomerWroteCheck {
                    check_number: payload.check_number,
                    amount: payload.amount,
                    balance,
                };
                Ok(vec![BankAccountEvent::CustomerWroteCheck(event_payload)])
            }
        }
    }

    fn apply(&mut self, event: &Self::Event) {
        match event {
            BankAccountEvent::AccountOpened(e) => {
                self.account_id = e.account_id.clone();
            }
            BankAccountEvent::CustomerDepositedMoney(e) => {
                self.balance = e.balance;
            }
            BankAccountEvent::CustomerWithdrewCash(e) => {
                self.balance = e.balance;
            }
            BankAccountEvent::CustomerWroteCheck(e) => {
                self.balance = e.balance;
            }
        }
    }
}

impl Default for BankAccount {
    fn default() -> Self {
        BankAccount {
            account_id: "".to_string(),
            balance: 0_f64,
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use crate::aggregate::BankAccount;
    use crate::commands::{BankAccountCommand, DepositMoney, WithdrawMoney, WriteCheck};
    use crate::events::{BankAccountEvent, CustomerDepositedMoney, CustomerWithdrewCash, CustomerWroteCheck};

    type AccountTestFramework = TestFramework<BankAccount>;

    #[test]
    fn test_deposit_money() {
        let expected = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(BankAccountCommand::DepositMoney(DepositMoney { amount: 200.0 }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_deposit_money_with_balance() {
        let previous = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        let expected = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 400.0 });
        AccountTestFramework::default()
            .given(vec![previous])
            .when(BankAccountCommand::DepositMoney(DepositMoney { amount: 200.0 }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money() {
        let previous = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        let expected = BankAccountEvent::CustomerWithdrewCash(CustomerWithdrewCash { amount: 100.0, balance: 100.0 });
        AccountTestFramework::default()
            .given(vec![previous])
            .when(BankAccountCommand::WithdrawMoney(WithdrawMoney { amount: 100.0 }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money_funds_not_available() {
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(BankAccountCommand::WithdrawMoney(WithdrawMoney { amount: 200.0 }))
            .then_expect_error("funds not available")
    }

    #[test]
    fn test_wrote_check() {
        let previous = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        let expected = BankAccountEvent::CustomerWroteCheck(CustomerWroteCheck { check_number: "1170".to_string(), amount: 100.0, balance: 100.0 });
        AccountTestFramework::default()
            .given(vec![previous])
            .when(BankAccountCommand::WriteCheck(WriteCheck { check_number: "1170".to_string(), amount: 100.0 }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_wrote_check_funds_not_available() {
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(BankAccountCommand::WriteCheck(WriteCheck { check_number: "1170".to_string(), amount: 100.0 }))
            .then_expect_error("funds not available")
    }
}
