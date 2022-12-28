mod rpc;
mod service;

use proto::{Message, MessageBus};
use signal_hook::{
    consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM},
    iterator::Signals,
};
use std::sync::Arc;
use tracing::log::info;
use tracing_subscriber::FmtSubscriber;

fn init_log() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (jsonrpc_sender, jsonrpc_reciver) = tokio::sync::mpsc::channel(1024);
    let (auth_sender, auth_reciver) = tokio::sync::mpsc::channel(1024);
    let (chain_sender, chain_reciver) = tokio::sync::mpsc::channel(1024);

    let bus = Arc::new(MessageBus {
        jsonrpc_sender,
        auth_sender,
        chain_sender,
    });

    service::run()?;

    let bus_clone = bus.clone();
    tokio::spawn(async move {
        let mut ath = auth::Server::new(auth_reciver, bus_clone);
        ath.run().await.unwrap();
    });

    let bus_clone = bus.clone();
    tokio::spawn(async move {
        let mut chain = chain::Server::new(chain_reciver, bus_clone);
        chain.run().await.unwrap();
    });

    const SIGNALS: &[std::ffi::c_int] = &[SIGHUP, SIGTERM, SIGQUIT, SIGINT];
    let mut sigs = Signals::new(SIGNALS)?;
    for signal in &mut sigs {
        info!("Received signal {:?}", signal);
        match signal {
            SIGTERM | SIGQUIT | SIGINT => {
                bus.auth_sender.send(Message::Close).await?;
                bus.jsonrpc_sender.send(Message::Close).await?;
                bus.chain_sender.send(Message::Close).await?;
                break;
            }
            SIGHUP => {
                info!("rotator log");
            }
            _ => {
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_log();

    run().await
}
