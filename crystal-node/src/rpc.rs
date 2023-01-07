use jsonrpsee::RpcModule;
use std::sync::Arc;
use txpool::TransactionPool;

pub fn new<P>(pool: Arc<P>, enable_dev_signer: bool) -> anyhow::Result<RpcModule<()>>
where
    P: TransactionPool + 'static,
{
    use json_rpc::{EthApiServer, EthDevSigner, EthSigner};

    let mut io = RpcModule::new(());

    let mut signers = Vec::new();
    if enable_dev_signer {
        signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
    }

    // let pool = Arc::new(txpool::BasicPool::new());

    io.merge(json_rpc::Server::new(signers, pool).into_rpc())?;

    Ok(io)
}
