//! Identity Proof Example
//!
//! This example demonstrates the Identity circuit for hash preimage proofs.
//! It shows how to create a circuit that proves knowledge of a password
//! without revealing the password itself.

use sha2::{Digest, Sha256};
use zk_groth16_snark::groth16::{setup, prove, verify};
use zk_groth16_snark::identity::IdentityCircuit;
use zk_groth16_snark::Groth16Circuit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Identity Circuit Demo ===\n");

    let password = "my_secret_password";
    let password_hash = sha256(password.as_bytes());
    println!("Password hash: {:02x?}", password_hash);

    let circuit = IdentityCircuit::new(password_hash, password.len());

    // Setup
    println!("\n[1/3] Running trusted setup...");
    let (pk, vk) = setup::<IdentityCircuit>(&circuit)?;
    println!("  ✓ Setup complete");

    // Prove
    println!("\n[2/3] Generating proof...");
    let witness = circuit.generate_witness_for_preimage(password.as_bytes().to_vec());
    let public_inputs = IdentityCircuit::public_inputs(&witness);
    let proof = prove::<IdentityCircuit>(&pk, &witness)?;
    println!("  ✓ Proof generated: {} bytes", proof.len());

    // Verify
    println!("\n[3/3] Verifying proof...");
    let is_valid = verify::<IdentityCircuit>(&vk, &public_inputs, &proof)?;
    println!("  Verification: {}", if is_valid { "✓ VALID" } else { "✗ INVALID" });

    if is_valid {
        println!("\n✅ Success! Password proven without revealing it");
    }

    Ok(())
}

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
