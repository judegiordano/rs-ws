use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum MessageType {
    Coordinates = 0,
    Unhandled,
}

impl MessageType {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::Coordinates,
            _ => Self::Unhandled,
        }
    }
}
