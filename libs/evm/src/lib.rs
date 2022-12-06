use codec::{Decode, Encode};
use frame_support::weights::Weight;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{H160, U256};
use sp_std::vec::Vec;

pub use evm::{
    backend::{Basic as Account, Log},
    Config, ExitReason,
};

pub use self::{
    precompile::{
        Context, ExitError, ExitRevert, ExitSucceed, LinearCostPrecompile, Precompile,
        PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult, PrecompileSet,
        Transfer,
    },
    validation::{
        CheckEvmTransaction, CheckEvmTransactionConfig, CheckEvmTransactionInput,
        InvalidEvmTransactionError,
    },
};

#[derive(Clone, Eq, PartialEq, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
/// External input from the transaction.
pub struct Vicinity {
    /// Current transaction gas price.
    pub gas_price: U256,
    /// Origin of the transaction.
    pub origin: H160,
}

pub fn add(left: usize, right: usize) -> usize {
    // Execute the EVM call.
    let vicinity = Vicinity {
        gas_price: base_fee,
        origin: source,
    };

    let metadata = StackSubstateMetadata::new(gas_limit, config);
    let state = SubstrateStackState::new(&vicinity, metadata);
    let mut executor = StackExecutor::new_with_precompiles(state, config, precompiles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
