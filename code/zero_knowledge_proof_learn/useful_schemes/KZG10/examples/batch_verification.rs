//! Batch verification example for KZG10
//!
//! Demonstrates efficient batch verification of multiple proofs.

use ark_bls12_381::{Bls12_381, Fr};
use ark_poly::DenseUVPolynomial;
use ark_poly::Polynomial;
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_std::UniformRand;
use ark_std::test_rng;
use kzg10::{batch_verify, Commitment, SRS};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = test_rng();
    let max_degree = 20;
    let num_proofs = 100;

    // Setup
    let secret = Fr::rand(&mut rng);
    let srs = SRS::<Bls12_381>::setup_for_testing(secret, max_degree)?;
    println!("🔐 SRS generated (max degree: {})\n", max_degree);

    // Use the same evaluation point for all proofs
    let eval_point = Fr::from(5u64);

    // Generate commitments and proofs
    let mut commitments = Vec::new();
    let mut openings = Vec::new();

    for i in 0..num_proofs {
        let degree = 3 + (i % 5);
        let coeffs: Vec<_> = (0..=degree).map(|_| Fr::rand(&mut rng)).collect();
        let poly = DensePolynomial::from_coefficients_vec(coeffs);

        let commitment = Commitment::commit(&poly, &srs);
        let opening = Commitment::open(&poly, eval_point, &srs);

        commitments.push(commitment);
        openings.push(opening);
    }

    println!("✓ Generated {} commitments and proofs at point {}\n", num_proofs, eval_point);

    // Individual verification
    println!("🔍 Verifying individually...");
    let start = Instant::now();
    for (comm, opening) in commitments.iter().zip(openings.iter()) {
        comm.verify(opening, &srs);
    }
    let individual_time = start.elapsed();
    println!("✓ Time: {:?}\n", individual_time);

    // Batch verification
    println!("🚀 Verifying in batch...");
    let start = Instant::now();
    let batch_valid = batch_verify::<Bls12_381>(&commitments, &openings, &srs)?;
    let batch_time = start.elapsed();
    println!("✓ Time: {:?}", batch_time);
    println!("✓ All valid: {}\n", batch_valid);

    let speedup = individual_time.as_secs_f64() / batch_time.as_secs_f64();
    println!("📊 Speedup: {:.2}×", speedup);

    Ok(())
}
