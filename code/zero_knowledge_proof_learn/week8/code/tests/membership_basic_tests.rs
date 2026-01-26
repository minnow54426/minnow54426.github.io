//! Tests for Membership circuit basic functionality

use zk_groth16_snark::membership::MembershipCircuit;

#[test]
fn test_membership_circuit_new() {
    // Create a membership circuit with a root
    let root = [42u8; 32];
    let circuit = MembershipCircuit::new(root);

    // Circuit should hold the expected root
    assert_eq!(circuit.root, root);
}

#[test]
fn test_membership_circuit_empty_root() {
    // Create with empty root
    let root = [0u8; 32];
    let circuit = MembershipCircuit::new(root);

    assert_eq!(circuit.root, root);
}

#[test]
fn test_membership_circuit_various_roots() {
    // Test with different root values
    for i in 0..32 {
        let mut root = [0u8; 32];
        root[i] = 1;
        let circuit = MembershipCircuit::new(root);
        assert_eq!(circuit.root, root);
    }
}
