//! Tests for Privacy circuit basic functionality

use zk_groth16_snark::privacy::PrivacyCircuit;

#[test]
fn test_privacy_circuit_new() {
    // Create a privacy circuit with min and max bounds
    let min = 10u64;
    let max = 100u64;
    let circuit = PrivacyCircuit::new(min, max);

    // Circuit should hold the expected bounds
    assert_eq!(circuit.min, min);
    assert_eq!(circuit.max, max);
}

#[test]
fn test_privacy_circuit_zero_bounds() {
    // Create with zero bounds
    let circuit = PrivacyCircuit::new(0, 0);

    assert_eq!(circuit.min, 0);
    assert_eq!(circuit.max, 0);
}

#[test]
fn test_privacy_circuit_large_bounds() {
    // Create with large bounds
    let circuit = PrivacyCircuit::new(u64::MAX - 1, u64::MAX);

    assert_eq!(circuit.min, u64::MAX - 1);
    assert_eq!(circuit.max, u64::MAX);
}
