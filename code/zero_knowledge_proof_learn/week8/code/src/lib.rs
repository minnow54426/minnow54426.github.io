//! # zk-groth16-snark
//!
//! A Groth16 SNARK library demonstrating practical zero-knowledge proof implementations
//! across three real-world applications:
//!
//! - **Identity Circuit**: Hash preimage proofs (prove knowledge of secret without revealing it)
//! - **Membership Circuit**: Merkle tree membership (prove inclusion without revealing leaf)
//! - **Privacy Circuit**: Range proofs (prove value within bounds without revealing value)
//!
//! ## Quick Start
//!
//! TODO: Add quick start example when modules are implemented.
//!
//! ## Modules
//!
//! - [`circuit`] - Core trait abstraction for all circuits
//! - [`groth16`] - Setup/prove/verify infrastructure
//! - [`error`] - Comprehensive error types
//! - [`identity`] - Hash preimage circuit
//! - [`membership`] - Merkle membership circuit
//! - [`privacy`] - Range proof circuit
//!
//! ## Examples
//!
//! See the `examples/` directory for complete demonstrations:
//!
//! ```bash
//! cargo run --example identity_proof
//! cargo run --example membership_proof
//! cargo run --example privacy_proof
//! cargo run --example full_demo
//! ```

pub mod error;
pub mod circuit;
pub mod utils;
pub mod groth16;
pub mod identity;
pub mod privacy;

// pub mod membership;

// Re-exports for convenience
pub use error::{
    Error, ErrorKind, Result,
    CircuitError, SetupError, ProveError, VerifyError, SerializationError,
    IdentityError, MembershipError, PrivacyError,
};
// pub use groth16::{setup, verify};

// Re-export circuit traits
pub use circuit::Groth16Circuit;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
