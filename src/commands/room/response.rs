use serde::Serialize;

use crate::responses::ToResponse;

#[derive(Debug, Serialize)]
pub struct JoinRoomSuccess {
    pub ok: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateRoomSuccess {
    pub id: String,
    pub name: String,
}

impl ToResponse for JoinRoomSuccess {}
impl ToResponse for CreateRoomSuccess {}
