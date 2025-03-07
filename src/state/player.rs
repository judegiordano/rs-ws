use std::net::TcpStream;

use serde::Serialize;
use tokio_tungstenite::tungstenite::WebSocket;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Player {
    #[allow(dead_code)]
    #[serde(skip)]
    pub id: Uuid,
    // #[serde(skip)]
    // pub session: WebSocket<TcpStream>,
    pub display_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SanitizedPlayer {
    pub display_name: String,
}
