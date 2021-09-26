use cqrs_es::{Aggregate, AggregateError};
use serde::{Deserialize, Serialize};

use crate::commands::BankAccountCommand;
use crate::events::BankAccountEvent;

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    account_id: String,
    balance: f64,
}

impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;

    // This identifier should be unique to the system.
    fn aggregate_type() -> &'static str {
        "account"
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, AggregateError> {
        match command {
            BankAccountCommand::OpenAccount { account_id } => {
                Ok(vec![BankAccountEvent::AccountOpened { account_id }])
            }
            BankAccountCommand::DepositMoney{amount} => {
                let balance = self.balance + amount;
                Ok(vec![BankAccountEvent::CustomerDepositedMoney {
                    amount,
                    balance,
                }])
            }
            BankAccountCommand::WithdrawMoney{amount} => {
                let balance = self.balance - amount;
                if balance < 0_f64 {
                    return Err(AggregateError::new("funds not available"));
                }
                Ok(vec![BankAccountEvent::CustomerWithdrewCash {
                    amount,
                    balance,
                }])
            }
            BankAccountCommand::WriteCheck{check_number, amount} => {
                let balance = self.balance - amount;
                if balance < 0_f64 {
                    return Err(AggregateError::new("funds not available"));
                }
                Ok(vec![BankAccountEvent::CustomerWroteCheck {
                    check_number,
                    amount,
                    balance,
                }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { account_id } => {
                self.account_id = account_id;
            }
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance;
            }
            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            }
            BankAccountEvent::CustomerWroteCheck {
                check_number: _,
                amount: _,
                balance,
            } => {
                self.balance = balance;
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
    use crate::events::BankAccountEvent;

    type AccountTestFramework = TestFramework<BankAccount>;

    #[test]
    fn test_deposit_money() {
        let expected = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(BankAccountCommand::DepositMoney(DepositMoney {
                amount: 200.0,
            }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_deposit_money_with_balance() {
        let previous = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        let expected = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 400.0,
        };
        AccountTestFramework::default()
            .given(vec![previous])
            .when(BankAccountCommand::DepositMoney(DepositMoney {
                amount: 200.0,
            }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money() {
        let previous = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        let expected = BankAccountEvent::CustomerWithdrewCash {
            amount: 100.0,
            balance: 100.0,
        };
        AccountTestFramework::default()
            .given(vec![previous])
            .when(BankAccountCommand::WithdrawMoney(WithdrawMoney {
                amount: 100.0,
            }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money_funds_not_available() {
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(BankAccountCommand::WithdrawMoney(WithdrawMoney {
                amount: 200.0,
            }))
            .then_expect_error("funds not available")
    }

    #[test]
    fn test_wrote_check() {
        let previous = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        let expected = BankAccountEvent::CustomerWroteCheck {
            check_number: "1170".to_string(),
            amount: 100.0,
            balance: 100.0,
        };
        AccountTestFramework::default()
            .given(vec![previous])
            .when(BankAccountCommand::WriteCheck(WriteCheck {
                check_number: "1170".to_string(),
                amount: 100.0,
            }))
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_wrote_check_funds_not_available() {
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(BankAccountCommand::WriteCheck(WriteCheck {
                check_number: "1170".to_string(),
                amount: 100.0,
            }))
            .then_expect_error("funds not available")
    }
}
