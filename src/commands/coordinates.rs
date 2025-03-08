use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    responses::{Response, ToResponse},
    state::player::WebSocket,
};

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

impl MessageHandler for Coordinates {
    async fn response_handler(data: &[u8], _: Arc<Mutex<WebSocket>>) -> Result<Response> {
        let response = Self::parse_from_slice(data)?;
        tracing::debug!("[COORDINATES]: [{:?}]", response);
        Ok(Response::CoordinatesOk(CoordinatesOk { ok: true }))
    }
}
