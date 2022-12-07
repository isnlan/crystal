#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::str::FromStr;

use codec::{Decode, Encode};
use evm::backend::{ApplyBackend, MemoryAccount, MemoryBackend, MemoryVicinity};
use evm::executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata};
pub use evm::{
    backend::{Basic as Account, Log},
    Config, ExitReason,
};
use evm::{executor, Context};
use primitive_types::{H160, U256};

// #[derive(Clone, Eq, PartialEq, Default, Encode, Decode)]
// #[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
// /// External input from the transaction.
// pub struct Vicinity {
//     /// Current transaction gas price.
//     pub gas_price: U256,
//     /// Origin of the transaction.
//     pub origin: H160,
// }

pub fn call(left: usize, right: usize) -> usize {
    // Execute the EVM call.
    // let vicinity = Vicinity {
    //     gas_price: base_fee,
    //     origin: source,
    // };

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
    let mstate = MemoryStackState::new(metadata, &backend);
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
        let context = Context {
            address: contract_address,
            caller: caller,
            apparent_value: Default::default(),
        };
        let mut runtime = evm::Runtime::new(code, Rc::new(data), context, &config);

        let metadata = StackSubstateMetadata::new(gas_limit, &config);
        let mstate = MemoryStackState::new(metadata, &backend);
        let mut executor = StackExecutor::new_with_precompiles(mstate, &config, &precompile);
        let reason = executor.execute(&mut runtime);
        let gas = executor.gas();
        let (values, logs) = executor.into_state().deconstruct();
        backend.apply(values, logs, false);
        let ret = runtime.machine().return_value();

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
