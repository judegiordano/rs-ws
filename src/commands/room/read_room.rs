use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    commands::MessageHandler,
    responses::{error_message::ErrorMessage, Response},
    session::STATE,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadRoom {
    pub room_id: String,
}

impl MessageHandler<ReadRoom> for ReadRoom {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let data = Self::parse_from_slice(data)?;
        tracing::debug!("[READ ROOM]: [{:?}]", data);
        let state = STATE.lock().await;
        let id = Uuid::from_str(&data.room_id)?;
        let room = match state.get(&id) {
            Some(room) => room.to_owned(),
            None => {
                return Ok(Response::Error(ErrorMessage {
                    message: format!("room {id} not found"),
                }))
            }
        };
        tracing::debug!("[ROOM {id}]: {:#?}", room);
        Ok(Response::ReadRoomSuccess(room))
    }
}
