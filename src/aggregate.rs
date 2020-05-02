use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    pub balance: f64,
}

impl Aggregate for BankAccount {
    fn aggregate_type() -> &'static str {
        "account"
    }
}

impl Default for BankAccount {
    fn default() -> Self {
        BankAccount {
            balance: 0_f64
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use crate::aggregate::BankAccount;
    use crate::commands::{DepositMoney, WithdrawMoney, WriteCheck};
    use crate::events::{BankAccountEvent, CustomerDepositedMoney, CustomerWithdrewCash, CustomerWroteCheck};

    type AccountTestFramework = TestFramework<BankAccount, BankAccountEvent>;

    #[test]
    fn test_deposit_money() {
        let expected = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(DepositMoney { amount: 200.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_deposit_money_with_balance() {
        let previous = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        let expected = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 400.0 });
        AccountTestFramework::default()
            .given(vec![previous])
            .when(DepositMoney { amount: 200.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money() {
        let previous = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        let expected = BankAccountEvent::CustomerWithdrewCash(CustomerWithdrewCash { amount: 100.0, balance: 100.0 });
        AccountTestFramework::default()
            .given(vec![previous])
            .when(WithdrawMoney { amount: 100.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money_funds_not_available() {
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(WithdrawMoney { amount: 200.0 })
            .then_expect_error("funds not available")
    }

    #[test]
    fn test_wrote_check() {
        let previous = BankAccountEvent::CustomerDepositedMoney(CustomerDepositedMoney { amount: 200.0, balance: 200.0 });
        let expected = BankAccountEvent::CustomerWroteCheck(CustomerWroteCheck { check_number: "1170".to_string(), amount: 100.0, balance: 100.0 });
        AccountTestFramework::default()
            .given(vec![previous])
            .when(WriteCheck { check_number: "1170".to_string(), amount: 100.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_wrote_check_funds_not_available() {
        AccountTestFramework::default()
            .given_no_previous_events()
            .when(WriteCheck { check_number: "1170".to_string(), amount: 100.0 })
            .then_expect_error("funds not available")
    }
}
