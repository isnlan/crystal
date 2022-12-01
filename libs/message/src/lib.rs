use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum Message {
    Close,
    NetworkMessage(i32),
}

pub struct MessageBus {
    pub jsonrpc_sender: Sender<Message>,
    pub auth_sender: Sender<Message>,
}
