use std::{future::Future, sync::Arc};

use message::{Message, MessageBus};
use tokio::sync::mpsc::Receiver;

pub struct Server {
    tx: Receiver<Message>,
    bus: Arc<MessageBus>,
}

impl Server {
    pub fn new(tx: Receiver<Message>, bus: Arc<MessageBus>) -> Self {
        Self { tx, bus }
    }

    async fn run(&mut self) -> result::Result<()> {
        while let Some(msg) = self.tx.recv().await {
            self.process(msg).await?;
        }

        Ok(())
    }

    async fn process(&mut self, msg: Message) -> result::Result<()> {
        println!("msg {:?}", msg);

        Ok(())
    }
}
impl Future for Server {
    type Output = result::Result<()>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Box::pin(self.run()).as_mut().poll(cx)
    }
}
