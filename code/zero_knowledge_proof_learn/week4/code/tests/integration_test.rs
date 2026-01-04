use toychain_rs::{State, Account, Block, block_hash, apply_block};
use tx_rs::{Transaction, SignedTransaction, sign};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[test]
fn test_end_to_end_blockchain_workflow() {
    // === Setup: Create keys for 3 users ===
    let mut csprng = OsRng;
    let alice_key = Keypair::generate(&mut csprng);
    let bob_key = Keypair::generate(&mut csprng);
    let charlie_key = Keypair::generate(&mut csprng);

    // === Genesis: Initial balances ===
    let mut state = State::new();
    state.set_account(alice_key.public, Account::new(100, 0));
    state.set_account(bob_key.public, Account::new(50, 0));
    state.set_account(charlie_key.public, Account::new(75, 0));

    println!("=== Genesis State ===");
    println!("Alice: {:?}", state.get_account(&alice_key.public));
    println!("Bob: {:?}", state.get_account(&bob_key.public));
    println!("Charlie: {:?}", state.get_account(&charlie_key.public));

    // === Block 1: Alice sends 30 to Bob ===
    let tx1 = Transaction::new(
        alice_key.public,
        bob_key.public,
        30,
        0,
    );
    let sig1 = sign(&tx1, &alice_key);
    let signed_tx1 = SignedTransaction::new(tx1, sig1);

    let block1 = Block::new(
        [0u8; 32], // Genesis prev_hash
        vec![signed_tx1],
        1,
        1234567890,
    );

    let block1_hash = block_hash(&block1);
    println!("\n=== Block 1 ===");
    println!("Hash: {}", hex::encode(block1_hash));

    apply_block(&mut state, &block1).unwrap();

    println!("Alice: {:?}", state.get_account(&alice_key.public));
    println!("Bob: {:?}", state.get_account(&bob_key.public));

    // Verify state
    assert_eq!(state.get_account(&alice_key.public).unwrap().balance, 70);
    assert_eq!(state.get_account(&alice_key.public).unwrap().nonce, 1);
    assert_eq!(state.get_account(&bob_key.public).unwrap().balance, 80);

    // === Block 2: Bob sends 20 to Charlie, Alice sends 10 to Charlie ===
    let tx2a = Transaction::new(
        bob_key.public,
        charlie_key.public,
        20,
        0,
    );
    let sig2a = sign(&tx2a, &bob_key);
    let signed_tx2a = SignedTransaction::new(tx2a, sig2a);

    let tx2b = Transaction::new(
        alice_key.public,
        charlie_key.public,
        10,
        1, // Alice's second tx
    );
    let sig2b = sign(&tx2b, &alice_key);
    let signed_tx2b = SignedTransaction::new(tx2b, sig2b);

    let block2 = Block::new(
        block1_hash,
        vec![signed_tx2a, signed_tx2b],
        2,
        1234567900,
    );

    let block2_hash = block_hash(&block2);
    println!("\n=== Block 2 ===");
    println!("Hash: {}", hex::encode(block2_hash));
    println!("Prev: {}", hex::encode(block2.prev_hash));

    apply_block(&mut state, &block2).unwrap();

    println!("Alice: {:?}", state.get_account(&alice_key.public));
    println!("Bob: {:?}", state.get_account(&bob_key.public));
    println!("Charlie: {:?}", state.get_account(&charlie_key.public));

    // Verify final state
    assert_eq!(state.get_account(&alice_key.public).unwrap().balance, 60);
    assert_eq!(state.get_account(&alice_key.public).unwrap().nonce, 2);
    assert_eq!(state.get_account(&bob_key.public).unwrap().balance, 60);
    assert_eq!(state.get_account(&bob_key.public).unwrap().nonce, 1);
    assert_eq!(state.get_account(&charlie_key.public).unwrap().balance, 105);
    assert_eq!(state.get_account(&charlie_key.public).unwrap().nonce, 0);

    println!("\n=== Integration Test Passed! ===");
}
