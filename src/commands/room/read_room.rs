use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    commands::MessageHandler,
    responses::{error_message::ErrorMessage, Response},
    state::session::STATE,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadRoom {
    pub room_id: String,
    pub player_id: String,
}

impl MessageHandler for ReadRoom {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let data = Self::parse_from_slice(data)?;
        tracing::debug!("[READ ROOM]: [{:?}]", data);
        let state = STATE.lock().await;
        let room_id = Uuid::from_str(&data.room_id)?;
        let room = match state.get(&room_id) {
            Some(room) => room,
            None => return Ok(Response::error("room not found")),
        };
        let id = Uuid::from_str(&data.player_id)?;
        if !room.players.contains_key(&id) {
            return Ok(Response::Error(ErrorMessage {
                message: format!("you do not have access to this room"),
            }));
        }
        tracing::debug!("[ROOM {id}]: {:#?}", room);
        Ok(Response::ReadRoomSuccess(room.sanitize()))
    }
}
