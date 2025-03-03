use tokio_tungstenite::tungstenite::Bytes;

use crate::message_type::RequestType;

pub struct MessageBytes(pub Bytes);

impl MessageBytes {
    pub fn first_byte(&self) -> u8 {
        self.0[0]
    }

    pub fn message_type(&self) -> RequestType {
        RequestType::from_byte(self.first_byte())
    }

    pub fn message_body(&self) -> &[u8] {
        &self.0[1..]
    }
}
