use serde::Serialize;
use std::collections::BTreeMap;
use uuid::Uuid;

use super::player::{Player, SanitizedPlayer};
use crate::responses::ToResponse;

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
}
