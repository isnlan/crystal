use evm_core::ExitReason;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::backend::{ApplyBackend, CrystalBackend, CrystalStackState, Vicinity};
use crate::executor::stack::{PrecompileFn, StackExecutor, StackSubstateMetadata};
use anyhow::Result;
use codec::{Decode, Encode};
use ethereum::Log;
use ethereum_types::*;
use evm_runtime::{Config, CreateScheme};
use kvdb::KeyValueDB;

// #[derive(Clone, Eq, PartialEq, Encode, Decode)]
pub struct ExecutionInfo<T> {
    pub exit_reason: ExitReason,
    pub value: T,
    pub used_gas: U256,
    pub logs: Vec<Log>,
}

pub struct Executive<T> {
    db: Arc<T>,
    config: Config,
    precompile: BTreeMap<H160, PrecompileFn>,
    _marker: PhantomData<T>,
}

impl<T: KeyValueDB> Executive<T> {
    pub fn new(db: Arc<T>) -> Self {
        Self {
            db: db,
            config: Config::berlin(),
            precompile: Default::default(),
            _marker: PhantomData::default(),
        }
    }
}

impl<T: KeyValueDB> Executive<T> {
    pub fn call(
        &self,
        source: H160,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        _max_fee_per_gas: Option<U256>,
        _max_priority_fee_per_gas: Option<U256>,
        _nonce: Option<U256>,
        _access_list: Vec<(H160, Vec<H256>)>,
        _is_transactional: bool,
        validate: bool,
        vicinity: Vicinity,
    ) -> Result<ExecutionInfo<Vec<u8>>> {
        if validate {
            // todo validate args
        }

        let mut backend = CrystalBackend::new(&vicinity, self.db.clone());
        let metadata = StackSubstateMetadata::new(&self.config);
        let state = CrystalStackState::new(metadata, &backend);
        let mut executor =
            StackExecutor::new_with_precompiles(state, &self.config, &self.precompile);
        let (reason, retv) = executor.transact_call(source, target, value, input, vec![]);
        // let gas = executor.gas();
        // let used_gas = U256::from(executor.used_gas());
        // let actual_fee = executor.fee(tot)

        let (value, logs) = executor.into_state().deconstruct();
        backend.apply(value, vec![], false);

        Ok(ExecutionInfo {
            exit_reason: reason,
            value: retv,
            used_gas: U256::zero(),
            logs: Vec::from_iter(logs), //logs, //state.substate.logs(),
        })
    }

    pub fn create(
        &self,
        source: H160,
        init: Vec<u8>,
        value: U256,
        gas_limit: u64,
        _max_fee_per_gas: Option<U256>,
        _max_priority_fee_per_gas: Option<U256>,
        _nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
        _is_transactional: bool,
        _validate: bool,
        vicinity: Vicinity,
    ) -> Result<ExecutionInfo<H160>> {
        let mut backend = CrystalBackend::new(&vicinity, self.db.clone());
        let metadata = StackSubstateMetadata::new(&self.config);
        let state = CrystalStackState::new(metadata, &backend);
        let mut executor =
            StackExecutor::new_with_precompiles(state, &self.config, &self.precompile);
        let address = executor.create_address(CreateScheme::Legacy { caller: source });

        let (reason, _) = executor.transact_create(source, value, init, access_list);
        // let used_gas = U256::from(executor.used_gas());

        let (value, logs) = executor.into_state().deconstruct();
        backend.apply(value, vec![], false);

        Ok(ExecutionInfo {
            exit_reason: reason,
            value: address,
            used_gas: U256::zero(),
            logs: Vec::from_iter(logs), //logs, //state.substate.logs(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn it_works() {
        // let result = call(2, 2);
        // assert_eq!(result, 4);
        let kvdb = Arc::new(kvdb_memorydb::create(0));
        let vicinity = Vicinity {
            gas_price: Default::default(),
            origin: Default::default(),
            chain_id: Default::default(),
            block_hashes: vec![],
            block_number: Default::default(),
            block_coinbase: Default::default(),
            block_timestamp: Default::default(),
            block_difficulty: Default::default(),
            block_gas_limit: Default::default(),
            block_base_fee_per_gas: Default::default(),
        };
        let exec = Executive::new(kvdb.clone());

        let code = "6080604052600436106049576000357c0100000000000000000000000000000\
                000000000000000000000000000900463ffffffff16806360fe47b114604e57\
                80636d4ce63c146078575b600080fd5b348015605957600080fd5b506076600\
                4803603810190808035906020019092919050505060a0565b005b3480156083\
                57600080fd5b50608a60aa565b6040518082815260200191505060405180910\
                390f35b8060008190555050565b600080549050905600a165627a7a72305820\
                99c66a25d59f0aa78f7ebc40748fa1d1fbc335d8d780f284841b30e0365acd9\
                60029";
        let code = hex::decode(code).unwrap();
        let source = H160::from_str("0x0000000000000000000000000000000000000001").unwrap();

        let rev = exec
            .create(
                source,
                code,
                U256::from(1000),
                0,
                None,
                None,
                None,
                vec![],
                true,
                false,
                vicinity.clone(),
            )
            .unwrap();
        println!(
            "code address: {:?}, reason: {:?}",
            rev.value, rev.exit_reason
        );
        println!("=============================");

        let exec2 = Executive::new(kvdb);
        let data =
            hex::decode("60fe47b1000000000000000000000000000000000000000000000000000000000000002a")
                .unwrap();
        let rev2 = exec2
            .call(
                source,
                rev.value,
                data,
                U256::zero(),
                0,
                None,
                None,
                None,
                vec![],
                true,
                false,
                vicinity,
            )
            .unwrap();
        println!("{:?}: {:?}", rev2.exit_reason, rev2.value);
    }
}
