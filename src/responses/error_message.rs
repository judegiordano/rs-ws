use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::commands::MessageHandler;

use super::{ResponseType, ToResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ToResponse for ErrorMessage {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let signature = ResponseType::Err;
        let mut msg = vec![signature.as_byte()];
        let payload = serde_json::to_vec(&self)?;
        msg.extend(payload);
        Ok(msg)
    }
}

impl MessageHandler<ErrorMessage> for ErrorMessage {
    fn response_handler(_: &[u8]) -> Result<Box<dyn ToResponse>> {
        tracing::debug!("[UNHANDLED]");
        Ok(Box::new(ErrorMessage {
            message: "unknown command".to_string(),
        }))
    }
}
