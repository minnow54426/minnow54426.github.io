use ed25519_dalek::{Signature, Keypair, Signer};
use rand::rngs::OsRng;
use serde_json;

fn main() {
    println!("Testing if ed25519_dalek::Signature can serialize...\n");

    let mut csprng = OsRng;
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let message = b"test message";
    let signature = keypair.sign(message);

    println!("✓ Created signature");
    println!("  Signature bytes: {:?}", signature.to_bytes());

    // Test JSON serialization
    match serde_json::to_string(&signature) {
        Ok(json) => {
            println!("\n✓ SUCCESS: Signature CAN be serialized to JSON!");
            println!("  JSON: {}", json);
        }
        Err(e) => {
            println!("\n✗ FAILED: Signature CANNOT be serialized");
            println!("  Error: {}", e);
        }
    }

    // Test binary serialization
    match bincode::serialize(&signature) {
        Ok(bytes) => {
            println!("\n✓ SUCCESS: Signature CAN be serialized to binary!");
            println!("  Bytes (hex): {}", hex::encode(&bytes));
        }
        Err(e) => {
            println!("\n✗ FAILED: Signature CANNOT be serialized to binary");
            println!("  Error: {}", e);
        }
    }
}
