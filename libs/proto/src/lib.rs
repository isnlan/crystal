pub mod protos;
pub use crate::protos::*;

use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum Message {
    Inner(InnerMessage),
    Close,
}

pub struct MessageBus {
    pub jsonrpc_sender: Sender<Message>,
    pub auth_sender: Sender<Message>,
    pub chain_sender: Sender<Message>,
}
