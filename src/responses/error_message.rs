use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::commands::MessageHandler;

use super::{Response, ToResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ToResponse for ErrorMessage {}

impl MessageHandler for ErrorMessage {
    async fn response_handler(_: &[u8]) -> Result<Response> {
        tracing::warn!("[UNHANDLED COMMAND]");
        Ok(Response::Error(ErrorMessage {
            message: "unhandled command".to_string(),
        }))
    }
}
