/// Verification test for SignedTransaction serialization
///
/// This test demonstrates that after adding the Serialize/Deserialize derives,
/// SignedTransaction can be serialized and deserialized correctly.
///
/// Add this test to week3/code/src/crypto.rs in the #[cfg(test)] mod tests section

#[test]
fn test_signed_transaction_can_be_serialized() {
    use super::*;
    use crate::transaction::Transaction;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    // Create a signed transaction
    let mut csprng = OsRng;
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

    let tx = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        100,
        1,
    );

    let signature = sign(&tx, &keypair);
    let signed_tx = SignedTransaction::new(tx.clone(), signature);

    // Test 1: JSON serialization
    let json = serde_json::to_string(&signed_tx)
        .expect("SignedTransaction should serialize to JSON");
    println!("JSON serialization successful: {}", json);

    // Test 2: JSON deserialization
    let deserialized_from_json: SignedTransaction = serde_json::from_str(&json)
        .expect("Should deserialize from JSON");

    assert_eq!(signed_tx.tx_id, deserialized_from_json.tx_id,
        "Transaction ID should match after JSON round-trip");
    assert_eq!(signed_tx.signature, deserialized_from_json.signature,
        "Signature should match after JSON round-trip");

    // Test 3: Binary serialization
    let bytes = bincode::serialize(&signed_tx)
        .expect("SignedTransaction should serialize to binary");
    println!("Binary serialization successful: {} bytes", bytes.len());

    // Test 4: Binary deserialization
    let deserialized_from_bytes: SignedTransaction = bincode::deserialize(&bytes)
        .expect("Should deserialize from binary");

    assert_eq!(signed_tx.tx_id, deserialized_from_bytes.tx_id,
        "Transaction ID should match after binary round-trip");
    assert_eq!(signed_tx.signature, deserialized_from_bytes.signature,
        "Signature should match after binary round-trip");

    // Test 5: Signature still valid after deserialization
    assert!(deserialized_from_json.verify(),
        "Signature should still be valid after JSON round-trip");
    assert!(deserialized_from_bytes.verify(),
        "Signature should still be valid after binary round-trip");

    println!("✓ All serialization tests passed!");
}

#[test]
fn test_transaction_can_be_serialized() {
    use crate::transaction::Transaction;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    let mut csprng = OsRng;
    let keypair1: Keypair = Keypair::generate(&mut csprng);
    let keypair2: Keypair = Keypair::generate(&mut csprng);

    let tx = Transaction::new(
        keypair1.public,
        keypair2.public,
        100,
        1,
    );

    // Test binary serialization
    let bytes = bincode::serialize(&tx)
        .expect("Transaction should serialize to binary");
    println!("Transaction serialized to {} bytes", bytes.len());

    let deserialized: Transaction = bincode::deserialize(&bytes)
        .expect("Should deserialize from binary");

    assert_eq!(tx, deserialized,
        "Transaction should be identical after round-trip");

    println!("✓ Transaction serialization test passed!");
}

#[test]
fn test_hashable_public_key_can_be_serialized() {
    use crate::transaction::HashablePublicKey;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    let mut csprng = OsRng;
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let pubkey = HashablePublicKey::from(keypair.public);

    // Test JSON serialization
    let json = serde_json::to_string(&pubkey)
        .expect("HashablePublicKey should serialize to JSON");
    println!("HashablePublicKey JSON: {}", json);

    let deserialized: HashablePublicKey = serde_json::from_str(&json)
        .expect("Should deserialize from JSON");

    assert_eq!(pubkey.0.as_bytes(), deserialized.0.as_bytes(),
        "Public key bytes should match after round-trip");

    println!("✓ HashablePublicKey serialization test passed!");
}
