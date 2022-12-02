use message::MessageBus;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn init_log() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_file(true)
        .with_line_number(true)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (jsonrpc_sender, jsonrpc_reciver) = tokio::sync::mpsc::channel(1024);
    let (auth_sender, auth_reciver) = tokio::sync::mpsc::channel(1024);

    let bus = Arc::new(MessageBus {
        jsonrpc_sender,
        auth_sender,
    });

    let jsonrpc = json_rpc::Server::new(jsonrpc_reciver, bus.clone());
    let auth = auth::Server::new(auth_reciver, bus);

    let _ = tokio::join!(jsonrpc, auth);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_log();

    run().await
}
