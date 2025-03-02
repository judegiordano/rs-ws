use serde::{Deserialize, Serialize};

use crate::responses::{error_message::ErrorMessage, ResponseType, ToResponse};

use super::MessageHandler;

#[derive(Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Serialize)]
pub struct CoordinatesOk {
    pub ok: bool,
}

impl ToResponse<CoordinatesOk> for CoordinatesOk {
    fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        let signature = ResponseType::CoordinatesOk;
        let mut msg = vec![signature.as_byte()];
        let payload = serde_json::to_vec(&self)?;
        msg.extend(payload);
        Ok(msg)
    }
}

impl MessageHandler<Coordinates> for Coordinates {}

impl Coordinates {
    pub fn message_handler(data: &[u8]) -> Result<Vec<u8>, serde_json::Error> {
        let bytes = match Coordinates::parse_from_slice(data) {
            Ok(cords) => {
                tracing::debug!("[COORDINATES]: [{:?}]", cords);
                let response = CoordinatesOk { ok: true };
                response.as_bytes()
            }
            Err(err) => {
                tracing::error!("[COORDINATES ERROR]: [{:?}]", err);
                let response = ErrorMessage {
                    message: err.to_string(),
                };
                response.as_bytes()
            }
        };
        Ok(bytes?)
    }
}
