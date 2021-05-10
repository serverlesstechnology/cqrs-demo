#[cfg(test)]
mod simple_application_tests {
    use cqrs_es::CqrsFramework;
    use cqrs_es::mem_store::MemStore;

    use crate::aggregate::BankAccount;
    use crate::commands::{BankAccountCommand, DepositMoney};
    use crate::queries::SimpleLoggingQueryProcessor;

    #[test]
    fn test_event_store_single_command() {
        let event_store = MemStore::<BankAccount>::default();
        let query = SimpleLoggingQueryProcessor {};
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)]);
        cqrs.execute("test_id", BankAccountCommand::DepositMoney(DepositMoney { amount: 1000_f64 })).unwrap()
    }
}