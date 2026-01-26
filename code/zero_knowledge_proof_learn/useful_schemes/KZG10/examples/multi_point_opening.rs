//! Multi-point opening example for KZG10
//!
//! Demonstrates opening a polynomial at multiple points.

use ark_bls12_381::{Bls12_381, Fr};
use ark_poly::DenseUVPolynomial;
use ark_poly::Polynomial;
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_std::UniformRand;
use ark_std::test_rng;
use kzg10::{Commitment, SRS};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = test_rng();
    let max_degree = 20;

    // Setup
    let secret = Fr::rand(&mut rng);
    let srs = SRS::<Bls12_381>::setup_for_testing(secret, max_degree)?;
    println!("🔐 SRS generated\n");

    // Create polynomial: P(x) = x³ + 2x² + 3x + 4
    let coeffs = vec![Fr::from(4u64), Fr::from(3u64), Fr::from(2u64), Fr::from(1u64)];
    let poly = DensePolynomial::from_coefficients_vec(coeffs);
    println!("📝 Polynomial: P(x) = x³ + 2x² + 3x + 4\n");

    let commitment = Commitment::commit(&poly, &srs);
    println!("✓ Committed\n");

    // Generate proofs at multiple points
    let points = vec![1u64, 2, 3, 5, 7, 11, 13];
    let mut all_valid = true;

    println!("🔍 Generating and verifying proofs at {} points:\n", points.len());
    for &x in &points {
        let point = Fr::from(x);
        let value = poly.evaluate(&point);
        println!("  P({}) = {}", x, value);

        let opening = Commitment::open(&poly, point, &srs);
        let is_valid = commitment.verify(&opening, &srs);
        
        if !is_valid {
            all_valid = false;
        }
    }

    println!("\n✓ Generated and verified {} proofs", points.len());
    println!("✅ All valid: {}\n", all_valid);

    Ok(())
}
