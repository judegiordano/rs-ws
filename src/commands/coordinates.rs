use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::responses::{ResponseType, ToResponse};

use super::MessageHandler;

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Serialize)]
pub struct CoordinatesOk {
    pub ok: bool,
}

impl ToResponse for CoordinatesOk {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let signature = ResponseType::CoordinatesOk;
        let mut msg = vec![signature.as_byte()];
        let payload = serde_json::to_vec(&self)?;
        msg.extend(payload);
        Ok(msg)
    }
}

impl MessageHandler<Coordinates> for Coordinates {
    fn response_handler(data: &[u8]) -> Result<Box<dyn ToResponse>> {
        let response = Self::parse_from_slice(data)?;
        tracing::debug!("[COORDINATES]: [{:?}]", response);
        Ok(Box::new(CoordinatesOk { ok: true }))
    }
}
