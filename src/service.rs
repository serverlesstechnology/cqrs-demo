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
        payload_type: &str,
        aggregate_id: &str,
        payload: String,
    ) -> Result<(), AggregateError> {
        // The serialization method used requires some formatting to deserialize the inbound
        // payload into the correct `BankAccountCommand` enum.
        // TODO: add helper methods to reduce this complexity
        let event_ser = format!("{{\"{}\":{}}}", payload_type, payload);
        // Deserialize the payload into a `BankAccountCommand`.
        let payload = match serde_json::from_str(event_ser.as_str()) {
            Ok(payload) => payload,
            Err(err) => {
                return Err(AggregateError::TechnicalError(err.to_string()));
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
