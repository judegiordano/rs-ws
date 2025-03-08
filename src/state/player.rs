use std::sync::Arc;

use serde::Serialize;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

pub type WebSocket =
    futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Player {
    #[serde(skip)]
    pub id: Uuid,
    #[serde(skip)]
    pub session: Arc<Mutex<WebSocket>>,
    pub display_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SanitizedPlayer {
    pub display_name: String,
}
