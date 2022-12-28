use ethereum_types::{H160, H256, U256, U64};
use jsonrpsee::{core::RpcResult as Result, proc_macros::rpc};

use crate::types::*;

/// Eth rpc interface.
#[rpc(server)]
#[async_trait]
pub trait EthApi {
    /// Returns accounts list.
    #[method(name = "eth_accounts")]
    fn accounts(&self) -> Result<Vec<H160>>;

    /// Returns highest block number.
    #[method(name = "eth_blockNumber")]
    fn block_number(&self) -> Result<U256>;

    /// Returns the chain ID used for transaction signing at the
    /// current best block. None is returned if not
    /// available.
    #[method(name = "eth_chainId")]
    fn chain_id(&self) -> Result<Option<U64>>;

    /// Returns block with given hash.
    #[method(name = "eth_getBlockByHash")]
    async fn block_by_hash(&self, hash: H256, full: bool) -> Result<Option<RichBlock>>;

    /// Returns block with given number.
    #[method(name = "eth_getBlockByNumber")]
    async fn block_by_number(&self, number: BlockNumber, full: bool) -> Result<Option<RichBlock>>;

    /// Get transaction by its hash.
    #[method(name = "eth_getTransactionByHash")]
    async fn transaction_by_hash(&self, hash: H256) -> Result<Option<Transaction>>;

    /// Returns transaction receipt by transaction hash.
    #[method(name = "eth_getTransactionReceipt")]
    async fn transaction_receipt(&self, hash: H256) -> Result<Option<Receipt>>;

    /// Returns balance of the given account.
    #[method(name = "eth_getBalance")]
    fn balance(&self, address: H160, number: Option<BlockNumber>) -> Result<U256>;

    /// Returns content of the storage at given address.
    #[method(name = "eth_getStorageAt")]
    fn storage_at(&self, address: H160, index: U256, number: Option<BlockNumber>) -> Result<H256>;

    /// Returns the number of transactions sent from given address at given time (block number).
    #[method(name = "eth_getTransactionCount")]
    fn transaction_count(&self, address: H160, number: Option<BlockNumber>) -> Result<U256>;

    /// Returns the code at given address at given time (block number).
    #[method(name = "eth_getCode")]
    fn code_at(&self, address: H160, number: Option<BlockNumber>) -> Result<Bytes>;

    /// Call contract, returning the output data.
    #[method(name = "eth_call")]
    fn call(&self, request: CallRequest, number: Option<BlockNumber>) -> Result<Bytes>;

    /// Sends transaction; will block waiting for signer to return the
    /// transaction hash.
    #[method(name = "eth_sendTransaction")]
    async fn send_transaction(&self, request: TransactionRequest) -> Result<H256>;

    /// Sends signed transaction, returning its hash.
    #[method(name = "eth_sendRawTransaction")]
    async fn send_raw_transaction(&self, bytes: Bytes) -> Result<H256>;
}
