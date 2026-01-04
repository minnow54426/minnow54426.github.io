//! tx-rs: Educational Transaction and Signature Library
//!
//! This library demonstrates how blockchain transactions work with digital signatures.
//! It covers key concepts like:
//! - Transaction structure and serialization
//! - Digital signatures using ed25519
//! - Transaction verification and replay protection
//! - Mempool management and deduplication

pub mod transaction;
pub mod crypto;
pub mod mempool;

pub use transaction::{Transaction, TxId, HashablePublicKey};
pub use crypto::{SignedTransaction, sign, verify, CryptoError};
pub use mempool::{Mempool, MempoolError};

// Re-export for convenience
pub use ed25519_dalek::{PublicKey, SecretKey, Keypair};