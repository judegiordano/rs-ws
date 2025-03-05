use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum RequestType {
    Coordinates = 0,
    Ping = 1,
    // rooms
    JoinRoom = 2,
    CreateRoom = 3,
    ReadRoom = 4,
    //
    Unhandled,
}

impl RequestType {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::Coordinates,
            1 => Self::Ping,
            2 => Self::JoinRoom,
            3 => Self::CreateRoom,
            4 => Self::ReadRoom,
            //
            _ => Self::Unhandled,
        }
    }
}
