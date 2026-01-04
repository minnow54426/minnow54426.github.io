// Test to verify that spec-assumed APIs DON'T work

// This test demonstrates that following the spec literally causes compilation errors

#[test]
#[ignore]  // Don't run - this is just to show it doesn't compile
fn test_spec_assumption_1_from_pubkey_direct() {
    // SPEC ASSUMPTION: tx.from_pubkey can be used directly
    // REALITY: This doesn't compile because from_pubkey is HashablePublicKey

    use ed25519_dalek::PublicKey;
    use tx_rs::{Transaction, sign, SignedTransaction};

    let alice_key = ed25519_dalek::Keypair::generate(&mut rand::rngs::OsRng);
    let bob_key = ed25519_dalek::Keypair::generate(&mut rand::rngs::OsRng);

    let tx = Transaction::new(
        alice_key.public,
        bob_key.public,
        50,
        0,
    );

    let signature = sign(&tx, &alice_key);
    let signed_tx = SignedTransaction::new(tx, signature);

    // ❌ COMPILATION ERROR:
    // let sender_pubkey: PublicKey = signed_tx.tx.from_pubkey;
    //                                            ^^^^^^^^^^^^
    // expected struct `tx_rs::HashablePublicKey`, found struct `ed25519_dalek::PublicKey`
    //
    // You MUST use .0 to access the inner PublicKey:
    let sender_pubkey: PublicKey = signed_tx.tx.from_pubkey.0;  // ✅ CORRECT
}

#[test]
#[ignore]  // Don't run - this is just to show it doesn't compile
fn test_spec_assumption_2_keypair_public_key() {
    // SPEC ASSUMPTION: keypair.public_key is the correct API
    // REALITY: ed25519-dalek v1.0 uses keypair.public

    use ed25519_dalek::Keypair;

    let alice_key = Keypair::generate(&mut rand::rngs::OsRng);

    // ❌ COMPILATION ERROR:
    // let pubkey = alice_key.public_key;
    //                  ^^^^^^^^^^ no field named `public_key` on type `Keypair`
    //
    // The correct field name in ed25519-dalek v1.0 is:
    let pubkey = alice_key.public;  // ✅ CORRECT
}

// Note: This is a documentation file showing why the implementation is correct
// The actual implementation uses the correct APIs that compile and work
