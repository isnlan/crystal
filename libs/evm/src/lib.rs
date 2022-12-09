#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use codec::{Decode, Encode};
use ethereum_types::{Address, H160, H256, U256};
use evm::backend::{ApplyBackend, MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, PrecompileFn, StackExecutor, StackSubstateMetadata};
pub use evm::{
    backend::{Basic as Account, Log},
    Config, ExitReason,
};
use evm::{executor, Context};
use db::{DB, MemoryDB};
use crate::stack::CrystalStackState;
use anyhow::Result;
use crate::backend::CrystalBackend;

#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode)]
/// External input from the transaction.
pub struct Vicinity {
    /// Gas price.
    pub gas_price: U256,
    /// Origin.
    pub origin: H160,
    /// Chain ID.
    pub chain_id: U256,
    /// Environmental block hashes.
    pub block_hashes: Vec<H256>,
    /// Environmental block number.
    pub block_number: U256,
    /// Environmental coinbase.
    pub block_coinbase: H160,
    /// Environmental block timestamp.
    pub block_timestamp: U256,
    /// Environmental block difficulty.
    pub block_difficulty: U256,
    /// Environmental block gas limit.
    pub block_gas_limit: U256,
    /// Environmental base fee per gas.
    pub block_base_fee_per_gas: U256,
}

mod stack;
mod backend;

    #[derive(Clone, Debug)]
    pub struct Transaction {
        pub from: Address,
        pub to: Option<Address>, // Some for call and None for create.
        pub value: U256,
        pub nonce: U256,
        pub gas_limit: u64,
        pub gas_price: U256,
        pub input: Vec<u8>,
    }

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
pub struct ExecutionInfo<T> {
    pub exit_reason: ExitReason,
    pub value: T,
    pub used_gas: U256,
    pub logs: Vec<Log>,
}

pub struct Executive<T: DB> {
    db: Arc<T>,
    config: Config,
    precompile: BTreeMap<H160, PrecompileFn>,
    _marker: PhantomData<T>
}

impl <T: DB>Executive<T> {
    pub fn call(&self, tx: Transaction,source: H160, target: H160, gas_limit:u64, value: U256, input: Vec<u8>) {
        let config = Config::berlin();
        let vicinity = Vicinity {
            gas_price: U256::zero(),
            origin: H160::default(),
            block_hashes: Vec::new(),
            block_number: Default::default(),
            block_coinbase: Default::default(),
            block_timestamp: Default::default(),
            block_difficulty: Default::default(),
            block_gas_limit: Default::default(),
            chain_id: U256::one(),
            block_base_fee_per_gas: U256::zero(),
        };

        let backend = CrystalBackend::new(&vicinity, self.db.clone());
        let metadata = StackSubstateMetadata::new(gas_limit, &config);
        let state = CrystalStackState::new(metadata, &backend);
        let mut executor = StackExecutor::new_with_precompiles(state, &config, &self.precompile);
        let (reason ,ret) = executor.transact_call(source, target, value, input, gas_limit, vec![] );
        // let gas = executor.gas();
        let used_gas = U256::from(executor.used_gas());
        // let actual_fee = executor.fee(tot)

        let state = executor.into_state();
        // far address


    }

    // fn execute<'precompiles,F, R>(caller: H160, value: U256, gas_limit: u64, f: F) -> Result<ExecutionInfo<R>>
    // where F: FnOnce(&mut StackExecutor<'precompiles, CrystalStackState<'_, '_, T>, T>)
    // {
    //     todo!()
    // }
}


pub fn call(left: usize, right: usize) -> usize {
    // Execute the EVM call.
    // let vicinity = Vicinity {
    //     gas_price: base_fee,
    //     origin: source,
    // };

    // let e = Executive(MemoryDB::new(true));

    // let metadata = StackSubstateMetadata::new(gas_limit, config);
    // let state = SubstrateStackState::new(&vicinity, metadata);
    // let mut executor = StackExecutor::new_with_precompiles(state, config, precompiles);

    let config = Config::berlin();
    let gas_limit = 1000000000;

    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
    };

    let code = "6080604052600436106049576000357c0100000000000000000000000000000\
                000000000000000000000000000900463ffffffff16806360fe47b114604e57\
                80636d4ce63c146078575b600080fd5b348015605957600080fd5b506076600\
                4803603810190808035906020019092919050505060a0565b005b3480156083\
                57600080fd5b50608a60aa565b6040518082815260200191505060405180910\
                390f35b8060008190555050565b600080549050905600a165627a7a72305820\
                99c66a25d59f0aa78f7ebc40748fa1d1fbc335d8d780f284841b30e0365acd9\
                60029";
    let contract_address = H160::from_str("0xBd770416a3345F91E4B34576cb804a576fa48EB1").unwrap();
    let caller = H160::from_str("0x0000000000000000000000000000000000000001").unwrap();

    let mut state = BTreeMap::new();
    state.insert(
        contract_address,
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: hex::decode(code).unwrap(),
        },
    );
    state.insert(
        caller,
        MemoryAccount {
            nonce: U256::one(),
            balance: U256::from(10000000),
            storage: BTreeMap::new(),
            code: Vec::new(),
        },
    );

    let mut backend = MemoryBackend::new(&vicinity, state);
    let metadata = StackSubstateMetadata::new(gas_limit, &config);
    let mstate = CrystalStackState::new(metadata, &backend);
    let precompile = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(mstate, &config, &precompile);

    let code = Rc::new(hex::decode(code).unwrap());
    // set value
    let data =
        hex::decode("60fe47b1000000000000000000000000000000000000000000000000000000000000002a")
            .unwrap();
    let context = Context {
        address: contract_address,
        caller: caller,
        apparent_value: Default::default(),
    };
    let mut runtime = evm::Runtime::new(code.clone(), Rc::new(data), context, &config);
    let reason = executor.execute(&mut runtime);

    let gas = executor.gas();
    let (values, logs) = executor.into_state().deconstruct();
    backend.apply(values, logs, false);

    {
        let data = hex::decode("6d4ce63c").unwrap();
        // let context = Context {
        //     address: contract_address,
        //     caller: caller,
        //     apparent_value: Default::default(),
        // };
        // let mut runtime = evm::Runtime::new(code, Rc::new(data), context, &config);

        let metadata = StackSubstateMetadata::new(gas_limit, &config);
        let mstate = MemoryStackState::new(metadata, &backend);
        let mut executor = StackExecutor::new_with_precompiles(mstate, &config, &precompile);
        let (reasson ,ret) = executor.transact_call(caller, contract_address, Default::default(), data, gas_limit, vec![] );
        // let reason = executor.execute(&mut runtime);
        let gas = executor.gas();
        let state = executor.into_state();
        // state.substate.

        // backend.apply(values, logs, false);
        // let ret = runtime.machine().return_value();

        println!("{:?}", ret);
    }
    return left + right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = call(2, 2);
        assert_eq!(result, 4);
    }
}
