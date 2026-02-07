//! # Example Circuits
//!
//! This crate provides example circuits demonstrating Groth16:
//! - Simple multiplier (a × b = c)
//! - Cubic polynomial (ax³ + bx² + cx + d = y)
//! - Hash preimage
//! - Merkle tree membership
//! - Range proof

pub mod cubic;
pub mod hash_preimage;
pub mod merkle;
pub mod multiplier;
pub mod range_proof;
