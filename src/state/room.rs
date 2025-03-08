use futures_util::SinkExt;
use serde::Serialize;
use std::collections::BTreeMap;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use super::player::{Player, SanitizedPlayer};
use crate::{
    commands::room::response::RoomBroadcast,
    responses::{Response, ToResponse},
};

pub type PlayerMap = BTreeMap<Uuid, Player>;

const MAX_ROOM_CAPACITY: usize = 10;

#[derive(Debug, Serialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub players: PlayerMap,
}

#[derive(Debug, Serialize)]
pub struct SanitizedRoom {
    pub id: Uuid,
    pub name: String,
    pub players: Vec<SanitizedPlayer>,
}

impl ToResponse for Room {}
impl ToResponse for SanitizedRoom {}

impl Room {
    pub fn sanitize(&self) -> SanitizedRoom {
        let players = self
            .players
            .iter()
            .map(|(_, player)| SanitizedPlayer {
                display_name: player.display_name.to_string(),
            })
            .collect();
        SanitizedRoom {
            id: self.id,
            name: self.name.to_string(),
            players,
        }
    }

    pub fn is_full(&self) -> bool {
        self.players.len() == MAX_ROOM_CAPACITY
    }

    pub async fn broadcast(&mut self, message: String) {
        let buffer = Response::RoomBroadcast(RoomBroadcast { message }).build_response();
        let msg = Message::binary(buffer);
        for (_, player) in &self.players {
            let client = &mut player.session.lock().await;
            match client.send(msg.clone()).await {
                Ok(_) => (),
                Err(err) => {
                    tracing::error!("[BROADCAST ERROR]: {:?}", err.to_string());
                }
            }
        }
    }
}
