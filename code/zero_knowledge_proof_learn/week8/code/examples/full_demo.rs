//! Full Demo Example
//!
//! This example demonstrates all three circuits in the zk-groth16-snark library:
//! - Identity: Hash preimage proofs
//! - Membership: Merkle tree inclusion proofs
//! - Privacy: Range proofs
//!
//! Each circuit shows a complete setup, prove, and verify pipeline.

use sha2::{Digest, Sha256};
use zk_groth16_snark::groth16::{setup, prove, verify};
use zk_groth16_snark::identity::IdentityCircuit;
use zk_groth16_snark::membership::MembershipCircuit;
use zk_groth16_snark::privacy::PrivacyCircuit;
use zk_groth16_snark::Groth16Circuit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== zk-groth16-snark Full Demo ===\n");

    // Identity Circuit Demo
    println!("1. Identity Circuit");
    println!("   Use Case: Prove knowledge of a password without revealing it\n");

    let password = "my_secret_password";
    let password_hash = sha256(password.as_bytes());
    println!("   Password hash: {:02x?}", password_hash);

    let identity_circuit = IdentityCircuit::new(password_hash, password.len());

    println!("\n   [1/3] Setup...");
    let (identity_pk, identity_vk) = setup::<IdentityCircuit>(&identity_circuit)?;
    println!("   ✓ Setup complete");

    println!("\n   [2/3] Prove...");
    let identity_witness = identity_circuit.generate_witness_for_preimage(password.as_bytes().to_vec());
    let identity_public_inputs = IdentityCircuit::public_inputs(&identity_witness);
    let identity_proof = prove::<IdentityCircuit>(&identity_pk, &identity_witness)?;
    println!("   ✓ Proof generated: {} bytes", identity_proof.len());

    println!("\n   [3/3] Verify...");
    let identity_valid = verify::<IdentityCircuit>(&identity_vk, &identity_public_inputs, &identity_proof)?;
    println!("   Verification: {}\n", if identity_valid { "✓ VALID" } else { "✗ INVALID" });

    // Membership Circuit Demo
    println!("2. Membership Circuit");
    println!("   Use Case: Prove membership in a Merkle tree without revealing which leaf\n");

    let membership_root = [0u8; 32];
    let membership_leaf = b"secret_value";
    let membership_path = vec![[1u8; 32]; 8];

    println!("   Merkle root: {:02x?}", membership_root);
    println!("   Leaf value: {:?}", String::from_utf8_lossy(membership_leaf));

    let membership_circuit = MembershipCircuit::new(membership_root);

    println!("\n   [1/3] Setup...");
    let (membership_pk, membership_vk) = setup::<MembershipCircuit>(&membership_circuit)?;
    println!("   ✓ Setup complete");

    println!("\n   [2/3] Prove...");
    let membership_witness = membership_circuit.generate_witness_for_path(
        membership_leaf.to_vec(),
        membership_path.clone()
    );
    let membership_public_inputs = MembershipCircuit::public_inputs(&membership_witness);
    let membership_proof = prove::<MembershipCircuit>(&membership_pk, &membership_witness)?;
    println!("   ✓ Proof generated: {} bytes", membership_proof.len());

    println!("\n   [3/3] Verify...");
    let membership_valid = verify::<MembershipCircuit>(&membership_vk, &membership_public_inputs, &membership_proof)?;
    println!("   Verification: {}\n", if membership_valid { "✓ VALID" } else { "✗ INVALID" });

    // Privacy Circuit Demo
    println!("3. Privacy Circuit");
    println!("   Use Case: Prove a value is within a range without revealing the value\n");

    let privacy_min = 10u64;
    let privacy_max = 100u64;
    let privacy_value = 50u64;

    println!("   Range: [{}, {}]", privacy_min, privacy_max);
    println!("   Secret value: {}", privacy_value);

    let privacy_circuit = PrivacyCircuit::new(privacy_min, privacy_max);

    println!("\n   [1/3] Setup...");
    let (privacy_pk, privacy_vk) = setup::<PrivacyCircuit>(&privacy_circuit)?;
    println!("   ✓ Setup complete");

    println!("\n   [2/3] Prove...");
    let privacy_witness = privacy_circuit.generate_witness_for_value(privacy_value);
    let privacy_public_inputs = PrivacyCircuit::public_inputs(&privacy_witness);
    let privacy_proof = prove::<PrivacyCircuit>(&privacy_pk, &privacy_witness)?;
    println!("   ✓ Proof generated: {} bytes", privacy_proof.len());

    println!("\n   [3/3] Verify...");
    let privacy_valid = verify::<PrivacyCircuit>(&privacy_vk, &privacy_public_inputs, &privacy_proof)?;
    println!("   Verification: {}\n", if privacy_valid { "✓ VALID" } else { "✗ INVALID" });

    // Summary
    println!("=== Demo Complete ===");
    println!("\nAll circuits executed successfully!");
    println!("- Identity proof: {}", if identity_valid { "✓" } else { "✗" });
    println!("- Membership proof: {}", if membership_valid { "✓" } else { "✗" });
    println!("- Privacy proof: {}", if privacy_valid { "✓" } else { "✗" });
    println!("\nThank you for trying zk-groth16-snark!");

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
