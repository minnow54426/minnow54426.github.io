//! # Groth16 Mathematical Primitives
//!
//! This crate provides core mathematical primitives for Groth16:
//! - Finite field operations
//! - Bilinear pairings
//! - Polynomial operations

pub mod fields;
pub mod pairing;
pub mod polynomial;

#[cfg(test)]
mod fields_tests;
#[cfg(test)]
mod pairing_tests;
#[cfg(test)]
mod polynomial_tests;
