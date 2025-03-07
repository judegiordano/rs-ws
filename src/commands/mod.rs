use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use crate::responses::Response;

pub mod coordinates;
pub mod ping;
pub mod room;

pub trait MessageHandler: Serialize + DeserializeOwned {
    fn parse_from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice::<Self>(slice)
    }

    async fn response_handler(data: &[u8]) -> Result<Response>;
}
