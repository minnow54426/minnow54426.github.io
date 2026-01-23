//! BIP340 Schnorr signatures on secp256k1
//!
//! This library implements the Schnorr signature scheme as specified in BIP340:
//! <https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki>
//!
//! # Features
//!
//! - Deterministic nonce generation (prevents nonce reuse)
//! - Single and batch verification
//! - Type-safe API (newtype wrappers prevent misuse)
//! - Constant-time operations (timing attack resistant)
//!
//! # Example
//!
//! ```rust,no_run,ignore
//! use schnorr::KeyPair;
//! use rand::rngs::OsRng;
//!
//! # let mut rng = OsRng;
//! # let keypair = KeyPair::new(&mut rng);
//! # let message = b"Hello, Schnorr!";
//! # let signature = keypair.sign(message);
//! # assert!(keypair.public_key().verify(message, &signature).is_ok());
//! ```

pub mod error;
pub mod keypair;
pub mod signature;

mod challenge;
mod nonce;
mod sign;
mod verify;

// Re-export public types
pub use error::Error;
pub use keypair::{KeyPair, PublicKey, SecretKey};
pub use signature::Signature;
pub use verify::verify_batch;
