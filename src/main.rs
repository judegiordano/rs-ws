use commands::{
    coordinates::Coordinates,
    ping::Ping,
    room::{create_room::CreateRoom, join_room::JoinRoom, read_room::ReadRoom},
    MessageHandler,
};
use futures_util::{SinkExt, StreamExt};
use message_bytes::MessageBytes;
use message_type::RequestType;
use responses::{error_message::ErrorMessage, Response};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod auth;
mod commands;
mod message_bytes;
mod message_type;
mod responses;
mod state;

async fn message_handler(msg: Message) -> Result<Message> {
    let msg_bytes = MessageBytes(msg.into_data());
    let msg_type = msg_bytes.message_type();
    let data = msg_bytes.message_body();
    let handler = match msg_type {
        RequestType::Coordinates => Coordinates::response_handler(data).await,
        RequestType::Ping => Ping::response_handler(data).await,
        // rooms
        RequestType::JoinRoom => JoinRoom::response_handler(data).await,
        RequestType::CreateRoom => CreateRoom::response_handler(data).await,
        RequestType::ReadRoom => ReadRoom::response_handler(data).await,
        // unhandled
        _ => ErrorMessage::response_handler(data).await,
    };
    // TODO: handle
    let buffer = match handler {
        Ok(response) => response.build_response(),
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
    // let response = handler.unwrap().build_response();
    Ok(Message::binary(buffer))
}

async fn handle_connection(_: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_binary() {
            let resp = message_handler(msg).await?;
            ws_stream.send(resp).await?;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
