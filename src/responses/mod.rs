use serde::{Deserialize, Serialize};

pub mod error_message;

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    CoordinatesOk = 0,
    Err,
}

impl ResponseType {
    pub fn as_byte(&self) -> u8 {
        match self {
            ResponseType::CoordinatesOk => 0,
            ResponseType::Err => 1,
        }
    }
}

pub trait ToResponse<T: Serialize>: Serialize {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error>;
}
