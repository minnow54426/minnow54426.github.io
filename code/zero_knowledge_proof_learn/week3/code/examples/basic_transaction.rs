//! Basic transaction example demonstrating the complete workflow
//!
//! This example shows how to:
//! 1. Create a transaction
//! 2. Sign it with a private key
//! 3. Verify the signature
//! 4. Add it to a mempool

use tx_rs::{Transaction, sign, Mempool};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Transaction Example ===\n");

    // Generate cryptographic key pairs for Alice and Bob
    let mut csprng = OsRng;
    let alice_keypair = Keypair::generate(&mut csprng);
    let bob_keypair = Keypair::generate(&mut csprng);

    println!("Alice's public key: {:?}", alice_keypair.public);
    println!("Bob's public key: {:?}\n", bob_keypair.public);

    // Create a transaction from Alice to Bob
    let tx = Transaction::new(
        alice_keypair.public,
        bob_keypair.public,
        100, // Amount
        1,    // Nonce
    );

    println!("Created transaction:");
    println!("  From: {:?}", tx.from_pubkey);
    println!("  To: {:?}", tx.to_pubkey);
    println!("  Amount: {}", tx.amount);
    println!("  Nonce: {}", tx.nonce);
    println!("  TxId: {}\n", tx.compute_id());

    // Alice signs the transaction
    let signature = sign(&tx, &alice_keypair);
    println!("Transaction signed by Alice");
    println!("Signature: {:?}\n", signature);

    // Create a signed transaction
    let signed_tx = tx_rs::SignedTransaction::new(tx.clone(), signature);

    // Verify the signature
    let is_valid = signed_tx.verify();
    println!("Signature verification: {}\n", is_valid);

    if !is_valid {
        return Err("Signature verification failed!".into());
    }

    // Add to mempool
    let mut mempool = Mempool::new();
    mempool.add_transaction(signed_tx)?;

    println!("Transaction added to mempool");
    println!("Mempool size: {}", mempool.len());
    println!("Alice's current nonce: {}", mempool.get_account_nonce(&alice_keypair.public));

    // Try to add the same transaction again (should fail)
    let duplicate_signature = sign(&tx, &alice_keypair);
    let duplicate_signed_tx = tx_rs::SignedTransaction::new(tx, duplicate_signature);

    match mempool.add_transaction(duplicate_signed_tx) {
        Ok(_) => println!("ERROR: Duplicate transaction was accepted!"),
        Err(e) => println!("Correctly rejected duplicate transaction: {}", e),
    }

    println!("\n=== Example completed successfully! ===");
    Ok(())
}