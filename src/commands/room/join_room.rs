use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    commands::{room::response::JoinRoomSuccess, MessageHandler},
    responses::Response,
    state::{
        player::{Player, WebSocket},
        session::STATE,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoom {
    pub room_id: String,
    pub display_name: String,
}

impl MessageHandler for JoinRoom {
    async fn response_handler(data: &[u8], receiver: Arc<Mutex<WebSocket>>) -> Result<Response> {
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
        // TODO: provide optional player_id and attempt to rejoin as the same player
        // otherwise a new player is inserted on every connection
        let player_id = Uuid::new_v4();
        let username = data.display_name;
        room.players.insert(
            player_id,
            Player {
                id: player_id,
                session: receiver.clone(),
                display_name: username.to_string(),
            },
        );
        tracing::debug!("[ROOM]: {:#?}", room);
        room.broadcast(format!("{:?} has joined.", username)).await;
        Ok(Response::JoinRoomSuccess(JoinRoomSuccess { player_id }))
    }
}
