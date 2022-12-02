use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::body::Buf;
use hyper::Method;
use jsonrpsee::RpcModule;
use jsonrpsee::server::{AllowHosts, ServerBuilder};
use tower_http::cors::{Any, CorsLayer};
use message::{Message, MessageBus};

pub async fn run_server(bus: Arc<MessageBus>) -> result::Result<()>{
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
	module.register_async_method("say_hello",  |x, y| async move {
		println!("say_hello method called! {:?}", x.clone());
		let v = vec![1,2,2];

		y.clone().auth_sender.send(Message::NetworkMessage(12)).await.unwrap();
		Ok(v)
	})?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	handle.stopped().await;
	// tokio::spawn(handle.stopped());

	Ok(())
}