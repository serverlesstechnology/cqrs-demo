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
        let event_ser = format!("{{\"{}\":{}}}", payload_type, payload);
        let payload = match serde_json::from_str(event_ser.as_str()) {
            Ok(payload) => payload,
            Err(err) => {
                return Err(AggregateError::TechnicalError(err.to_string()));
            }
        };
        let mut metadata = HashMap::new();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        self.cqrs
            .execute_with_metadata(aggregate_id, payload, metadata)
            .await
    }
}
