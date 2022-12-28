use crate::key_mapping::StorageDoubleMap;
use crate::Vicinity;
use anyhow::Result;
use ethereum::Account;
use ethereum_types::{H160, H256, U256};
use evm::backend::{Apply, ApplyBackend, Backend, Basic, Log};
use kvdb::KeyValueDB;
use std::sync::Arc;
use tracing::error;

const STORAGE_CODE_KEY: u8 = 1;
const STORAGE_SLOT_KEY: u8 = 2;

#[derive(Clone, Debug)]
pub struct CrystalBackend<'vicinity, T> {
    vicinity: &'vicinity Vicinity,
    state: Arc<T>,
}

impl<'vicinity, T: KeyValueDB> CrystalBackend<'vicinity, T> {
    /// Create a new memory backend.
    pub fn new(vicinity: &'vicinity Vicinity, state: Arc<T>) -> Self {
        Self { vicinity, state }
    }

    fn address_key(address: H160) -> StorageDoubleMap {
        StorageDoubleMap {
            module: "evm".as_bytes().to_vec(),
            storage: address.as_bytes().to_vec(),
        }
    }

    fn gen_code_key(address: H160, hash: H256) -> Vec<u8> {
        Self::address_key(address).storage_double_map_final_key(STORAGE_CODE_KEY, hash)
    }

    fn gen_code_key_perfix(address: H160) -> Vec<u8> {
        Self::address_key(address).storage_double_map_key_prefix(STORAGE_CODE_KEY)
    }

    fn gen_slot_key(address: H160, index: H256) -> Vec<u8> {
        Self::address_key(address).storage_double_map_final_key(STORAGE_SLOT_KEY, index)
    }

    // evm:address+slot+index ->  value
    fn gen_slot_key_perfix(address: H160) -> Vec<u8> {
        Self::address_key(address).storage_double_map_key_prefix(STORAGE_SLOT_KEY)
    }

    // evm:address -> account
    fn gen_accout_key(address: H160) -> Vec<u8> {
        Self::address_key(address).storage_double_map_prefix()
    }
    /// Get the underlying `BTreeMap` storing the state.
    pub fn state(&self) -> &T {
        &self.state
    }

    fn contains(&self, key: &[u8]) -> bool {
        self.state.has_key(0, key).unwrap_or(false)
    }

    fn get_account(&self, address: H160) -> Result<Option<Account>> {
        let v = self.state.get(0, &Self::gen_accout_key(address))?;
        match v {
            Some(data) => Ok(rlp::decode(&data)?),
            None => Ok(None),
        }
    }

    fn apply<A, I, L>(&mut self, values: A, _logs: L, delete_empty: bool) -> Result<()>
    where
        A: IntoIterator<Item = Apply<I>>,
        I: IntoIterator<Item = (H256, H256)>,
        L: IntoIterator<Item = Log>,
    {
        let mut tx = self.state.transaction();
        for apply in values {
            match apply {
                Apply::Modify {
                    address,
                    basic,
                    code,
                    storage,
                    reset_storage,
                } => {
                    let mut find = false;
                    let mut tx_sub = self.state.transaction();

                    let mut account = match self.get_account(address)? {
                        Some(account) => {
                            find = true;
                            account
                        }
                        None => Account {
                            nonce: U256::zero(),
                            balance: U256::zero(),
                            storage_root: H256::zero(),
                            code_hash: H256::zero(),
                        },
                    };

                    account.balance = basic.balance;
                    account.nonce = basic.nonce;
                    let has_code = code.is_some();

                    if let Some(code) = code {
                        let hash = keccak_hash::keccak(&code);
                        account.code_hash = hash;

                        tx_sub.put(0, Self::gen_code_key(address, hash).as_ref(), &code)
                    }

                    if reset_storage {
                        tx_sub.delete_prefix(0, &Self::gen_slot_key_perfix(address))
                    }

                    let storage_prefix = Self::gen_slot_key_perfix(address);
                    let iter = self.state.iter_with_prefix(0, &storage_prefix);
                    for item in iter {
                        let (key, value) = item?;
                        if H256::from_slice(&value) == H256::default() {
                            tx_sub.delete(0, &key)
                        }
                    }

                    for (index, value) in storage {
                        if value == H256::default() {
                            // account.storage.remove(&index);
                            tx_sub.delete(0, &Self::gen_slot_key(address, index));
                        } else {
                            // account.storage.insert(index, value);
                            tx_sub.put(0, &Self::gen_slot_key(address, index), value.as_bytes())
                        }
                    }

                    if account.balance == U256::zero()
                        && account.nonce == U256::zero()
                        && delete_empty
                        && (has_code
                            || self.state.has_key(0, &Self::gen_code_key_perfix(address))?)
                    {
                        tx_sub.ops.clear();

                        tx.delete_prefix(0, &Self::gen_accout_key(address))
                    } else {
                        tx_sub.put(0, &Self::gen_accout_key(address), &rlp::encode(&account));

                        tx.ops.append(&mut tx_sub.ops);
                    }
                }
                Apply::Delete { address } => tx.delete_prefix(0, &Self::gen_accout_key(address)),
            }
        }

        self.state.write(tx)?;

        Ok(())
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
        let key = Self::gen_slot_key(address, index);

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
        if let Err(err) = self.apply(values, logs, delete_empty) {
            error!("commit status into db error: {}", err);
        }
    }
}
