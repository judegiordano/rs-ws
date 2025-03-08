use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum RequestType {
    Ping = 0,
    // rooms
    CreateRoom = 1,
    JoinRoom = 2,
    ReadRoom = 3,
    LeaveRoom = 4,
    //
    Unhandled,
}

impl RequestType {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::Ping,
            1 => Self::CreateRoom,
            2 => Self::JoinRoom,
            3 => Self::ReadRoom,
            4 => Self::LeaveRoom,
            //
            _ => Self::Unhandled,
        }
    }
}
