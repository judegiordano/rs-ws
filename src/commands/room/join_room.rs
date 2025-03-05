use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    commands::{room::response::JoinRoomSuccess, MessageHandler},
    responses::{error_message::ErrorMessage, Response},
    session::{Player, STATE},
};

const MAX_ROOM_CAPACITY: usize = 10;

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoom {
    pub room_id: String,
    pub display_name: String,
}

impl MessageHandler<JoinRoom> for JoinRoom {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let data = Self::parse_from_slice(data)?;
        tracing::debug!("[JOIN ROOM]: [{:?}]", data);
        let mut state = STATE.lock().await;
        let id = Uuid::from_str(&data.room_id)?;
        let room = match state.get_mut(&id) {
            Some(room) => room,
            None => {
                return Ok(Response::Error(ErrorMessage {
                    message: format!("room {id} not found"),
                }))
            }
        };
        if room.players.len() == MAX_ROOM_CAPACITY {
            return Ok(Response::Error(ErrorMessage {
                message: format!("room {id} full"),
            }));
        }
        let player_id = Uuid::new_v4();
        room.players.insert(
            player_id,
            Player {
                id,
                display_name: data.display_name,
            },
        );
        tracing::debug!("[ROOM {id}]: {:#?}", room);
        Ok(Response::JoinRoomSuccess(JoinRoomSuccess { ok: true }))
    }
}
