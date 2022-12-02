use std::{future::Future, sync::Arc};

use crate::jsonrpc::run_server;
use message::{Message, MessageBus};
use tokio::sync::mpsc::Receiver;

mod jsonrpc;

pub struct Server {
    rx: Receiver<Message>,
    bus: Arc<MessageBus>,
}

impl Server {
    pub fn new(rx: Receiver<Message>, bus: Arc<MessageBus>) -> Self {
        Self { rx, bus }
    }

    async fn run(&mut self) -> result::Result<()> {
        let bus = self.bus.clone();
        run_server(bus).await?;

        while let Some(msg) = self.rx.recv().await {
            println!("======");
            self.process(msg).await?;
        }

        Ok(())
    }

    async fn process(&mut self, msg: Message) -> result::Result<()> {
        println!("jsonrpc-- msg {:?}", msg);

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
