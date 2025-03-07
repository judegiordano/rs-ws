use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::BTreeMap;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::responses::ToResponse;

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    #[allow(dead_code)]
    #[serde(skip)]
    pub id: Uuid,
    pub display_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub players: BTreeMap<Uuid, Player>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SanitizedRoom {
    pub id: String,
    pub name: String,
    pub players: Vec<Player>,
}

const MAX_ROOM_CAPACITY: usize = 10;

impl Room {
    pub fn sanitize(&self) -> SanitizedRoom {
        SanitizedRoom {
            id: self.id.to_string(),
            name: self.name.to_string(),
            players: self.players.values().cloned().collect(),
        }
    }

    pub fn has_capacity(&self) -> bool {
        self.players.len() == MAX_ROOM_CAPACITY
    }
}

impl ToResponse for Room {}
impl ToResponse for SanitizedRoom {}

type GameState = Mutex<BTreeMap<Uuid, Room>>;

lazy_static! {
    pub static ref STATE: GameState = Mutex::new(BTreeMap::new());
}
