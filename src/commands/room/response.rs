use serde::Serialize;

use crate::responses::ToResponse;

#[derive(Debug, Serialize)]
pub struct JoinRoomSuccess {
    pub player_id: String,
}

#[derive(Debug, Serialize)]
pub struct CreateRoomSuccess {
    pub room_id: String,
    pub name: String,
}

impl ToResponse for JoinRoomSuccess {}
impl ToResponse for CreateRoomSuccess {}
