use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{responses::Response, state::player::WebSocket};

pub mod coordinates;
pub mod health;
pub mod room;

pub trait MessageHandler: Serialize + DeserializeOwned {
    fn parse_from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice::<Self>(slice)
    }

    async fn response_handler(data: &[u8], receiver: Arc<Mutex<WebSocket>>) -> Result<Response>;
}
