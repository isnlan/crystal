use std::sync::Arc;
use jsonrpsee::RpcModule;
use txpool::TransactionPool;


pub fn new<P>(pool : Arc<P>, enable_dev_signer: bool) -> anyhow::Result<RpcModule<()>>
where
    P: TransactionPool + 'static,
{
    use json_rpc::{EthApiServer, EthSigner, EthDevSigner};

    let mut io = RpcModule::new(());

    let mut signers = Vec::new();
    if true {
        signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
    }

    // let pool = Arc::new(txpool::BasicPool::new());

    io.merge(json_rpc::Server::new(signers, pool).into_rpc())?;

    Ok(io)
}
