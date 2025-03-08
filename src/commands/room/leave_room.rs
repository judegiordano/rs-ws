use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    commands::{room::response::JoinRoomSuccess, MessageHandler},
    responses::Response,
    state::{player::WebSocket, session::STATE},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LeaveRoom {
    pub room_id: String,
    pub player_id: String,
}

impl MessageHandler for LeaveRoom {
    async fn response_handler(data: &[u8], _: Arc<Mutex<WebSocket>>) -> Result<Response> {
        let data = Self::parse_from_slice(data)?;
        tracing::debug!("[LEAVE ROOM]: [{:?}]", data);
        let mut state = STATE.lock().await;
        let room_id = Uuid::from_str(&data.room_id)?;
        let player_id = Uuid::from_str(&data.player_id)?;
        let room = match state.get_mut(&room_id) {
            Some(room) => room,
            None => return Ok(Response::error("room not found")),
        };
        let user = match room.players.remove(&player_id) {
            Some(player) => player,
            None => return Ok(Response::error("player not found in room")),
        };
        tracing::debug!("[ROOM]: {:#?}", room);
        room.broadcast(format!("{:?} has left.", &user.display_name))
            .await;
        Ok(Response::JoinRoomSuccess(JoinRoomSuccess { player_id }))
    }
}
