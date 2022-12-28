use std::sync::Arc;
use jsonrpsee::RpcModule;


pub fn new() -> anyhow::Result<RpcModule<()>> {
    use json_rpc::{EthApiServer, EthSigner, EthDevSigner};

    let mut io = RpcModule::new(());

    let mut signers = Vec::new();
    if true {
        signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
    }

    let pool = Arc::new(txpool::BasicPool::new());

    io.merge(json_rpc::Server::new(signers, pool).into_rpc())?;

    Ok(io)
}
