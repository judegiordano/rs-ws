use serde::Serialize;

use super::{ResponseType, ToResponse};

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ToResponse<ErrorMessage> for ErrorMessage {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let signature = ResponseType::Err;
        let mut msg = vec![signature.as_byte()];
        let payload = serde_json::to_vec(&self)?;
        msg.extend(payload);
        Ok(msg)
    }
}

impl ErrorMessage {
    pub fn message_handler() -> Result<Vec<u8>, serde_json::Error> {
        tracing::debug!("[UNHANDLED]");
        let err = ErrorMessage {
            message: "unknown command".to_string(),
        };
        Ok(err.as_bytes()?)
    }
}
