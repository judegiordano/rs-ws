use serde::Serialize;
use std::collections::BTreeMap;
use uuid::Uuid;

use super::player::Player;
use crate::responses::ToResponse;

pub type PlayerMap = BTreeMap<Uuid, Player>;

const MAX_ROOM_CAPACITY: usize = 10;

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub players: PlayerMap,
}

#[derive(Debug, Serialize, Clone)]
pub struct SanitizedRoom {
    pub id: Uuid,
    pub name: String,
    pub players: Vec<Player>,
}

impl ToResponse for Room {}
impl ToResponse for SanitizedRoom {}

impl Room {
    pub fn sanitize(&self) -> SanitizedRoom {
        SanitizedRoom {
            id: self.id,
            name: self.name.to_string(),
            players: self.players.values().cloned().collect(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.players.len() == MAX_ROOM_CAPACITY
    }
}
