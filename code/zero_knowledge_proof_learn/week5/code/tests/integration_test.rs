use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use toychain_rs::{apply_block, block_hash, Account, Block, Blockchain, State};
use tx_rs::{sign, SignedTransaction, Transaction};

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
    let tx1 = Transaction::new(alice_key.public, bob_key.public, 30, 0);
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
    let tx2a = Transaction::new(bob_key.public, charlie_key.public, 20, 0);
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

    let block2 = Block::new(block1_hash, vec![signed_tx2a, signed_tx2b], 2, 1234567900);

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

#[test]
fn test_fork_resolution() {
    // === Setup ===
    let mut blockchain = Blockchain::new();
    let mut state = State::new();

    let alice_key = Keypair::generate(&mut OsRng);
    let bob_key = Keypair::generate(&mut OsRng);

    state.set_account(alice_key.public, Account::new(100, 0));
    state.set_account(bob_key.public, Account::new(50, 0));

    // === Genesis Block ===
    let genesis = Block::new([0u8; 32], vec![], 0, 1000);
    let genesis_hash = genesis.hash();
    blockchain.add_block(genesis).unwrap();

    println!("=== Genesis ===");
    println!("Hash: {}", hex::encode(genesis_hash));
    println!("Tip: {:?}", blockchain.get_tip().map(|h| hex::encode(h)));

    // === Fork: Two competing blocks at height 1 ===

    // Fork A: Alice sends 20 to Bob
    let tx_a = Transaction::new(alice_key.public, bob_key.public, 20, 0);
    let sig_a = sign(&tx_a, &alice_key);
    let signed_tx_a = SignedTransaction::new(tx_a, sig_a);

    let block1a = Block::new(genesis_hash, vec![signed_tx_a], 1, 2000);
    let hash1a = block1a.hash();
    blockchain.add_block(block1a.clone()).unwrap();

    println!("\n=== Fork A (height 1) ===");
    println!("Hash: {}", hex::encode(hash1a));
    println!("Tip: {:?}", blockchain.get_tip().map(|h| hex::encode(h)));
    assert_eq!(blockchain.get_tip(), Some(&hash1a));

    // Fork B: Alice sends 30 to Bob
    let tx_b = Transaction::new(alice_key.public, bob_key.public, 30, 0);
    let sig_b = sign(&tx_b, &alice_key);
    let signed_tx_b = SignedTransaction::new(tx_b, sig_b);

    let block1b = Block::new(genesis_hash, vec![signed_tx_b], 1, 2001);
    let hash1b = block1b.hash();
    blockchain.add_block(block1b.clone()).unwrap();

    println!("\n=== Fork B (height 1) ===");
    println!("Hash: {}", hex::encode(hash1b));
    println!("Tip: {:?}", blockchain.get_tip().map(|h| hex::encode(h)));
    // Tip should still be hash1a (first one wins on tie)
    assert_eq!(blockchain.get_tip(), Some(&hash1a));

    // Both blocks exist in storage
    assert!(blockchain.get_block(&hash1a).is_some());
    assert!(blockchain.get_block(&hash1b).is_some());
    assert_eq!(blockchain.len(), 3); // genesis + 1a + 1b

    // === Extend Fork A to height 2 ===
    let tx2 = Transaction::new(alice_key.public, bob_key.public, 10, 1);
    let sig2 = sign(&tx2, &alice_key);
    let signed_tx2 = SignedTransaction::new(tx2, sig2);

    let block2a = Block::new(hash1a, vec![signed_tx2], 2, 3000);
    let hash2a = block2a.hash();
    blockchain.add_block(block2a).unwrap();

    println!("\n=== Fork A Extended (height 2) ===");
    println!("Hash: {}", hex::encode(hash2a));
    println!("Tip: {:?}", blockchain.get_tip().map(|h| hex::encode(h)));
    // Tip should now be hash2a (height 2 > height 1)
    assert_eq!(blockchain.get_tip(), Some(&hash2a));

    // === Get canonical chain ===
    let chain = blockchain.get_canonical_chain();
    println!("\n=== Canonical Chain ===");
    for block in &chain {
        println!("Height {}: {}", block.height, hex::encode(block.hash()));
    }

    assert_eq!(chain.len(), 3); // genesis + 1a + 2a
    assert_eq!(chain[0].height, 0);
    assert_eq!(chain[1].height, 1);
    assert_eq!(chain[2].height, 2);

    // Verify canonical chain is genesis -> 1a -> 2a
    assert_eq!(chain[0].hash(), genesis_hash);
    assert_eq!(chain[1].hash(), hash1a);
    assert_eq!(chain[2].hash(), hash2a);

    println!("\n=== Fork Resolution Test Passed! ===");
}

#[test]
fn test_chain_reorg_on_longer_fork() {
    let mut blockchain = Blockchain::new();

    // Build initial chain: genesis -> 1a -> 2a (height 2)
    let genesis = Block::new([0u8; 32], vec![], 0, 1000);
    let genesis_hash = genesis.hash();
    blockchain.add_block(genesis).unwrap();

    let block1a = Block::new(genesis_hash, vec![], 1, 2000);
    let hash1a = block1a.hash();
    blockchain.add_block(block1a).unwrap();

    let block2a = Block::new(hash1a, vec![], 2, 3000);
    let hash2a = block2a.hash();
    blockchain.add_block(block2a).unwrap();

    assert_eq!(blockchain.get_tip(), Some(&hash2a));

    // Create competing fork: genesis -> 1b -> 2b -> 3b (height 3)
    let block1b = Block::new(genesis_hash, vec![], 1, 2001);
    let hash1b = block1b.hash();
    blockchain.add_block(block1b).unwrap();

    let block2b = Block::new(hash1b, vec![], 2, 3001);
    let hash2b = block2b.hash();
    blockchain.add_block(block2b).unwrap();

    // Tip should still be hash2a (heights tie, first wins)
    assert_eq!(blockchain.get_tip(), Some(&hash2a));

    let block3b = Block::new(hash2b, vec![], 3, 4000);
    let hash3b = block3b.hash();
    blockchain.add_block(block3b).unwrap();

    // Tip should now be hash3b (height 3 > height 2) - REORG!
    assert_eq!(blockchain.get_tip(), Some(&hash3b));

    // Verify canonical chain is the new longer fork
    let chain = blockchain.get_canonical_chain();
    assert_eq!(chain.len(), 4); // genesis + 1b + 2b + 3b
    assert_eq!(chain[0].hash(), genesis_hash);
    assert_eq!(chain[1].hash(), hash1b);
    assert_eq!(chain[2].hash(), hash2b);
    assert_eq!(chain[3].hash(), hash3b);

    println!("=== Chain Reorg Test Passed! ===");
}
