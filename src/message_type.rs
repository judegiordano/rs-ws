use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum RequestType {
    Coordinates = 0,
    Ping,
    Unhandled,
}

impl RequestType {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::Coordinates,
            1 => Self::Ping,
            _ => Self::Unhandled,
        }
    }
}
