//! Individual circuit benchmarks
//!
//! This file benchmarks the setup, prove, and verify operations
//! for each circuit independently.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sha2::{Digest, Sha256};
use zk_groth16_snark::{groth16, identity, membership, privacy, Groth16Circuit};
use ark_bn254::Fr;

/// Benchmark identity circuit operations
fn bench_identity(c: &mut Criterion) {
    // Create test data: hash preimage for "test_password"
    let preimage = b"test_password";
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let hash: [u8; 32] = hasher.finalize().into();

    let circuit = identity::IdentityCircuit::new(hash, preimage.len());
    let witness = circuit.generate_witness_for_preimage(preimage.to_vec());

    // Benchmark setup
    c.bench_function("identity_setup", |b| {
        b.iter(|| {
            let result = groth16::setup(&circuit);
            black_box(result)
        })
    });

    // Setup once for prove/verify benchmarks
    let (pk, vk) = groth16::setup(&circuit).unwrap();
    let public_inputs = <identity::IdentityCircuit as Groth16Circuit<Fr>>::public_inputs(&witness);

    // Benchmark prove
    c.bench_function("identity_prove", |b| {
        b.iter(|| {
            let result = groth16::prove::<identity::IdentityCircuit>(&pk, black_box(&witness));
            black_box(result)
        })
    });

    // Benchmark verify
    let proof = groth16::prove::<identity::IdentityCircuit>(&pk, &witness).unwrap();
    c.bench_function("identity_verify", |b| {
        b.iter(|| {
            let result =
                groth16::verify::<identity::IdentityCircuit>(&vk, black_box(&public_inputs), &proof);
            black_box(result)
        })
    });
}

/// Benchmark privacy circuit operations
fn bench_privacy(c: &mut Criterion) {
    // Create test data: prove age 27 is in range [18, 150]
    let min_age = 18u64;
    let max_age = 150u64;
    let age = 27u64;

    let circuit = privacy::PrivacyCircuit::new(min_age, max_age);
    let witness = circuit.generate_witness_for_value(age);

    // Benchmark setup
    c.bench_function("privacy_setup", |b| {
        b.iter(|| {
            let result = groth16::setup(&circuit);
            black_box(result)
        })
    });

    // Setup once for prove/verify benchmarks
    let (pk, vk) = groth16::setup(&circuit).unwrap();
    let public_inputs = <privacy::PrivacyCircuit as Groth16Circuit<Fr>>::public_inputs(&witness);

    // Benchmark prove
    c.bench_function("privacy_prove", |b| {
        b.iter(|| {
            let result = groth16::prove::<privacy::PrivacyCircuit>(&pk, black_box(&witness));
            black_box(result)
        })
    });

    // Benchmark verify
    let proof = groth16::prove::<privacy::PrivacyCircuit>(&pk, &witness).unwrap();
    c.bench_function("privacy_verify", |b| {
        b.iter(|| {
            let result =
                groth16::verify::<privacy::PrivacyCircuit>(&vk, black_box(&public_inputs), &proof);
            black_box(result)
        })
    });
}

/// Benchmark membership circuit operations
fn bench_membership(c: &mut Criterion) {
    // Create test data: prove leaf in Merkle tree of depth 8
    let leaf = b"secret_leaf_data";
    let mut path = vec![];

    // Build a Merkle path of depth 8
    for i in 0..8 {
        let sibling = membership::MembershipCircuit::hash_leaf(format!("sibling_{}", i).as_bytes());
        path.push(sibling);
    }

    let root = membership::MembershipCircuit::compute_root(leaf, &path);
    let circuit = membership::MembershipCircuit::new(root);
    let witness = circuit.generate_witness_for_path(leaf.to_vec(), path);

    // Benchmark setup
    c.bench_function("membership_setup", |b| {
        b.iter(|| {
            let result = groth16::setup(&circuit);
            black_box(result)
        })
    });

    // Setup once for prove/verify benchmarks
    let (pk, vk) = groth16::setup(&circuit).unwrap();
    let public_inputs = <membership::MembershipCircuit as Groth16Circuit<Fr>>::public_inputs(&witness);

    // Benchmark prove
    c.bench_function("membership_prove", |b| {
        b.iter(|| {
            let result =
                groth16::prove::<membership::MembershipCircuit>(&pk, black_box(&witness));
            black_box(result)
        })
    });

    // Benchmark verify
    let proof = groth16::prove::<membership::MembershipCircuit>(&pk, &witness).unwrap();
    c.bench_function("membership_verify", |b| {
        b.iter(|| {
            let result = groth16::verify::<membership::MembershipCircuit>(
                &vk,
                black_box(&public_inputs),
                &proof,
            );
            black_box(result)
        })
    });
}

/// Benchmark full workflow for each circuit
fn bench_full_workflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_workflow");

    // Identity circuit
    let preimage = b"test_password";
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let hash: [u8; 32] = hasher.finalize().into();
    let identity_circuit = identity::IdentityCircuit::new(hash, preimage.len());
    let identity_witness = identity_circuit.generate_witness_for_preimage(preimage.to_vec());

    group.bench_function("identity_full", |b| {
        b.iter(|| {
            let (pk, vk) = groth16::setup(&identity_circuit).unwrap();
            let proof = groth16::prove::<identity::IdentityCircuit>(&pk, &identity_witness).unwrap();
            let public_inputs = <identity::IdentityCircuit as Groth16Circuit<Fr>>::public_inputs(&identity_witness);
            let result = groth16::verify::<identity::IdentityCircuit>(&vk, &public_inputs, &proof);
            black_box(result)
        })
    });

    // Privacy circuit
    let privacy_circuit = privacy::PrivacyCircuit::new(18, 150);
    let privacy_witness = privacy_circuit.generate_witness_for_value(27);

    group.bench_function("privacy_full", |b| {
        b.iter(|| {
            let (pk, vk) = groth16::setup(&privacy_circuit).unwrap();
            let proof = groth16::prove::<privacy::PrivacyCircuit>(&pk, &privacy_witness).unwrap();
            let public_inputs = <privacy::PrivacyCircuit as Groth16Circuit<Fr>>::public_inputs(&privacy_witness);
            let result = groth16::verify::<privacy::PrivacyCircuit>(&vk, &public_inputs, &proof);
            black_box(result)
        })
    });

    // Membership circuit
    let leaf = b"secret_leaf_data";
    let mut path = vec![];
    for i in 0..8 {
        let sibling = membership::MembershipCircuit::hash_leaf(format!("sibling_{}", i).as_bytes());
        path.push(sibling);
    }
    let root = membership::MembershipCircuit::compute_root(leaf, &path);
    let membership_circuit = membership::MembershipCircuit::new(root);
    let membership_witness = membership_circuit.generate_witness_for_path(leaf.to_vec(), path);

    group.bench_function("membership_full", |b| {
        b.iter(|| {
            let (pk, vk) = groth16::setup(&membership_circuit).unwrap();
            let proof =
                groth16::prove::<membership::MembershipCircuit>(&pk, &membership_witness).unwrap();
            let public_inputs = <membership::MembershipCircuit as Groth16Circuit<Fr>>::public_inputs(&membership_witness);
            let result =
                groth16::verify::<membership::MembershipCircuit>(&vk, &public_inputs, &proof);
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_identity,
    bench_privacy,
    bench_membership,
    bench_full_workflow
);
criterion_main!(benches);
