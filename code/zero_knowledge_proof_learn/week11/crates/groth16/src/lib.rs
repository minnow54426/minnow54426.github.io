//! # Groth16 Proving System
//!
//! This crate provides the Groth16 protocol implementation:
//! - Trusted setup (generating pk and vk)
//! - Proof generation
//! - Proof verification

pub mod error;
pub mod keys;
pub mod prove;
pub mod setup;
pub mod verify;

pub use error::Groth16Error;
pub use keys::{ProvingKey, VerificationKey};
pub use prove::{generate_proof, generate_proof_test, Proof};
pub use setup::{trusted_setup, trusted_setup_test};
pub use verify::{batch_verify, verify_proof};
