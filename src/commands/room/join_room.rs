use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    commands::{room::response::JoinRoomSuccess, MessageHandler},
    responses::Response,
    state::{player::Player, session::STATE},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoom {
    pub room_id: String,
    pub display_name: String,
}

impl MessageHandler for JoinRoom {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let data = Self::parse_from_slice(data)?;
        tracing::debug!("[JOIN ROOM]: [{:?}]", data);
        let mut state = STATE.lock().await;
        let room_id = Uuid::from_str(&data.room_id)?;
        let room = match state.get_mut(&room_id) {
            Some(room) => room,
            None => return Ok(Response::error("room not found")),
        };
        if room.is_full() {
            return Ok(Response::error("room is full"));
        }
        let player_id = Uuid::new_v4();
        room.players.insert(
            player_id,
            Player {
                id: room_id,
                display_name: data.display_name,
            },
        );
        tracing::debug!("[ROOM {room_id}]: {:#?}", room);
        Ok(Response::JoinRoomSuccess(JoinRoomSuccess { player_id }))
    }
}
