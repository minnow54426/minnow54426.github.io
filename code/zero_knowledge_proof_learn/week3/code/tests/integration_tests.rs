//! Integration tests for the complete transaction workflow
//!
//! These tests verify that all components work together correctly,
//! following the requirements from the prompt.

use tx_rs::{Transaction, sign, Mempool};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[test]
fn test_complete_transaction_workflow() {
    // Setup
    let mut csprng = OsRng;
    let alice_keypair = Keypair::generate(&mut csprng);
    let bob_keypair = Keypair::generate(&mut csprng);

    // Create transaction
    let tx = Transaction::new(
        alice_keypair.public,
        bob_keypair.public,
        1000,
        1,
    );

    // Sign transaction
    let signature = sign(&tx, &alice_keypair);
    let signed_tx = tx_rs::SignedTransaction::new(tx, signature);

    // Verify signature
    assert!(signed_tx.verify());

    // Add to mempool
    let mut mempool = Mempool::new();
    assert!(mempool.add_transaction(signed_tx).is_ok());
    assert_eq!(mempool.len(), 1);
}

#[test]
fn test_sign_then_verify_passes() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;

    let recipient_keypair = Keypair::generate(&mut csprng);

    let tx = Transaction::new(
        public_key,
        recipient_keypair.public,
        500,
        42,
    );

    let signature = sign(&tx, &keypair);
    let signed_tx = tx_rs::SignedTransaction::new(tx, signature);

    assert!(signed_tx.verify());
}

#[test]
fn test_modifying_amount_breaks_signature() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;

    let recipient_keypair = Keypair::generate(&mut csprng);

    let mut tx = Transaction::new(
        public_key,
        recipient_keypair.public,
        100,
        1,
    );

    let signature = sign(&tx, &keypair);

    // Modify the amount after signing
    tx.amount = 200;

    // Verification should fail
    assert!(!tx_rs::verify(&tx, &signature));
}

#[test]
fn test_modifying_nonce_breaks_signature() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;

    let recipient_keypair = Keypair::generate(&mut csprng);

    let mut tx = Transaction::new(
        public_key,
        recipient_keypair.public,
        100,
        1,
    );

    let signature = sign(&tx, &keypair);

    // Modify the nonce after signing
    tx.nonce = 2;

    // Verification should fail
    assert!(!tx_rs::verify(&tx, &signature));
}

#[test]
fn test_modifying_recipient_breaks_signature() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    let public_key = keypair.public;

    let recipient_keypair1 = Keypair::generate(&mut csprng);
    let recipient_keypair2 = Keypair::generate(&mut csprng);

    let mut tx = Transaction::new(
        public_key,
        recipient_keypair1.public,
        100,
        1,
    );

    let signature = sign(&tx, &keypair);

    // Modify the recipient after signing
    tx.to_pubkey = recipient_keypair2.public.into();

    // Verification should fail
    assert!(!tx_rs::verify(&tx, &signature));
}

#[test]
fn test_mempool_nonce_enforcement() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);
    // let secret_key = keypair.secret; // Not needed

    let recipient_keypair = Keypair::generate(&mut csprng);
    let mut mempool = Mempool::new();

    // Add transaction with nonce 1
    let tx1 = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        100,
        1,
    );
    let signature1 = sign(&tx1, &keypair);
    let signed_tx1 = tx_rs::SignedTransaction::new(tx1, signature1);
    assert!(mempool.add_transaction(signed_tx1).is_ok());

    // Try to add transaction with nonce 0 (should fail)
    let tx0 = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        50,
        0,
    );
    let signature0 = sign(&tx0, &keypair);
    let signed_tx0 = tx_rs::SignedTransaction::new(tx0, signature0);
    assert!(mempool.add_transaction(signed_tx0).is_err());

    // Add transaction with nonce 2 (should succeed)
    let tx2 = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        200,
        2,
    );
    let signature2 = sign(&tx2, &keypair);
    let signed_tx2 = tx_rs::SignedTransaction::new(tx2, signature2);
    assert!(mempool.add_transaction(signed_tx2).is_ok());

    assert_eq!(mempool.len(), 2);
}

#[test]
fn test_mempool_deduplication() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);

    let recipient_keypair = Keypair::generate(&mut csprng);
    let mut mempool = Mempool::new();

    // Create identical transactions
    let tx = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        100,
        1,
    );

    // Sign the same transaction twice (will produce same signature)
    let signature1 = sign(&tx, &keypair);
    let signature2 = sign(&tx, &keypair);

    let signed_tx1 = tx_rs::SignedTransaction::new(tx.clone(), signature1);
    let signed_tx2 = tx_rs::SignedTransaction::new(tx, signature2);

    // First should succeed, second should fail
    assert!(mempool.add_transaction(signed_tx1).is_ok());
    assert!(mempool.add_transaction(signed_tx2).is_err());

    assert_eq!(mempool.len(), 1);
}

#[test]
fn test_transaction_id_consistency() {
    let mut csprng = OsRng;
    let keypair = Keypair::generate(&mut csprng);

    let recipient_keypair = Keypair::generate(&mut csprng);

    let tx = Transaction::new(
        keypair.public,
        recipient_keypair.public,
        100,
        1,
    );

    // TxId should be consistent across multiple computations
    let id1 = tx.compute_id();
    let id2 = tx.compute_id();
    let id3 = tx_rs::TxId::from_tx(&tx);

    assert_eq!(id1, id2);
    assert_eq!(id2, id3);
}

#[test]
fn test_multi_user_mempool() {
    let mut csprng = OsRng;
    let alice_keypair = Keypair::generate(&mut csprng);
    let bob_keypair = Keypair::generate(&mut csprng);
    let charlie_keypair = Keypair::generate(&mut csprng);

    let mut mempool = Mempool::new();

    // Alice -> Bob
    let tx_ab = Transaction::new(
        alice_keypair.public,
        bob_keypair.public,
        100,
        1,
    );
    let sig_ab = sign(&tx_ab, &alice_keypair);
    let signed_tx_ab = tx_rs::SignedTransaction::new(tx_ab, sig_ab);
    assert!(mempool.add_transaction(signed_tx_ab).is_ok());

    // Bob -> Charlie
    let tx_bc = Transaction::new(
        bob_keypair.public,
        charlie_keypair.public,
        50,
        1,
    );
    let sig_bc = sign(&tx_bc, &bob_keypair);
    let signed_tx_bc = tx_rs::SignedTransaction::new(tx_bc, sig_bc);
    assert!(mempool.add_transaction(signed_tx_bc).is_ok());

    // Charlie -> Alice
    let tx_ca = Transaction::new(
        charlie_keypair.public,
        alice_keypair.public,
        25,
        1,
    );
    let sig_ca = sign(&tx_ca, &charlie_keypair);
    let signed_tx_ca = tx_rs::SignedTransaction::new(tx_ca, sig_ca);
    assert!(mempool.add_transaction(signed_tx_ca).is_ok());

    assert_eq!(mempool.len(), 3);

    // Verify each user's nonce is tracked correctly
    assert_eq!(mempool.get_account_nonce(&alice_keypair.public), 1);
    assert_eq!(mempool.get_account_nonce(&bob_keypair.public), 1);
    assert_eq!(mempool.get_account_nonce(&charlie_keypair.public), 1);
}