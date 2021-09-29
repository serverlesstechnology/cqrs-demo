use crate::aggregate::BankAccount;
use cqrs_es::AggregateError;
use postgres_es::PostgresCqrs;
use std::collections::HashMap;

pub struct CommandService {
    cqrs: PostgresCqrs<BankAccount>,
}

impl CommandService {
    pub fn new(cqrs: PostgresCqrs<BankAccount>) -> Self {
        Self { cqrs }
    }

    pub async fn process_command(
        &self,
        aggregate_id: &str,
        payload: &[u8],
    ) -> Result<(), AggregateError> {
        // Deserialize the payload into a `BankAccountCommand`.
        let payload = match serde_json::from_slice(payload) {
            Ok(payload) => payload,
            Err(_) => {
                return Err(AggregateError::new("not a valid command"));
            }
        };
        let mut metadata = HashMap::new();
        // TODO: add additional metadata from the request headers
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        self.cqrs
            .execute_with_metadata(aggregate_id, payload, metadata)
            .await
    }
}
