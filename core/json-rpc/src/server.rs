use std::sync::Arc;
use ethereum_types::*;
use jsonrpsee::core::async_trait;
use jsonrpsee::core::RpcResult as Result;
use rpc_core::{types::*, EthApiServer};
use txpool::TransactionPool;
use crate::internal_err;
use crate::signer::EthSigner;

pub struct Server<P> {
    signers: Vec<Box<dyn EthSigner>>,
    pool: Arc<P>,
}

impl <P>Server<P> {
    pub fn new(signers: Vec<Box<dyn EthSigner>>, pool: Arc<P>) -> Self {
        Server {
            signers,
            pool,
        }
    }

    pub fn accounts(&self) -> Result<Vec<H160>> {
        let accounts = Vec::new();
        Ok(accounts)
    }

    pub fn block_number(&self) -> Result<U256> {
        Ok(U256::zero())
    }

    pub fn chain_id(&self) -> Result<Option<U64>> {
        Ok(Some(U64::from(1)))
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

    fn gas_price(&self) -> Result<U256> {
        Ok(U256::zero())
    }
}

impl<P> Server<P>
    where
        P: TransactionPool+ Send + Sync + 'static,
{
    async fn send_transaction(&self, request: TransactionRequest) -> Result<H256> {
        let from = match request.from {
            Some(from) => from,
            None => {
                     return Err(internal_err("no signer available"))
                }
        };

        let nonce = match request.nonce {
            Some(nonce) => nonce,
            None => U256::zero(),
        };

        let chain_id = match self.chain_id() {
            Ok(Some(chain_id)) => chain_id.as_u64(),
            Ok(None) => return Err(internal_err("chain id not available")),
            Err(e) => return Err(e),
        };

        let gas_price = request.gas_price;
        let gas_limit = match request.gas {
            Some(gas_limit) => gas_limit,
            None => {
                    return Err(internal_err("block unavailable, cannot query gas limit"));
                }
        };
        let max_fee_per_gas = request.max_fee_per_gas;
        let message: Option<TransactionMessage> = request.into();
        let message = match message {
            Some(TransactionMessage::Legacy(mut m)) => {
                m.nonce = nonce;
                m.chain_id = Some(chain_id);
                m.gas_limit = gas_limit;
                if gas_price.is_none() {
                    m.gas_price = self.gas_price().unwrap_or_default();
                }
                TransactionMessage::Legacy(m)
            }
            Some(TransactionMessage::EIP2930(mut m)) => {
                m.nonce = nonce;
                m.chain_id = chain_id;
                m.gas_limit = gas_limit;
                if gas_price.is_none() {
                    m.gas_price = self.gas_price().unwrap_or_default();
                }
                TransactionMessage::EIP2930(m)
            }
            Some(TransactionMessage::EIP1559(mut m)) => {
                m.nonce = nonce;
                m.chain_id = chain_id;
                m.gas_limit = gas_limit;
                if max_fee_per_gas.is_none() {
                    m.max_fee_per_gas = self.gas_price().unwrap_or_default();
                }
                TransactionMessage::EIP1559(m)
            }
            _ => return Err(internal_err("invalid transaction parameters")),
        };

        let mut transaction = None;

        for signer in &self.signers {
            if signer.accounts().contains(&from) {
                match signer.sign(message, &from) {
                    Ok(t) => transaction = Some(t),
                    Err(e) => return Err(e),
                }
                break;
            }
        }

        let transaction = match transaction {
            Some(transaction) => transaction,
            None => return Err(internal_err("no signer available")),
        };
        let transaction_hash = transaction.hash();

        self.pool.submit_one(transaction)?;

        Ok(transaction_hash)
    }

    async fn send_raw_transaction(&self, _bytes: Bytes) -> Result<H256> {
        Ok(H256::default())
    }
}

#[async_trait]
impl <P>EthApiServer for Server<P>
where P: TransactionPool + Send + Sync + 'static
{
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
