use futures_util::{SinkExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{
    commands::{
        health::ping::Ping,
        room::{
            create_room::CreateRoom, join_room::JoinRoom, leave_room::LeaveRoom,
            read_room::ReadRoom,
        },
        MessageHandler,
    },
    message_bytes::MessageBytes,
    message_type::RequestType,
    responses::{error_message::ErrorMessage, Response},
    state::player::WebSocket,
};

async fn message_handler(
    msg: Message,
    responder: Arc<Mutex<WebSocket>>,
    receiver: Arc<Mutex<WebSocket>>,
) -> Result<()> {
    let msg_bytes = MessageBytes(msg.into_data());
    let msg_type = msg_bytes.message_type();
    let data = msg_bytes.message_body();
    let handler = match msg_type {
        RequestType::Ping => Ping::response_handler(data, receiver).await,
        // rooms
        RequestType::JoinRoom => JoinRoom::response_handler(data, receiver).await,
        RequestType::CreateRoom => CreateRoom::response_handler(data, receiver).await,
        RequestType::ReadRoom => ReadRoom::response_handler(data, receiver).await,
        RequestType::LeaveRoom => LeaveRoom::response_handler(data, receiver).await,
        // unhandled
        _ => ErrorMessage::response_handler(data, receiver).await,
    };
    // TODO: handle
    let buffer = match handler {
        Ok(response) => {
            tracing::info!("[RESPONSE]: {:?}", response);
            response.build_response()
        }
        Err(err) => {
            // TODO: refactor
            tracing::error!("[{:?}]: [{:?}]", msg_type, data);
            tracing::error!("[ERROR BUILDING RESPONSE] {:?}", err);
            let err = ErrorMessage {
                message: err.to_string(),
            };
            Response::Error(err).build_response()
        }
    };
    let msg = Message::binary(buffer);
    responder.lock().await.send(msg).await?;
    Ok(())
}

async fn handle_connection(_: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let (sender, mut receiver) = ws_stream.split();
    let sender = Arc::new(Mutex::new(sender));
    while let Some(msg) = receiver.next().await {
        let msg = msg?;
        if msg.is_binary() {
            message_handler(msg, sender.clone(), sender.clone()).await?;
        }
    }
    Ok(())
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::Protocol(_) | Error::Utf8 => (),
            Error::ConnectionClosed => {
                tracing::debug!("CLIENT CLOSED")
            }
            err => tracing::error!("[CLIENT ERROR]: {}", err),
        }
    }
}

pub async fn start() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .finish(),
    )?;
    let addr = "0.0.0.0:80";
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("listening on: {:?}", listener.local_addr()?);
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr()?;
        tokio::spawn(accept_connection(peer, stream));
    }
    Ok(())
}
