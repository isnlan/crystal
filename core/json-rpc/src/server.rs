use ethereum_types::*;
use jsonrpsee::core::RpcResult as Result;
use jsonrpsee::{core::async_trait, server::ServerBuilder};
use rpc_core::{types::*, EthApiServer};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn accounts(&self) -> Result<Vec<H160>> {
        let accounts = Vec::new();
        Ok(accounts)
    }

    pub fn block_number(&self) -> Result<U256> {
        Ok(U256::zero())
    }

    pub fn chain_id(&self) -> Result<Option<U64>> {
        Ok(Some(U64::zero()))
    }

    async fn block_by_hash(&self, _hash: H256, _full: bool) -> Result<Option<RichBlock>> {
        Ok(None)
    }

    async fn block_by_number(
        &self,
        _number: BlockNumber,
        _full: bool,
    ) -> Result<Option<RichBlock>> {
        Ok(None)
    }

    async fn transaction_by_hash(&self, _hash: H256) -> Result<Option<Transaction>> {
        Ok(None)
    }

    async fn transaction_receipt(&self, _hash: H256) -> Result<Option<Receipt>> {
        Ok(None)
    }

    fn balance(&self, _address: H160, _number: Option<BlockNumber>) -> Result<U256> {
        Ok(U256::zero())
    }

    fn storage_at(
        &self,
        _address: H160,
        _index: U256,
        _number: Option<BlockNumber>,
    ) -> Result<H256> {
        Ok(H256::default())
    }

    fn transaction_count(&self, _address: H160, _number: Option<BlockNumber>) -> Result<U256> {
        Ok(U256::zero())
    }

    fn code_at(&self, _address: H160, _number: Option<BlockNumber>) -> Result<Bytes> {
        Ok(Bytes::default())
    }

    fn call(&self, _request: CallRequest, _number: Option<BlockNumber>) -> Result<Bytes> {
        Ok(Bytes::default())
    }

    async fn send_transaction(&self, _request: TransactionRequest) -> Result<H256> {
        println!("{:?}", _request);
        Ok(H256::default())
    }

    async fn send_raw_transaction(&self, _bytes: Bytes) -> Result<H256> {
        Ok(H256::default())
    }
}

#[async_trait]
impl EthApiServer for Server {
    fn accounts(&self) -> Result<Vec<H160>> {
        self.accounts()
    }

    fn block_number(&self) -> Result<U256> {
        self.block_number()
    }

    fn chain_id(&self) -> Result<Option<U64>> {
        self.chain_id()
    }

    async fn block_by_hash(&self, hash: H256, full: bool) -> Result<Option<RichBlock>> {
        self.block_by_hash(hash, full).await
    }

    async fn block_by_number(&self, number: BlockNumber, full: bool) -> Result<Option<RichBlock>> {
        self.block_by_number(number, full).await
    }

    async fn transaction_by_hash(&self, hash: H256) -> Result<Option<Transaction>> {
        self.transaction_by_hash(hash).await
    }

    async fn transaction_receipt(&self, hash: H256) -> Result<Option<Receipt>> {
        self.transaction_receipt(hash).await
    }

    fn balance(&self, address: H160, number: Option<BlockNumber>) -> Result<U256> {
        self.balance(address, number)
    }

    fn storage_at(&self, address: H160, index: U256, number: Option<BlockNumber>) -> Result<H256> {
        self.storage_at(address, index, number)
    }

    fn transaction_count(&self, address: H160, number: Option<BlockNumber>) -> Result<U256> {
        self.transaction_count(address, number)
    }

    fn code_at(&self, address: H160, number: Option<BlockNumber>) -> Result<Bytes> {
        self.code_at(address, number)
    }

    fn call(&self, request: CallRequest, number: Option<BlockNumber>) -> Result<Bytes> {
        self.call(request, number)
    }

    async fn send_transaction(&self, request: TransactionRequest) -> Result<H256> {
        self.send_transaction(request).await
    }

    async fn send_raw_transaction(&self, bytes: Bytes) -> Result<H256> {
        self.send_raw_transaction(bytes).await
    }
}

