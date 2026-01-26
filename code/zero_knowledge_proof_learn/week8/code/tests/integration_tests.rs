//! Integration Tests for zk-groth16-snark
//!
//! This file contains end-to-end integration tests for all circuits,
//! testing the full pipeline: setup -> prove -> verify

use zk_groth16_snark::{identity::IdentityCircuit, privacy::PrivacyCircuit, membership::MembershipCircuit};
use zk_groth16_snark::groth16::{setup, prove, verify};
use zk_groth16_snark::Groth16Circuit;

#[test]
fn test_full_pipeline_identity() {
    let password = b"test_password";
    let hash = sha256(password);
    let circuit = IdentityCircuit::new(hash, password.len());

    let (pk, vk) = setup::<IdentityCircuit>(&circuit).unwrap();
    let witness = circuit.generate_witness_for_preimage(password.to_vec());
    let public_inputs = IdentityCircuit::public_inputs(&witness);
    let proof = prove::<IdentityCircuit>(&pk, &witness).unwrap();
    let is_valid = verify::<IdentityCircuit>(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}

#[test]
fn test_full_pipeline_privacy() {
    let circuit = PrivacyCircuit::new(10, 100);

    let (pk, vk) = setup::<PrivacyCircuit>(&circuit).unwrap();
    let witness = circuit.generate_witness_for_value(50);
    let public_inputs = PrivacyCircuit::public_inputs(&witness);
    let proof = prove::<PrivacyCircuit>(&pk, &witness).unwrap();
    let is_valid = verify::<PrivacyCircuit>(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}

#[test]
fn test_full_pipeline_membership() {
    let root = [0u8; 32];
    let leaf = b"secret";
    let path = vec![[1u8; 32]; 8];

    let circuit = MembershipCircuit::new(root);
    let (pk, vk) = setup::<MembershipCircuit>(&circuit).unwrap();
    let witness = circuit.generate_witness_for_path(leaf.to_vec(), path);
    let public_inputs = MembershipCircuit::public_inputs(&witness);
    let proof = prove::<MembershipCircuit>(&pk, &witness).unwrap();
    let is_valid = verify::<MembershipCircuit>(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}

fn sha256(data: &[u8]) -> [u8; 32] {
    use sha2::Sha256;
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
