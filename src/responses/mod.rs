use serde::Deserialize;

pub mod error_message;

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    Err = 0,
    CoordinatesOk,
    Pong,
}

impl ResponseType {
    pub fn as_byte(&self) -> u8 {
        match self {
            ResponseType::Err => 0,
            ResponseType::CoordinatesOk => 1,
            ResponseType::Pong => 2,
        }
    }
}

pub trait ToResponse {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error>;
}
