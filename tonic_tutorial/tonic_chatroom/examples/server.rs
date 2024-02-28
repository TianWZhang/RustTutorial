use tonic_chatroom::server;

// RUST_LOG=info cargo run --example server
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    server::start().await;
    Ok(())
}