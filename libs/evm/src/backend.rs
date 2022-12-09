use std::collections::BTreeMap;
use std::sync::Arc;
use ethereum_types::{H160, H256, U256};
use evm::backend::{Apply, ApplyBackend, Backend, Basic, Log};
use db::DB;
use crate::Vicinity;

/// Memory backend, storing all state values in a `BTreeMap` in memory.
#[derive(Clone, Debug)]
pub struct CrystalBackend<'vicinity, T> {
    vicinity: &'vicinity Vicinity,
    state: Arc<T>,
    logs: Vec<Log>,
}

impl<'vicinity, T:DB> CrystalBackend<'vicinity, T> {
    /// Create a new memory backend.
    pub fn new(vicinity: &'vicinity Vicinity, state: Arc<T>) -> Self {
        Self {
            vicinity,
            state,
            logs: Vec::new(),
        }
    }

    /// Get the underlying `BTreeMap` storing the state.
    pub fn state(&self) -> &T {
        &self.state
    }
}

impl<'vicinity, T:DB> Backend for CrystalBackend<'vicinity, T> {
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }
    fn origin(&self) -> H160 {
        self.vicinity.origin
    }
    fn block_hash(&self, number: U256) -> H256 {
        if number >= self.vicinity.block_number
            || self.vicinity.block_number - number - U256::one()
            >= U256::from(self.vicinity.block_hashes.len())
        {
            H256::default()
        } else {
            let index = (self.vicinity.block_number - number - U256::one()).as_usize();
            self.vicinity.block_hashes[index]
        }
    }
    fn block_number(&self) -> U256 {
        self.vicinity.block_number
    }
    fn block_coinbase(&self) -> H160 {
        self.vicinity.block_coinbase
    }
    fn block_timestamp(&self) -> U256 {
        self.vicinity.block_timestamp
    }
    fn block_difficulty(&self) -> U256 {
        self.vicinity.block_difficulty
    }
    fn block_gas_limit(&self) -> U256 {
        self.vicinity.block_gas_limit
    }
    fn block_base_fee_per_gas(&self) -> U256 {
        self.vicinity.block_base_fee_per_gas
    }

    fn chain_id(&self) -> U256 {
        self.vicinity.chain_id
    }

    fn exists(&self, address: H160) -> bool {
        self.state.contains(&address.as_bytes()).unwrap()
    }

    fn basic(&self, address: H160) -> Basic {
        // self.state
        //     .get(&address)
        //     .map(|a| Basic {
        //         balance: a.balance,
        //         nonce: a.nonce,
        //     })
        //     .unwrap_or_default()
        todo!()
    }

    fn code(&self, address: H160) -> Vec<u8> {
        // self.state
        //     .get(&address)
        //     .map(|v| v.code.clone())
        //     .unwrap_or_default()
        todo!()
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        // self.state
        //     .get(&address)
        //     .map(|v| v.storage.get(&index).cloned().unwrap_or_default())
        //     .unwrap_or_default()
        todo!()
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        Some(self.storage(address, index))
    }
}

impl<'vicinity, T:DB> ApplyBackend for CrystalBackend<'vicinity, T> {
    fn apply<A, I, L>(&mut self, values: A, logs: L, delete_empty: bool)
        where
            A: IntoIterator<Item = Apply<I>>,
            I: IntoIterator<Item = (H256, H256)>,
            L: IntoIterator<Item = Log>,
    {
        for apply in values {
            match apply {
                Apply::Modify {
                    address,
                    basic,
                    code,
                    storage,
                    reset_storage,
                } => {
                    let is_empty = {
                        false
                        // let account = self.state.entry(address).or_insert_with(Default::default);
                        // account.balance = basic.balance;
                        // account.nonce = basic.nonce;
                        // if let Some(code) = code {
                        //     account.code = code;
                        // }
                        //
                        // if reset_storage {
                        //     account.storage = BTreeMap::new();
                        // }
                        //
                        // let zeros = account
                        //     .storage
                        //     .iter()
                        //     .filter(|(_, v)| v == &&H256::default())
                        //     .map(|(k, _)| *k)
                        //     .collect::<Vec<H256>>();
                        //
                        // for zero in zeros {
                        //     account.storage.remove(&zero);
                        // }
                        //
                        // for (index, value) in storage {
                        //     if value == H256::default() {
                        //         account.storage.remove(&index);
                        //     } else {
                        //         account.storage.insert(index, value);
                        //     }
                        // }
                        //
                        // account.balance == U256::zero()
                        //     && account.nonce == U256::zero()
                        //     && account.code.is_empty()
                    };

                    if is_empty && delete_empty {
                        self.state.remove(&address.as_bytes()).unwrap();
                    }
                }
                Apply::Delete { address } => {
                    self.state.remove(&address.as_bytes()).unwrap();
                }
            }
        }

        for log in logs {
            self.logs.push(log);
        }
    }
}
