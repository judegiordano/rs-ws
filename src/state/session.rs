use lazy_static::lazy_static;
use std::collections::BTreeMap;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::room::Room;

pub type GameState = Mutex<BTreeMap<Uuid, Room>>;

lazy_static! {
    pub static ref STATE: GameState = Mutex::new(BTreeMap::new());
}
