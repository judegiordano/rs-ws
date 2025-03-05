use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::responses::{Response, ToResponse};

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

impl ToResponse for CoordinatesOk {}

impl MessageHandler<Coordinates> for Coordinates {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let response = Self::parse_from_slice(data)?;
        tracing::debug!("[COORDINATES]: [{:?}]", response);
        Ok(Response::CoordinatesOk(CoordinatesOk { ok: true }))
    }
}
