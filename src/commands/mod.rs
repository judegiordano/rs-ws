use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use crate::responses::Response;

pub mod coordinates;
pub mod ping;

pub trait MessageHandler<T: DeserializeOwned>: Serialize {
    fn parse_from_slice(slice: &[u8]) -> Result<T, serde_json::Error> {
        serde_json::from_slice::<T>(slice)
    }

    fn response_handler(data: &[u8]) -> Result<Response>;
}
