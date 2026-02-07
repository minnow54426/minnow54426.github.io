//! Benchmarks for batch verification performance
//!
//! Run with: cargo bench --package groth16 --bench batch_verify
//!
//! These benchmarks compare individual verification vs batch verification
//! for different batch sizes. Note that the current implementation of
//! batch_verify is O(n) (same as individual), so we don't expect to see
//! significant speedups yet. These benchmarks establish a baseline for
//! future optimization.

use ark_bn254::Fq;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use groth16::{batch_verify, generate_proof_test, trusted_setup_test, verify_proof};
use groth16_math::fields::FieldWrapper;
use groth16_qap::r1cs_to_qap;
use groth16_r1cs::constraint::R1CSConstraint;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// Setup: Generate keys and proofs for benchmarking
///
/// Uses a simple multiplier circuit: a × b = c
#[allow(clippy::type_complexity)]
fn setup_batch(
    size: usize,
) -> (
    Vec<(groth16::Proof, Vec<FieldWrapper<Fq>>)>,
    groth16::VerificationKey,
) {
    // Create multiplier circuit: a × b = c
    // Using standard Groth16 witness ordering: [1, c, a, b]
    let mut c1 = R1CSConstraint::<Fq>::new();
    c1.add_a_variable(2, FieldWrapper::<Fq>::from(1u64)); // a at index 2
    c1.add_b_variable(3, FieldWrapper::<Fq>::from(1u64)); // b at index 3
    c1.add_c_variable(1, FieldWrapper::<Fq>::from(1u64)); // c at index 1

    let constraints = vec![c1.clone(), c1.clone()];
    let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

    // Setup with 1 public input (c)
    let seed = [42u8; 32];
    let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

    // Generate multiple proofs with different witnesses
    let proofs_and_inputs: Vec<_> = (0..size)
        .map(|i| {
            // Use different multipliers: (i+2) × (i+3) = result
            let a_val = (i as u64) + 2;
            let b_val = (i as u64) + 3;
            let c_val = a_val * b_val;

            let witness = vec![
                FieldWrapper::<Fq>::from(1u64),  // constant 1
                FieldWrapper::<Fq>::from(c_val), // c
                FieldWrapper::<Fq>::from(a_val), // a
                FieldWrapper::<Fq>::from(b_val), // b
            ];

            let proof =
                generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap();
            (proof, vec![FieldWrapper::<Fq>::from(c_val)])
        })
        .collect();

    (proofs_and_inputs, vk)
}

/// Benchmark: Individual verification (baseline)
///
/// This iterates through each proof and verifies it separately,
/// which is the O(n) baseline we want to compare against.
fn bench_individual_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("individual_verify");

    for size in [1, 5, 10, 25, 50].iter() {
        let (proofs_and_inputs, vk) = setup_batch(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                for (proof, public_inputs) in &proofs_and_inputs {
                    black_box(verify_proof(&vk, proof, public_inputs).unwrap());
                }
            });
        });
    }

    group.finish();
}

/// Benchmark: Batch verification
///
/// Currently O(n) but should become O(1) in future optimizations.
/// These benchmarks establish a baseline for measuring improvements.
fn bench_batch_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_verify");

    for size in [1, 5, 10, 25, 50].iter() {
        let (proofs_and_inputs, vk) = setup_batch(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let mut rng = ChaCha20Rng::from_entropy();
            b.iter(|| {
                black_box(batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap());
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_individual_verification,
    bench_batch_verification
);
criterion_main!(benches);
