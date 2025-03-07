mod auth;
mod commands;
mod message_bytes;
mod message_type;
mod responses;
mod server;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::start().await
}
