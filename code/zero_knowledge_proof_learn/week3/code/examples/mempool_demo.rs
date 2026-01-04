//! Mempool demonstration showing advanced features
//!
//! This example demonstrates:
//! - Multiple transactions from different users
//! - Nonce tracking and replay protection
//! - Transaction ordering and management
//! - Mempool queries and filtering

use tx_rs::{Transaction, sign, Mempool};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Mempool Demo ===\n");

    // Generate key pairs for three users
    let mut csprng = OsRng;
    let alice_keypair = Keypair::generate(&mut csprng);
    let bob_keypair = Keypair::generate(&mut csprng);
    let charlie_keypair = Keypair::generate(&mut csprng);

    println!("Generated keys for Alice, Bob, and Charlie\n");

    // Create a mempool
    let mut mempool = Mempool::new();

    // Alice sends transactions with increasing nonces
    println!("Alice's transactions:");
    for i in 1..=3 {
        let tx = Transaction::new(
            alice_keypair.public,
            bob_keypair.public,
            i * 50,
            i,
        );
        let signature = sign(&tx, &alice_keypair);
        let signed_tx = tx_rs::SignedTransaction::new(tx, signature);

        match mempool.add_transaction(signed_tx) {
            Ok(_) => println!("  ✓ Transaction {} (nonce: {}) added", i, i),
            Err(e) => println!("  ✗ Transaction {} failed: {}", i, e),
        }
    }

    // Bob sends a transaction
    println!("\nBob's transactions:");
    let bob_tx = Transaction::new(
        bob_keypair.public,
        charlie_keypair.public,
        25,
        1,
    );
    let bob_signature = sign(&bob_tx, &bob_keypair);
    let bob_signed_tx = tx_rs::SignedTransaction::new(bob_tx, bob_signature);

    match mempool.add_transaction(bob_signed_tx) {
        Ok(_) => println!("  ✓ Bob's transaction added"),
        Err(e) => println!("  ✗ Bob's transaction failed: {}", e),
    }

    // Charlie sends a transaction
    println!("\nCharlie's transactions:");
    let charlie_tx = Transaction::new(
        charlie_keypair.public,
        alice_keypair.public,
        75,
        1,
    );
    let charlie_signature = sign(&charlie_tx, &charlie_keypair);
    let charlie_signed_tx = tx_rs::SignedTransaction::new(charlie_tx, charlie_signature);

    match mempool.add_transaction(charlie_signed_tx) {
        Ok(_) => println!("  ✓ Charlie's transaction added"),
        Err(e) => println!("  ✗ Charlie's transaction failed: {}", e),
    }

    // Display mempool status
    println!("\n=== Mempool Status ===");
    println!("Total transactions: {}", mempool.len());
    println!("Alice's nonce: {}", mempool.get_account_nonce(&alice_keypair.public));
    println!("Bob's nonce: {}", mempool.get_account_nonce(&bob_keypair.public));
    println!("Charlie's nonce: {}", mempool.get_account_nonce(&charlie_keypair.public));

    // List all transactions
    println!("\n=== All Transactions ===");
    for (i, tx) in mempool.transactions().enumerate() {
        println!("{}. {}", i + 1, tx.tx_id);
        println!("   From -> To: {} -> {}",
                format_hashable_pubkey(&tx.tx.from_pubkey),
                format_hashable_pubkey(&tx.tx.to_pubkey));
        println!("   Amount: {}, Nonce: {}", tx.tx.amount, tx.tx.nonce);
    }

    // Try to add a transaction with invalid nonce (replay attack)
    println!("\n=== Replay Protection Test ===");
    let replay_tx = Transaction::new(
        alice_keypair.public,
        bob_keypair.public,
        999,
        2, // Same nonce as Alice's second transaction
    );
    let replay_signature = sign(&replay_tx, &alice_keypair);
    let replay_signed_tx = tx_rs::SignedTransaction::new(replay_tx, replay_signature);

    match mempool.add_transaction(replay_signed_tx) {
        Ok(_) => println!("  ✗ ERROR: Replay transaction was accepted!"),
        Err(e) => println!("  ✓ Correctly rejected replay attempt: {}", e),
    }

    // Filter transactions by sender
    println!("\n=== Alice's Transactions ===");
    for tx in mempool.transactions_by_sender(&alice_keypair.public) {
        println!("  TxId: {}, Amount: {}, Nonce: {}",
                tx.tx_id, tx.tx.amount, tx.tx.nonce);
    }

    // Simulate mining: remove some transactions
    println!("\n=== Simulating Mining ===");
    let tx_ids: Vec<_> = mempool.transaction_ids().cloned().collect();
    for tx_id in tx_ids.iter().take(2) {
        if let Some(removed_tx) = mempool.remove_transaction(tx_id) {
            println!("  Mined transaction: {} (Amount: {})",
                    removed_tx.tx_id, removed_tx.tx.amount);
        }
    }
    println!("Remaining transactions: {}", mempool.len());

    println!("\n=== Demo completed! ===");
    Ok(())
}

fn format_pubkey(pubkey: &ed25519_dalek::PublicKey) -> String {
    let bytes = pubkey.as_bytes();
    format!("0x{}", &hex::encode(&bytes[..8])) // Show first 8 bytes for readability
}

fn format_hashable_pubkey(pubkey: &tx_rs::HashablePublicKey) -> String {
    format_pubkey(&pubkey.0)
}