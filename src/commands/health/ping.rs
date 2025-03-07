use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    commands::MessageHandler,
    responses::{Response, ToResponse},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ping {
    #[allow(dead_code)]
    pub ping: bool,
}

#[derive(Debug, Serialize)]
pub struct Pong {
    pub pong: bool,
}

impl MessageHandler for Ping {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let response = Self::parse_from_slice(data)?;
        tracing::debug!("[PING]: [{:?}]", response);
        Ok(Response::Pong(Pong { pong: true }))
    }
}

impl ToResponse for Pong {}
