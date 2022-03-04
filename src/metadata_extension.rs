use std::collections::HashMap;
use std::convert::Infallible;
use axum::extract::{FromRequest, RequestParts};
use async_trait::async_trait;

// A custom Axum extension the builds metadata from the inbound request.
pub struct MetadataExtension(pub HashMap<String, String>);

#[async_trait]
impl<B: Send> FromRequest<B> for MetadataExtension {
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("uri".to_string(), req.uri().to_string());
        let headers = match req.headers() {
            None => return Ok(MetadataExtension(metadata)),
            Some(headers) => headers,
        };
        if let Some(user_agent) = headers.get("User-Agent") {
            if let Ok(value) = user_agent.to_str() {
                metadata.insert("User-Agent".to_string(), value.to_string());
            }
        }
        Ok(MetadataExtension(metadata))
    }
}
