//! Ethereum Virtual Machine implementation in Rust

extern crate alloc;
extern crate core;

pub use evm_core::*;
pub use evm_runtime::*;

pub mod backend;
pub mod executor;
mod executive;