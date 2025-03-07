use serde::Serialize;
use uuid::Uuid;

use crate::responses::ToResponse;

#[derive(Debug, Serialize)]
pub struct JoinRoomSuccess {
    pub player_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateRoomSuccess {
    pub room_id: Uuid,
    pub name: String,
}

impl ToResponse for JoinRoomSuccess {}
impl ToResponse for CreateRoomSuccess {}
