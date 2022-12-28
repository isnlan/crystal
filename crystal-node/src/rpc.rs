use jsonrpsee::RpcModule;

pub fn new() -> anyhow::Result<RpcModule<()>> {
    use json_rpc::EthApiServer;

    let mut io = RpcModule::new(());

    io.merge(json_rpc::Server::new().into_rpc())?;

    Ok(io)
}
