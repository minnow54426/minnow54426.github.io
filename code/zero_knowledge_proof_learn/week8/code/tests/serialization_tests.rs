use zk_groth16_snark::utils::serialization::{serialize_proof, deserialize_proof};

#[test]
fn test_proof_serialization() {
    let proof_bytes = vec![1u8; 288]; // Mock proof
    let serialized = serialize_proof(&proof_bytes).unwrap();
    let deserialized: Vec<u8> = deserialize_proof(&serialized).unwrap();
    assert_eq!(proof_bytes, deserialized);
}
