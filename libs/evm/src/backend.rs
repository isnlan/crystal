use crate::{StorgeDoubelMap, Vicinity};
use anyhow::Result;
use ethereum::Account;
use ethereum_types::{H160, H256, U256};
use evm::backend::{Apply, ApplyBackend, Backend, Basic, Log};
use kvdb::KeyValueDB;
use log::error;
use std::sync::Arc;

/// Memory backend, storing all state values in a `BTreeMap` in memory.
#[derive(Clone, Debug)]
pub struct CrystalBackend<'vicinity, T> {
    vicinity: &'vicinity Vicinity,
    state: Arc<T>,
    logs: Vec<Log>,
}

impl<'vicinity, T: KeyValueDB> CrystalBackend<'vicinity, T> {
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

    fn remove(&self, key: &[u8]) {
        let mut t = self.state.transaction();
        t.delete(0, key);

        if let Err(err) = self.state.write(t) {
            error!("remove key error: {}", err);
        }
    }

    fn contains(&self, key: &[u8]) -> bool {
        self.state.has_key(0, key).unwrap_or(false)
    }

    fn get_account(&self, address: H160) -> Result<Option<Account>> {
        let v = self.state.get(0, address.as_bytes())?;
        match v {
            Some(data) => Ok(rlp::decode(&data)?),
            None => Ok(None),
        }
    }
}

impl<'vicinity, T: KeyValueDB> Backend for CrystalBackend<'vicinity, T> {
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
        self.contains(&address.as_bytes())
    }

    fn basic(&self, address: H160) -> Basic {
        self.get_account(address)
            .map(|v| match v {
                Some(acc) => Basic {
                    balance: acc.balance,
                    nonce: acc.nonce,
                },
                None => Basic::default(),
            })
            .unwrap_or_default()
    }

    fn code(&self, address: H160) -> Vec<u8> {
        let code_hash = match self.get_account(address) {
            Ok(acc) => match acc {
                Some(acc) => acc.code_hash,
                None => return vec![],
            },
            Err(_) => return vec![],
        };

        match self.state.get(0, code_hash.as_bytes()) {
            Ok(v) => match v {
                Some(code) => code,
                None => vec![],
            },
            Err(_) => vec![],
        }
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        let key = StorgeDoubelMap::storage_double_map_final_key(address, index);
        match self.state.get(0, key.as_ref()) {
            Ok(v) => match v {
                Some(data) => H256::from_slice(data.as_ref()),
                None => H256::zero(),
            },
            Err(_) => H256::zero(),
        }
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        Some(self.storage(address, index))
    }
}

impl<'vicinity, T: KeyValueDB> ApplyBackend for CrystalBackend<'vicinity, T> {
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
                        self.remove(&address.as_bytes())
                    }
                }
                Apply::Delete { address } => {
                    self.remove(&address.as_bytes());
                }
            }
        }

        for log in logs {
            self.logs.push(log);
        }
    }
}
