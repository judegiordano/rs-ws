use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::BTreeMap;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::responses::ToResponse;

#[derive(Debug, Serialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub display_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub players: BTreeMap<Uuid, Player>,
}

impl ToResponse for Room {}

type GameState = Mutex<BTreeMap<Uuid, Room>>;

lazy_static! {
    pub static ref STATE: GameState = Mutex::new(BTreeMap::new());
}
