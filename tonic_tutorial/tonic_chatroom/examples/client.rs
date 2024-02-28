use std::env;
use tokio::io::{self, AsyncBufReadExt};
use tonic_chatroom::client::Client;

// USERNAME=ztw RUST_LOG=info cargo run --example client
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let username = env::var("USERNAME")?;
    let mut client = Client::new(username).await;
    client.login().await?;
    client.get_messages().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Ok(Some(line)) = stdin.next_line().await {
        client.send_message("lobby", line).await?;
    }
    Ok(())
}