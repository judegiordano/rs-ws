use error_message::ErrorMessage;
use serde::Serialize;

use crate::{
    commands::{
        coordinates::CoordinatesOk,
        health::ping::Pong,
        room::response::{CreateRoomSuccess, JoinRoomSuccess, RoomBroadcast},
    },
    state::room::SanitizedRoom,
};

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
    JoinRoomSuccess(JoinRoomSuccess),
    CreateRoomSuccess(CreateRoomSuccess),
    ReadRoomSuccess(SanitizedRoom),
    RoomBroadcast(RoomBroadcast),
}

impl Response {
    pub fn response_signature(&self) -> u8 {
        match self {
            Self::Error(_) => 0,
            Self::CoordinatesOk(_) => 1,
            Self::Pong(_) => 2,
            Self::JoinRoomSuccess(_) => 3,
            Self::CreateRoomSuccess(_) => 4,
            Self::ReadRoomSuccess(_) => 5,
            Self::RoomBroadcast(_) => 6,
        }
    }

    pub fn response_body(&self) -> Result<Vec<u8>, serde_json::Error> {
        match self {
            Self::Error(data) => data.as_bytes(),
            Self::CoordinatesOk(data) => data.as_bytes(),
            Self::Pong(data) => data.as_bytes(),
            Self::JoinRoomSuccess(data) => data.as_bytes(),
            Self::CreateRoomSuccess(data) => data.as_bytes(),
            Self::ReadRoomSuccess(data) => data.as_bytes(),
            Self::RoomBroadcast(data) => data.as_bytes(),
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

    pub fn error<'b>(message: &str) -> Response {
        Self::Error(ErrorMessage {
            message: message.to_string(),
        })
    }
}
