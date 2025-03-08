use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{Response, ToResponse};
use crate::{commands::MessageHandler, state::player::WebSocket};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl ToResponse for ErrorMessage {}

impl MessageHandler for ErrorMessage {
    async fn response_handler(_: &[u8], _: Arc<Mutex<WebSocket>>) -> Result<Response> {
        tracing::warn!("[UNHANDLED COMMAND]");
        Ok(Response::Error(ErrorMessage {
            message: "unhandled command".to_string(),
        }))
    }
}
