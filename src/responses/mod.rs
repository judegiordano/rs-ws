use error_message::ErrorMessage;
use serde::Serialize;

use crate::commands::{coordinates::CoordinatesOk, ping::Pong};

pub mod error_message;

pub trait ToResponse: Serialize {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

#[derive(Debug, Serialize)]
pub enum Response {
    Error(ErrorMessage),
    CoordinatesOk(CoordinatesOk),
    Pong(Pong),
}

impl Response {
    pub fn response_signature(&self) -> u8 {
        match self {
            Self::Error(_) => 0,
            Self::CoordinatesOk(_) => 1,
            Self::Pong(_) => 2,
        }
    }

    pub fn response_body(&self) -> Result<Vec<u8>, serde_json::Error> {
        match self {
            Self::Error(data) => data.as_bytes(),
            Self::CoordinatesOk(data) => data.as_bytes(),
            Self::Pong(data) => data.as_bytes(),
        }
    }

    // TODO: handle
    pub fn build_response(&self) -> Vec<u8> {
        let signature = self.response_signature();
        let mut data = vec![signature];
        let body = self.response_body().unwrap();
        data.extend(body);
        data
    }
}
