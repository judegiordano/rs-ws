use anyhow::Result;
use serde::de::DeserializeOwned;

pub mod coordinates;

pub trait MessageHandler<T: DeserializeOwned> {
    fn parse_from_slice(slice: &[u8]) -> Result<T, serde_json::Error> {
        serde_json::from_slice::<T>(slice)
    }
}
