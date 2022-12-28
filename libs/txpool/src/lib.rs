mod txpool;

use std::hash::Hash;
use ethereum::TransactionV2;
pub use txpool::BasicPool;

/// Transaction pool interface.
pub trait TransactionPool: Send + Sync {
    /// Returns a future that imports one unverified transaction to the pool.
    fn submit_one(&self, tx: TransactionV2) -> anyhow::Result<()>;

    /// Get an iterator for ready transactions ordered by priority.
    fn ready(&self) -> Vec<TransactionV2>;
}