//! # Quadratic Arithmetic Programs (QAP)
//!
//! This crate provides QAP representation and R1CS to QAP transformation:
//! - R1CS to QAP conversion using Lagrange interpolation
//! - Polynomial divisibility checking

pub mod divisibility;
pub mod error;
pub mod polynomials;

pub use divisibility::{check_divisibility, target_polynomial};
pub use error::QapError;
pub use polynomials::{lagrange_interpolate, r1cs_to_qap};
