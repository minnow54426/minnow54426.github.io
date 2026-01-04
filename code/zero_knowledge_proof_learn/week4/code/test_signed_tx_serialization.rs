//! Test to verify if SignedTransaction can be serialized
//! This is an investigation file to understand the serialization issues

use tx_rs::{Transaction, sign};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

fn main() {
    println!("Testing SignedTransaction serialization...\n");

    // Generate keypair
    let mut csprng = OsRng;
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

    // Create a transaction
    let tx = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        100,
        1,
    );

    // Sign the transaction
    let signature = sign(&tx, &keypair);

    println!("✓ Transaction created and signed");
    println!("  Signature type: {:?}", std::any::type_name::<ed25519_dalek::Signature>());

    // Test 1: Can Signature be serialized?
    println!("\n--- Test 1: Signature Serialization ---");
    test_signature_serialization(&signature);

    // Test 2: Can Transaction be serialized?
    println!("\n--- Test 2: Transaction Serialization ---");
    test_transaction_serialization(&tx);

    // Test 3: What if we create a wrapper struct?
    println!("\n--- Test 3: Wrapper Struct Approach ---");
    test_wrapper_struct(&tx, &signature);

    println!("\n=== SUMMARY ===");
    println!("The issue: SignedTransaction cannot derive Serialize/Deserialize");
    println!("Root cause: Transaction doesn't have derives, and Signature serialization");
    println!("            requires the 'serde' feature (which is already enabled)");
    println!("\nRecommended solution:");
    println!("  1. Add Serialize/Deserialize to Transaction in week3");
    println!("  2. Add Serialize/Deserialize to SignedTransaction in week3");
    println!("  3. This is feasible since ed25519-dalek already has serde feature");
}

fn test_signature_serialization(signature: &ed25519_dalek::Signature) {
    // Try to serialize signature as bytes
    let sig_bytes = signature.to_bytes();
    println!("  Signature as bytes: {} bytes", sig_bytes.len());

    // Try serde serialization
    match serde_json::to_string(signature) {
        Ok(json) => println!("  ✓ Signature CAN be serialized to JSON: {}", json),
        Err(e) => println!("  ✗ Signature CANNOT be serialized: {}", e),
    }
}

fn test_transaction_serialization(tx: &Transaction) {
    match serde_json::to_string(tx) {
        Ok(json) => println!("  ✓ Transaction CAN be serialized: {}", json),
        Err(e) => println!("  ✗ Transaction CANNOT be serialized: {}", e),
    }
}

fn test_wrapper_struct(tx: &Transaction, signature: &ed25519_dalek::Signature) {
    #[derive(Debug, Serialize, Deserialize)]
    struct SerializableSignedTransaction {
        tx: Transaction,  // This will fail
        signature: Vec<u8>, // Use bytes instead
    }

    // This will fail to compile because Transaction doesn't implement Serialize
    // But we can see if the approach works in principle
    println!("  Creating wrapper with Vec<u8> for signature...");
    println!("  This would work if we serialize signatures as byte arrays");
}
