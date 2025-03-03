use commands::{coordinates::Coordinates, ping::Ping, MessageHandler};
use futures_util::{SinkExt, StreamExt};
use message_bytes::MessageBytes;
use message_type::MessageType;
use responses::error_message::ErrorMessage;
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod commands;
mod message_bytes;
mod message_type;
mod responses;

async fn message_handler(msg: Message) -> Result<Message> {
    let msg_bytes = MessageBytes(msg.into_data());
    let msg_type = msg_bytes.message_type();
    let data = msg_bytes.message_body();
    // handle each type of message type
    tokio::time::sleep(Duration::from_millis(0)).await;
    let response = match msg_type {
        MessageType::Coordinates => Coordinates::response_handler(data),
        MessageType::Ping => Ping::response_handler(data),
        _ => ErrorMessage::response_handler(data),
    };
    // TODO: handle
    let data = response.unwrap().as_bytes().unwrap();
    Ok(Message::binary(data))
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
