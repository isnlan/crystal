use std::sync::Arc;
use crate::rpc;

use jsonrpsee::RpcModule;

pub fn run() -> anyhow::Result<()> {
    let pool = Arc::new(txpool::BasicPool::new());

    let module = rpc::new(pool, true)?;

    tokio::task::spawn(async {
        run_json_rpc_server(module).await.unwrap();
    });

    Ok(())
}

async fn run_json_rpc_server(module: RpcModule<()>) -> anyhow::Result<()> {
    use hyper::Method;
    use jsonrpsee::server::{AllowHosts, ServerBuilder};
    use std::net::SocketAddr;
    use tower_http::cors::{Any, CorsLayer};

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

    let handle = server.start(module)?;

    handle.stopped().await;

    Ok(())
}
