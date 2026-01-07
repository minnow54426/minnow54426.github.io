//! # toychain-rs
//!
//! A minimal blockchain with fork support and longest-chain consensus.
//!
//! ## Overview
//!
//! This library provides a toy blockchain implementation that demonstrates:
//! - State transition functions (transactions, blocks)
//! - Fork handling and chain selection
//! - Longest-chain consensus rule
//! - Modular architecture with clear separation of concerns
//!
//! ## Architecture
//!
//! The codebase is organized into core modules:
//! - [`core::types`] - Common type definitions
//! - [`core::state`] - Account and state management
//! - [`core::block`] - Block structure and hashing
//! - [`core::tx`] - Transaction types (re-exported from tx-rs)
//! - [`core::chain`] - Blockchain storage and fork handling
//!
//! ## Example
//!
//! ```rust
//! use toychain_rs::{Blockchain, State, apply_block, Block, Account};
//! use toychain_rs::block_hash;
//! use ed25519_dalek::Keypair;
//! use rand::rngs::OsRng;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create blockchain and state
//! let mut blockchain = Blockchain::new();
//! let mut state = State::new();
//!
//! // Setup accounts
//! let alice = Keypair::generate(&mut OsRng);
//! state.set_account(alice.public, Account::new(100, 0));
//!
//! // Create and add genesis block
//! let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);
//! blockchain.add_block(genesis)?;
//!
//! // Create a block with transactions
//! // ... (create signed transactions)
//! // let block = Block::new(...);
//! // blockchain.add_block(block)?;
//!
//! # Ok(())
//! # }
//! ```

pub mod core;

// Public API exports - core types
pub use core::{
    block::{block_hash, Block},
    chain::{apply_block, Blockchain},
    state::{Account, State},
    tx::{sign, SignedTransaction, Transaction},
};

// Public API exports - type aliases
pub use core::types::{Hash, Height, Timestamp, Balance, Nonce};

#[cfg(test)]
mod public_api_tests {
    use super::*;

    #[test]
    fn test_core_modules_are_accessible() {
        // Test that all core types are accessible
        let _block = Block::new([0u8; 32], vec![], 0, 0);
        let _state = State::new();
        let _blockchain = Blockchain::new();
    }

    #[test]
    fn test_legacy_functions_still_work() {
        use tx_rs::Transaction;
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice = Keypair::generate(&mut OsRng);

        state.set_account(alice.public, Account::new(100, 0));

        let tx = Transaction::new(alice.public, alice.public, 10, 0);
        let sig = sign(&tx, &alice);
        let signed_tx = SignedTransaction::new(tx, sig);

        let block = Block::new([0u8; 32], vec![signed_tx], 1, 0);

        // Legacy apply_block still works
        assert!(apply_block(&mut state, &block).is_ok());
    }
}
