//! Comparison benchmarks for all circuits
//!
//! This file provides side-by-side comparisons of all three circuits
//! to understand their relative performance characteristics.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sha2::{Digest, Sha256};
use zk_groth16_snark::{groth16, identity, membership, privacy, Groth16Circuit};
use ark_bn254::Fr;

/// Compare setup time across all circuits
fn bench_setup_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("setup_comparison");

    // Identity circuit
    let preimage = b"test_password";
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let hash: [u8; 32] = hasher.finalize().into();
    let identity_circuit = identity::IdentityCircuit::new(hash, preimage.len());

    group.bench_function("identity", |b| {
        b.iter(|| {
            let result = groth16::setup(black_box(&identity_circuit));
            black_box(result)
        })
    });

    // Privacy circuit
    let privacy_circuit = privacy::PrivacyCircuit::new(18, 150);

    group.bench_function("privacy", |b| {
        b.iter(|| {
            let result = groth16::setup(black_box(&privacy_circuit));
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

    group.bench_function("membership", |b| {
        b.iter(|| {
            let result = groth16::setup(black_box(&membership_circuit));
            black_box(result)
        })
    });

    group.finish();
}

/// Compare prove time across all circuits
fn bench_prove_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("prove_comparison");

    // Identity circuit
    let preimage = b"test_password";
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let hash: [u8; 32] = hasher.finalize().into();
    let identity_circuit = identity::IdentityCircuit::new(hash, preimage.len());
    let identity_witness = identity_circuit.generate_witness_for_preimage(preimage.to_vec());
    let (identity_pk, _) = groth16::setup(&identity_circuit).unwrap();

    group.bench_function("identity", |b| {
        b.iter(|| {
            let result =
                groth16::prove::<identity::IdentityCircuit>(black_box(&identity_pk), black_box(&identity_witness));
            black_box(result)
        })
    });

    // Privacy circuit
    let privacy_circuit = privacy::PrivacyCircuit::new(18, 150);
    let privacy_witness = privacy_circuit.generate_witness_for_value(27);
    let (privacy_pk, _) = groth16::setup(&privacy_circuit).unwrap();

    group.bench_function("privacy", |b| {
        b.iter(|| {
            let result =
                groth16::prove::<privacy::PrivacyCircuit>(black_box(&privacy_pk), black_box(&privacy_witness));
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
    let (membership_pk, _) = groth16::setup(&membership_circuit).unwrap();

    group.bench_function("membership", |b| {
        b.iter(|| {
            let result = groth16::prove::<membership::MembershipCircuit>(
                black_box(&membership_pk),
                black_box(&membership_witness),
            );
            black_box(result)
        })
    });

    group.finish();
}

/// Compare verify time across all circuits
fn bench_verify_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("verify_comparison");

    // Identity circuit
    let preimage = b"test_password";
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let hash: [u8; 32] = hasher.finalize().into();
    let identity_circuit = identity::IdentityCircuit::new(hash, preimage.len());
    let identity_witness = identity_circuit.generate_witness_for_preimage(preimage.to_vec());
    let (identity_pk, identity_vk) = groth16::setup(&identity_circuit).unwrap();
    let identity_proof = groth16::prove::<identity::IdentityCircuit>(&identity_pk, &identity_witness).unwrap();
    let identity_public_inputs = <identity::IdentityCircuit as Groth16Circuit<Fr>>::public_inputs(&identity_witness);

    group.bench_function("identity", |b| {
        b.iter(|| {
            let result = groth16::verify::<identity::IdentityCircuit>(
                black_box(&identity_vk),
                black_box(&identity_public_inputs),
                black_box(&identity_proof),
            );
            black_box(result)
        })
    });

    // Privacy circuit
    let privacy_circuit = privacy::PrivacyCircuit::new(18, 150);
    let privacy_witness = privacy_circuit.generate_witness_for_value(27);
    let (privacy_pk, privacy_vk) = groth16::setup(&privacy_circuit).unwrap();
    let privacy_proof = groth16::prove::<privacy::PrivacyCircuit>(&privacy_pk, &privacy_witness).unwrap();
    let privacy_public_inputs = <privacy::PrivacyCircuit as Groth16Circuit<Fr>>::public_inputs(&privacy_witness);

    group.bench_function("privacy", |b| {
        b.iter(|| {
            let result = groth16::verify::<privacy::PrivacyCircuit>(
                black_box(&privacy_vk),
                black_box(&privacy_public_inputs),
                black_box(&privacy_proof),
            );
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
    let (membership_pk, membership_vk) = groth16::setup(&membership_circuit).unwrap();
    let membership_proof =
        groth16::prove::<membership::MembershipCircuit>(&membership_pk, &membership_witness).unwrap();
    let membership_public_inputs = <membership::MembershipCircuit as Groth16Circuit<Fr>>::public_inputs(&membership_witness);

    group.bench_function("membership", |b| {
        b.iter(|| {
            let result = groth16::verify::<membership::MembershipCircuit>(
                black_box(&membership_vk),
                black_box(&membership_public_inputs),
                black_box(&membership_proof),
            );
            black_box(result)
        })
    });

    group.finish();
}

/// Compare full workflow (setup + prove + verify) across all circuits
fn bench_full_workflow_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_workflow_comparison");

    // Identity circuit
    let preimage = b"test_password";
    let mut hasher = Sha256::new();
    hasher.update(preimage);
    let hash: [u8; 32] = hasher.finalize().into();
    let identity_circuit = identity::IdentityCircuit::new(hash, preimage.len());
    let identity_witness = identity_circuit.generate_witness_for_preimage(preimage.to_vec());

    group.bench_function("identity", |b| {
        b.iter(|| {
            let (pk, vk) = groth16::setup(black_box(&identity_circuit)).unwrap();
            let proof = groth16::prove::<identity::IdentityCircuit>(&pk, &identity_witness).unwrap();
            let public_inputs = <identity::IdentityCircuit as Groth16Circuit<Fr>>::public_inputs(&identity_witness);
            let result = groth16::verify::<identity::IdentityCircuit>(&vk, &public_inputs, &proof);
            black_box(result)
        })
    });

    // Privacy circuit
    let privacy_circuit = privacy::PrivacyCircuit::new(18, 150);
    let privacy_witness = privacy_circuit.generate_witness_for_value(27);

    group.bench_function("privacy", |b| {
        b.iter(|| {
            let (pk, vk) = groth16::setup(black_box(&privacy_circuit)).unwrap();
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

    group.bench_function("membership", |b| {
        b.iter(|| {
            let (pk, vk) = groth16::setup(black_box(&membership_circuit)).unwrap();
            let proof = groth16::prove::<membership::MembershipCircuit>(&pk, &membership_witness).unwrap();
            let public_inputs = <membership::MembershipCircuit as Groth16Circuit<Fr>>::public_inputs(&membership_witness);
            let result = groth16::verify::<membership::MembershipCircuit>(&vk, &public_inputs, &proof);
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_setup_comparison,
    bench_prove_comparison,
    bench_verify_comparison,
    bench_full_workflow_comparison
);
criterion_main!(benches);
