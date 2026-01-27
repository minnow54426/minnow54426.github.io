//! zk-proof-artifacts
//!
//! CLI tool and library for generating, serializing, and verifying ZK-SNARK proofs
//! with standardized JSON artifact formats.

pub mod error;
pub mod artifacts;
pub mod cli;
pub mod fileio;

pub use error::{CliError, CircuitType, Result};
pub use cli::Cli;
