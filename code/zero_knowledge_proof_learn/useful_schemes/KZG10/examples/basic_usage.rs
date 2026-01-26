//! Basic usage example for KZG10 polynomial commitment scheme
//!
//! # WARNING
//!
//! This example uses `setup_for_testing()` which generates a secret value.
//! **NEVER use this in production!** In production, always use a trusted
//! ceremony output like Ethereum's KZG ceremony.

use ark_bls12_381::{Bls12_381, Fr};
use ark_poly::DenseUVPolynomial;
use ark_poly::Polynomial;
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_std::UniformRand;
use ark_std::test_rng;
use kzg10::{Commitment, SRS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = test_rng();
    let max_degree = 10;

    // Setup: Generate SRS (FOR TESTING ONLY - never use in production!)
    let secret = Fr::rand(&mut rng);
    let srs = SRS::<Bls12_381>::setup_for_testing(secret, max_degree)?;
    println!("✓ SRS generated (max degree: {})\n", max_degree);

    // Create polynomial: P(x) = x² + 2x + 1
    let coeffs = vec![Fr::from(1u64), Fr::from(2u64), Fr::from(1u64)];
    let poly = DensePolynomial::from_coefficients_vec(coeffs);
    println!("📝 Polynomial: P(x) = x² + 2x + 1");

    // Commit to polynomial
    let commitment = Commitment::commit(&poly, &srs);
    println!("✓ Committed: C = P(s)·G (48 bytes)\n");

    // Generate proof at point z = 5
    let point = Fr::from(5u64);
    let value = poly.evaluate(&point);
    println!("🔍 Opening: z = {}, P({}) = {}", point, point, value);

    let opening = Commitment::open(&poly, point, &srs);
    println!("✓ Generated proof (48 bytes)\n");

    // Verify proof
    let is_valid = commitment.verify(&opening, &srs);
    println!("✅ Verification: {}\n", is_valid);
    assert!(is_valid);

    println!("✅ All operations successful!");
    Ok(())
}
