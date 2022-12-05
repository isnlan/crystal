use hyper::Method;
use jsonrpsee::server::{AllowHosts, ServerBuilder};
use jsonrpsee::RpcModule;
use proto::{InnerMessage, Message, MessageBus};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tower_http::cors::{Any, CorsLayer};

pub struct Server {
    rx: Receiver<Message>,
    bus: Arc<MessageBus>,
}

impl Server {
    pub fn new(rx: Receiver<Message>, bus: Arc<MessageBus>) -> Self {
        Self { rx, bus }
    }

    pub async fn run(&mut self) -> result::Result<()> {
        let bus = self.bus.clone();
        tokio::task::spawn(async {
            run_json_rpc_server(bus).await.unwrap();
        });

        while let Some(msg) = self.rx.recv().await {
            match msg {
                Message::Close => {
                    break;
                }
                _ => {
                    self.process(msg).await?;
                }
            }
        }

        Ok(())
    }

    async fn process(&mut self, msg: Message) -> result::Result<()> {
        println!("jsonrpc-- msg {:?}", msg);
        self.bus.auth_sender.send(msg).await?;

        Ok(())
    }
}

async fn run_json_rpc_server(bus: Arc<MessageBus>) -> result::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    let middleware = tower::ServiceBuilder::new().layer(cors);

    let server = ServerBuilder::default()
        .set_host_filtering(AllowHosts::Any)
        .set_middleware(middleware)
        .build("0.0.0.0:7777".parse::<SocketAddr>()?)
        .await?;

    let mut module = RpcModule::new(bus);
    module.register_async_method("say_hello", |x, y| async move {
        println!("say_hello method called! {:?}", x.clone());
        let v = vec![1, 2, 2];

        y.clone()
            .jsonrpc_sender
            .send(Message::Inner(InnerMessage::new()))
            .await
            .unwrap();
        Ok(v)
    })?;

    let handle = server.start(module)?;

    handle.stopped().await;

    Ok(())
}
