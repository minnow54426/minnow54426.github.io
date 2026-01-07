//! Transaction module
//!
//! This module re-exports types from the `tx-rs` crate for convenience.
//! The actual transaction logic lives in the Week 3 `tx-rs` crate.

pub use tx_rs::{sign, SignedTransaction, Transaction};
