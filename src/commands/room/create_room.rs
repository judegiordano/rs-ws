use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::{
    commands::{room::response::CreateRoomSuccess, MessageHandler},
    responses::Response,
    session::{Room, STATE},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoom {
    pub name: String,
}

impl MessageHandler<CreateRoom> for CreateRoom {
    async fn response_handler(data: &[u8]) -> Result<Response> {
        let data = Self::parse_from_slice(data)?;
        tracing::debug!("[CREATE ROOM]: [{:?}]", data);
        // TODO: check if room exists;
        let id = Uuid::new_v4();
        let mut state = STATE.lock().await;
        let room = Room {
            id,
            name: data.name.to_string(),
            players: BTreeMap::new(),
        };
        state.insert(id, room);
        Ok(Response::CreateRoomSuccess(CreateRoomSuccess {
            room_id: id.to_string(),
            name: data.name,
        }))
    }
}
