use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::responses::{ResponseType, ToResponse};

use super::MessageHandler;

#[derive(Debug, Deserialize)]
pub struct Ping {
    #[allow(dead_code)]
    pub ping: bool,
}

#[derive(Debug, Serialize)]
pub struct Pong {
    pub pong: bool,
}

impl MessageHandler<Ping> for Ping {
    fn response_handler(data: &[u8]) -> Result<Box<dyn ToResponse>> {
        let response = Self::parse_from_slice(data)?;
        tracing::debug!("[PING]: [{:?}]", response);
        Ok(Box::new(Pong { pong: true }))
    }
}

impl ToResponse for Pong {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let signature = ResponseType::Pong;
        let mut msg = vec![signature.as_byte()];
        let payload = serde_json::to_vec(&self)?;
        msg.extend(payload);
        Ok(msg)
    }
}
