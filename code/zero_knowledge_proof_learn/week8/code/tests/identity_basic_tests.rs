//! Tests for Identity circuit basic functionality

use ark_bn254::Fr;
use zk_groth16_snark::identity::IdentityCircuit;

#[test]
fn test_identity_circuit_new() {
    // Create an identity circuit with a hash
    let hash_val = [1u8; 32];
    let circuit = IdentityCircuit::new(hash_val);

    // Circuit should hold the expected hash
    assert_eq!(circuit.hash, hash_val);
}

#[test]
fn test_identity_circuit_empty_hash() {
    // Create with empty hash
    let hash_val = [0u8; 32];
    let circuit = IdentityCircuit::new(hash_val);

    assert_eq!(circuit.hash, hash_val);
}
