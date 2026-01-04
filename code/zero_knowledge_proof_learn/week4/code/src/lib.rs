//! # toychain-rs
//!
//! A minimal blockchain state transition function implementation.
//!
//! ## Overview
//!
//! This library provides the core state management and transaction validation
//! logic for a simple account-based blockchain. It demonstrates how transactions
//! are validated (signature, balance, nonce) and applied to state atomically.
//!
//! ## Key Types
//!
//! - [`State`]: Manages account storage and transaction application
//! - [`Account`]: Represents a user's balance and nonce
//! - [`Block`]: Contains ordered transactions with metadata
//!
//! ## Example
//!
//! ```rust
//! use toychain_rs::{State, Account, Block, apply_block};
//! use tx_rs::{Transaction, SignedTransaction, sign};
//! use ed25519_dalek::Keypair;
//! use rand::rngs::OsRng;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut state = State::new();
//! let alice = Keypair::generate(&mut OsRng);
//!
//! state.set_account(alice.public, Account::new(100, 0));
//!
//! let tx = Transaction::new(alice.public, alice.public, 10, 0);
//! let sig = sign(&tx, &alice);
//! let signed_tx = SignedTransaction::new(tx, sig);
//!
//! let block = Block::new([0u8; 32], vec![signed_tx], 1, 0);
//! apply_block(&mut state, &block)?;
//! # Ok(())
//! # }
//! ```

pub mod block;
pub mod chain;
pub mod state;

// Public API exports
pub use block::{block_hash, Block};
pub use chain::apply_block;
pub use state::{Account, State};
