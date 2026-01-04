# ToyChain STF Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a minimal blockchain state transition function in Rust that validates transactions and applies them to account state, with block structure and chain validation.

**Architecture:** Account-based state model using HashMap storing balance and nonce per public key. Transactions from Week 3 (tx-rs) are validated (signature, balance, nonce) then applied atomically. Blocks contain ordered transactions with metadata and are applied sequentially.

**Tech Stack:** Rust 2021, ed25519-dalek (signatures), serde (serialization), sha2 (hashing), anyhow (errors). Reuses tx-rs crate from Week 3 as dependency.

---

## Task 1: Create Project Structure and Dependencies

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `README.md`

**Step 1: Initialize Cargo project**

Run: `cargo init --lib --name toychain`

**Step 2: Update Cargo.toml with dependencies**

```toml
[package]
name = "toychain-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
tx-rs = { path = "../../week3/code" }
ed25519-dalek = { version = "1.0", features = ["serde"] }
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
hex = "0.4"
anyhow = "1.0"

[dev-dependencies]
rand = "0.7"
```

**Step 3: Create basic lib.rs structure**

```rust
pub mod state;
pub mod block;
pub mod chain;

pub use state::{State, Account};
pub use block::{Block, block_hash};
pub use chain::apply_block;
```

**Step 4: Create placeholder modules**

Create: `src/state.rs` with:
```rust
// State module placeholder
```

Create: `src/block.rs` with:
```rust
// Block module placeholder
```

Create: `src/chain.rs` with:
```rust
// Chain module placeholder
```

**Step 5: Verify project builds**

Run: `cargo build`
Expected: SUCCESS with warnings about unused code

**Step 6: Commit**

```bash
git add Cargo.toml src/lib.rs src/state.rs src/block.rs src/chain.rs
git commit -m "feat: initialize toychain-rs project with dependencies"
```

---

## Task 2: Implement Account Type

**Files:**
- Modify: `src/state.rs`

**Step 1: Write test for Account creation and serialization**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_account_creation() {
        let account = Account::new(100, 0);
        assert_eq!(account.balance, 100);
        assert_eq!(account.nonce, 0);
    }

    #[test]
    fn test_account_serialization() {
        let account = Account::new(100, 5);
        let json = serde_json::to_string(&account).unwrap();
        assert_eq!(json, r#"{"balance":100,"nonce":5}"#);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_account`
Expected: COMPILE ERROR - Account not defined

**Step 3: Implement Account struct**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

impl Account {
    pub fn new(balance: u64, nonce: u64) -> Self {
        Self { balance, nonce }
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_account`
Expected: PASS (2 tests)

**Step 5: Commit**

```bash
git add src/state.rs
git commit -m "feat: add Account type with serialization support"
```

---

## Task 3: Implement State Structure

**Files:**
- Modify: `src/state.rs`

**Step 1: Write test for State operations**

```rust
#[test]
fn test_state_get_and_set() {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let mut state = State::new();
    let key = SigningKey::generate(&mut OsRng);
    let pubkey = key.public_key();

    // Get non-existent account
    let account = state.get_account(&pubkey);
    assert_eq!(account, None);

    // Set account
    state.set_account(pubkey, Account::new(100, 0));

    // Get account
    let account = state.get_account(&pubkey);
    assert_eq!(account, Some(&Account::new(100, 0)));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_state_get_and_set`
Expected: COMPILE ERROR - State not defined

**Step 3: Implement State struct**

```rust
use std::collections::HashMap;
use ed25519_dalek::PublicKey;

#[derive(Debug, Clone)]
pub struct State {
    accounts: HashMap<PublicKey, Account>,
}

impl State {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn get_account(&self, pubkey: &PublicKey) -> Option<&Account> {
        self.accounts.get(pubkey)
    }

    pub fn set_account(&mut self, pubkey: PublicKey, account: Account) {
        self.accounts.insert(pubkey, account);
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_state_get_and_set`
Expected: PASS

**Step 5: Commit**

```bash
git add src/state.rs
git commit -m "feat: add State with account storage"
```

---

## Task 4: Implement Transaction Validation - Signature Check

**Files:**
- Modify: `src/state.rs`

**Step 1: Write test for signature validation**

```rust
#[test]
fn test_apply_tx_invalid_signature() {
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let mut state = State::new();
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);

    // Fund Alice
    state.set_account(alice_key.public_key(), Account::new(100, 0));

    // Create transaction from Alice to Bob
    let tx = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
        50,
        0,
    );

    // Sign with WRONG key (Bob signs instead of Alice)
    let signature = sign(&tx, &bob_key);
    let signed_tx = SignedTransaction::new(tx, signature);

    // Try to apply - should fail
    let result = state.apply_tx(&signed_tx);
    assert!(result.is_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_apply_tx_invalid_signature`
Expected: COMPILE ERROR - apply_tx not defined

**Step 3: Implement signature validation in apply_tx**

```rust
use anyhow::Result;

impl State {
    // ... existing methods ...

    pub fn apply_tx(&mut self, signed_tx: &SignedTransaction) -> Result<()> {
        // Verify signature
        if !signed_tx.verify() {
            return Err(anyhow::anyhow!("Invalid signature"));
        }

        Ok(())
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_apply_tx_invalid_signature`
Expected: PASS

**Step 5: Commit**

```bash
git add src/state.rs
git commit -m "feat: add signature validation to apply_tx"
```

---

## Task 5: Implement Transaction Validation - Balance Check

**Files:**
- Modify: `src/state.rs`

**Step 1: Write test for insufficient balance**

```rust
#[test]
fn test_apply_tx_insufficient_balance() {
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let mut state = State::new();
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);

    // Fund Alice with only 10 tokens
    state.set_account(alice_key.public_key(), Account::new(10, 0));

    // Try to send 50 tokens
    let tx = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
        50,
        0,
    );

    let signature = sign(&tx, &alice_key);
    let signed_tx = SignedTransaction::new(tx, signature);

    // Should fail - insufficient balance
    let result = state.apply_tx(&signed_tx);
    assert!(result.is_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_apply_tx_insufficient_balance`
Expected: FAIL - balance check not implemented

**Step 3: Add balance check to apply_tx**

```rust
pub fn apply_tx(&mut self, signed_tx: &SignedTransaction) -> Result<()> {
    // Verify signature
    if !signed_tx.verify() {
        return Err(anyhow::anyhow!("Invalid signature"));
    }

    let tx = &signed_tx.tx;

    // Check sender has sufficient balance
    let sender_account = self.get_account(&tx.from_pubkey)
        .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;

    if sender_account.balance < tx.amount {
        return Err(anyhow::anyhow!("Insufficient balance"));
    }

    Ok(())
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_apply_tx_insufficient_balance`
Expected: PASS

**Step 5: Commit**

```bash
git add src/state.rs
git commit -m "feat: add balance validation to apply_tx"
```

---

## Task 6: Implement Transaction Validation - Nonce Check

**Files:**
- Modify: `src/state.rs`

**Step 1: Write test for incorrect nonce**

```rust
#[test]
fn test_apply_tx_incorrect_nonce() {
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let mut state = State::new();
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);

    // Fund Alice with nonce = 5
    state.set_account(alice_key.public_key(), Account::new(100, 5));

    // Try to send with nonce = 3 (wrong, should be 5)
    let tx = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
        50,
        3, // Wrong nonce
    );

    let signature = sign(&tx, &alice_key);
    let signed_tx = SignedTransaction::new(tx, signature);

    // Should fail - incorrect nonce
    let result = state.apply_tx(&signed_tx);
    assert!(result.is_err());
}

#[test]
fn test_apply_tx_correct_nonce() {
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let mut state = State::new();
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);

    // Fund Alice with nonce = 5
    state.set_account(alice_key.public_key(), Account::new(100, 5));

    // Send with correct nonce = 5
    let tx = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
        50,
        5, // Correct nonce
    );

    let signature = sign(&tx, &alice_key);
    let signed_tx = SignedTransaction::new(tx, signature);

    // Should pass validation (won't update state yet)
    let result = state.apply_tx(&signed_tx);
    assert!(result.is_ok());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_apply_tx_incorrect_nonce`
Expected: FAIL - nonce check not implemented

**Step 3: Add nonce check to apply_tx**

```rust
pub fn apply_tx(&mut self, signed_tx: &SignedTransaction) -> Result<()> {
    // Verify signature
    if !signed_tx.verify() {
        return Err(anyhow::anyhow!("Invalid signature"));
    }

    let tx = &signed_tx.tx;

    // Check sender exists
    let sender_account = self.get_account(&tx.from_pubkey)
        .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;

    // Check sufficient balance
    if sender_account.balance < tx.amount {
        return Err(anyhow::anyhow!("Insufficient balance"));
    }

    // Check nonce matches
    if sender_account.nonce != tx.nonce {
        return Err(anyhow::anyhow!("Invalid nonce: expected {}, got {}",
            sender_account.nonce, tx.nonce));
    }

    Ok(())
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_apply_tx_nonce`
Expected: PASS (both tests)

**Step 5: Commit**

```bash
git add src/state.rs
git commit -m "feat: add nonce validation to apply_tx"
```

---

## Task 7: Implement State Updates

**Files:**
- Modify: `src/state.rs`

**Step 1: Write test for state mutation**

```rust
#[test]
fn test_apply_tx_updates_state() {
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let mut state = State::new();
    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);

    // Setup: Alice has 100, Bob has 50
    state.set_account(alice_key.public_key(), Account::new(100, 0));
    state.set_account(bob_key.public_key(), Account::new(50, 0));

    // Alice sends 30 to Bob
    let tx = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
        30,
        0,
    );

    let signature = sign(&tx, &alice_key);
    let signed_tx = SignedTransaction::new(tx, signature);

    // Apply transaction
    state.apply_tx(&signed_tx).unwrap();

    // Verify: Alice has 70, nonce 1
    let alice_account = state.get_account(&alice_key.public_key()).unwrap();
    assert_eq!(alice_account.balance, 70);
    assert_eq!(alice_account.nonce, 1);

    // Verify: Bob has 80, nonce 0
    let bob_account = state.get_account(&bob_key.public_key()).unwrap();
    assert_eq!(bob_account.balance, 80);
    assert_eq!(bob_account.nonce, 0);

    // Verify: Can't replay same transaction
    let result = state.apply_tx(&signed_tx);
    assert!(result.is_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_apply_tx_updates_state`
Expected: FAIL - state not being updated

**Step 3: Implement state updates in apply_tx**

```rust
pub fn apply_tx(&mut self, signed_tx: &SignedTransaction) -> Result<()> {
    // Verify signature
    if !signed_tx.verify() {
        return Err(anyhow::anyhow!("Invalid signature"));
    }

    let tx = &signed_tx.tx;

    // Check sender exists
    let sender_account = self.get_account(&tx.from_pubkey)
        .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;

    // Check sufficient balance
    if sender_account.balance < tx.amount {
        return Err(anyhow::anyhow!("Insufficient balance"));
    }

    // Check nonce matches
    if sender_account.nonce != tx.nonce {
        return Err(anyhow::anyhow!("Invalid nonce: expected {}, got {}",
            sender_account.nonce, tx.nonce));
    }

    // === Update state ===

    // Deduct from sender and increment nonce
    let sender_account = self.accounts.get_mut(&tx.from_pubkey).unwrap();
    sender_account.balance -= tx.amount;
    sender_account.nonce += 1;

    // Add to recipient (create account if needed)
    let recipient_account = self.accounts
        .entry(tx.to_pubkey)
        .or_insert_with(|| Account::new(0, 0));
    recipient_account.balance += tx.amount;

    Ok(())
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_apply_tx_updates_state`
Expected: PASS

**Step 5: Commit**

```bash
git add src/state.rs
git commit -m "feat: implement state updates in apply_tx"
```

---

## Task 8: Implement Block Structure

**Files:**
- Modify: `src/block.rs`

**Step 1: Write test for Block creation**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_block_creation() {
        let key = SigningKey::generate(&mut OsRng);
        let pubkey = key.public_key();

        let block = Block::new(
            [0u8; 32], // prev_hash
            vec![],     // empty transactions
            1,         // height
            1234567890, // timestamp
        );

        assert_eq!(block.prev_hash, [0u8; 32]);
        assert_eq!(block.txs.len(), 0);
        assert_eq!(block.height, 1);
        assert_eq!(block.timestamp, 1234567890);
    }

    #[test]
    fn test_block_serialization() {
        let block = Block::new([0u8; 32], vec![], 1, 1234567890);
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"prev_hash\""));
        assert!(json.contains("\"height\":1"));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_block`
Expected: COMPILE ERROR - Block not defined

**Step 3: Implement Block struct**

```rust
use serde::{Deserialize, Serialize};
use tx_rs::SignedTransaction;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub prev_hash: [u8; 32],
    pub txs: Vec<SignedTransaction>,
    pub height: u64,
    pub timestamp: u64,
}

impl Block {
    pub fn new(prev_hash: [u8; 32], txs: Vec<SignedTransaction>, height: u64, timestamp: u64) -> Self {
        Self {
            prev_hash,
            txs,
            height,
            timestamp,
        }
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_block`
Expected: PASS (2 tests)

**Step 5: Commit**

```bash
git add src/block.rs
git commit -m "feat: add Block struct with serialization"
```

---

## Task 9: Implement Block Hashing

**Files:**
- Modify: `src/block.rs`

**Step 1: Write test for block_hash**

```rust
#[test]
fn test_block_hash() {
    use sha2::{Sha256, Digest};

    let block = Block::new([0u8; 32], vec![], 1, 1234567890);

    let hash1 = block_hash(&block);
    let hash2 = block_hash(&block);

    // Same block should produce same hash
    assert_eq!(hash1, hash2);

    // Different block should produce different hash
    let block2 = Block::new([1u8; 32], vec![], 1, 1234567890);
    let hash3 = block_hash(&block2);
    assert_ne!(hash1, hash3);

    // Hash should be 32 bytes
    assert_eq!(hash1.len(), 32);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_block_hash`
Expected: COMPILE ERROR - block_hash not defined

**Step 3: Implement block_hash function**

```rust
use sha2::{Sha256, Digest};

pub fn block_hash(block: &Block) -> [u8; 32] {
    let serialized = serde_json::to_vec(block).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    let result = hasher.finalize();

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_block_hash`
Expected: PASS

**Step 5: Commit**

```bash
git add src/block.rs
git commit -m "feat: add block_hash function"
```

---

## Task 10: Implement Block Application

**Files:**
- Modify: `src/chain.rs`

**Step 1: Write test for apply_block**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{State, Account, Block};
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_apply_block_with_valid_txs() {
        let mut state = State::new();
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        // Setup accounts
        state.set_account(alice_key.public_key(), Account::new(100, 0));
        state.set_account(bob_key.public_key(), Account::new(50, 0));

        // Create transactions
        let tx1 = Transaction::new(
            alice_key.public_key(),
            bob_key.public_key(),
            30,
            0,
        );
        let sig1 = sign(&tx1, &alice_key);
        let signed_tx1 = SignedTransaction::new(tx1, sig1);

        // Create block
        let block = Block::new(
            [0u8; 32],
            vec![signed_tx1],
            1,
            1234567890,
        );

        // Apply block
        apply_block(&mut state, &block).unwrap();

        // Verify state
        let alice_account = state.get_account(&alice_key.public_key()).unwrap();
        assert_eq!(alice_account.balance, 70);
        assert_eq!(alice_account.nonce, 1);

        let bob_account = state.get_account(&bob_key.public_key()).unwrap();
        assert_eq!(bob_account.balance, 80);
    }

    #[test]
    fn test_apply_block_with_invalid_tx() {
        let mut state = State::new();
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        // Alice has insufficient balance
        state.set_account(alice_key.public_key(), Account::new(10, 0));

        // Try to send 100
        let tx = Transaction::new(
            alice_key.public_key(),
            bob_key.public_key(),
            100,
            0,
        );
        let sig = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, sig);

        let block = Block::new(
            [0u8; 32],
            vec![signed_tx],
            1,
            1234567890,
        );

        // Should fail
        let result = apply_block(&mut state, &block);
        assert!(result.is_err());

        // State should be unchanged
        let alice_account = state.get_account(&alice_key.public_key()).unwrap();
        assert_eq!(alice_account.balance, 10);
        assert_eq!(alice_account.nonce, 0);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_apply_block`
Expected: COMPILE ERROR - apply_block not defined

**Step 3: Implement apply_block function**

```rust
use anyhow::Result;
use crate::{State, Block};

pub fn apply_block(state: &mut State, block: &Block) -> Result<()> {
    for signed_tx in &block.txs {
        state.apply_tx(signed_tx)?;
    }
    Ok(())
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_apply_block`
Expected: PASS (2 tests)

**Step 5: Commit**

```bash
git add src/chain.rs
git commit -m "feat: add apply_block function"
```

---

## Task 11: End-to-End Integration Test

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Write comprehensive integration test**

```rust
use toychain_rs::{State, Account, Block, block_hash, apply_block};
use tx_rs::{Transaction, SignedTransaction, sign};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[test]
fn test_end_to_end_blockchain_workflow() {
    // === Setup: Create keys for 3 users ===
    let mut csprng = OsRng;
    let alice_key = SigningKey::generate(&mut csprng);
    let bob_key = SigningKey::generate(&mut csprng);
    let charlie_key = SigningKey::generate(&mut csprng);

    // === Genesis: Initial balances ===
    let mut state = State::new();
    state.set_account(alice_key.public_key(), Account::new(100, 0));
    state.set_account(bob_key.public_key(), Account::new(50, 0));
    state.set_account(charlie_key.public_key(), Account::new(75, 0));

    println!("=== Genesis State ===");
    println!("Alice: {:?}", state.get_account(&alice_key.public_key()));
    println!("Bob: {:?}", state.get_account(&bob_key.public_key()));
    println!("Charlie: {:?}", state.get_account(&charlie_key.public_key()));

    // === Block 1: Alice sends 30 to Bob ===
    let tx1 = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
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

    println!("Alice: {:?}", state.get_account(&alice_key.public_key()));
    println!("Bob: {:?}", state.get_account(&bob_key.public_key()));

    // Verify state
    assert_eq!(state.get_account(&alice_key.public_key()).unwrap().balance, 70);
    assert_eq!(state.get_account(&alice_key.public_key()).unwrap().nonce, 1);
    assert_eq!(state.get_account(&bob_key.public_key()).unwrap().balance, 80);

    // === Block 2: Bob sends 20 to Charlie, Alice sends 10 to Charlie ===
    let tx2a = Transaction::new(
        bob_key.public_key(),
        charlie_key.public_key(),
        20,
        0,
    );
    let sig2a = sign(&tx2a, &bob_key);
    let signed_tx2a = SignedTransaction::new(tx2a, sig2a);

    let tx2b = Transaction::new(
        alice_key.public_key(),
        charlie_key.public_key(),
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

    println!("Alice: {:?}", state.get_account(&alice_key.public_key()));
    println!("Bob: {:?}", state.get_account(&bob_key.public_key()));
    println!("Charlie: {:?}", state.get_account(&charlie_key.public_key()));

    // Verify final state
    assert_eq!(state.get_account(&alice_key.public_key()).unwrap().balance, 60);
    assert_eq!(state.get_account(&alice_key.public_key()).unwrap().nonce, 2);
    assert_eq!(state.get_account(&bob_key.public_key()).unwrap().balance, 60);
    assert_eq!(state.get_account(&bob_key.public_key()).unwrap().nonce, 1);
    assert_eq!(state.get_account(&charlie_key.public_key()).unwrap().balance, 105);
    assert_eq!(state.get_account(&charlie_key.public_key()).unwrap().nonce, 0);

    println!("\n=== Integration Test Passed! ===");
}
```

**Step 2: Run test to verify it works**

Run: `cargo test test_end_to_end -- --nocapture`
Expected: PASS with printed output showing state progression

**Step 3: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: add comprehensive end-to-end integration test"
```

---

## Task 12: Write Documentation

**Files:**
- Modify: `README.md`

**Step 1: Create comprehensive README**

```markdown
# toychain-rs: Minimal Blockchain State Transition Function

A clean, simple Rust implementation of a blockchain's core state transition logic. This project demonstrates how blocks containing transactions are validated and applied to account state.

## 🎯 Learning Objectives

After studying this code, you'll understand:

- **State Transition Function (STF)**: How blockchain state changes deterministically
- **Account-Based Model**: How Ethereum-style accounts work (balance + nonce)
- **Transaction Validation**: Signature verification, balance checks, nonce validation
- **Block Structure**: How blocks chain together via hashes
- **Atomic Updates**: How all transactions in a block succeed or fail together

## 📋 Requirements Met

✅ Week 4 of ZK Learning Plan:
- `State` with `HashMap<PubKey, Account { balance, nonce }>`
- `apply_tx(state, signed_tx) -> Result<()>`
- `Block { prev_hash, txs, height, timestamp }`
- `apply_block(state, block) -> Result<()>`
- `block_hash(block) -> Hash32`
- End-to-end test with genesis, keys, signed txs, blocks

## 🚀 Quick Start

```rust
use toychain_rs::{State, Account, Block, block_hash, apply_block};
use tx_rs::{Transaction, SignedTransaction, sign};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut csprng = OsRng;

    // Create accounts
    let mut state = State::new();
    let alice = SigningKey::generate(&mut csprng);
    let bob = SigningKey::generate(&mut csprng);

    // Genesis: fund Alice
    state.set_account(alice.public_key(), Account::new(100, 0));

    // Create transaction
    let tx = Transaction::new(alice.public_key(), bob.public_key(), 30, 0);
    let sig = sign(&tx, &alice);
    let signed_tx = SignedTransaction::new(tx, sig);

    // Create block
    let block = Block::new([0u8; 32], vec![signed_tx], 1, 1234567890);

    // Apply block
    apply_block(&mut state, &block)?;

    Ok(())
}
```

## 🧮 Key Concepts

### Account-Based State

Unlike UTXO-based systems (like Bitcoin), this uses an account model similar to Ethereum:

- **Account**: Identified by public key, stores balance and nonce
- **Balance**: Amount of tokens owned
- **Nonce**: Transaction counter, prevents replay attacks

### Transaction Validation Rules

Every transaction must pass ALL checks:

1. **Signature Valid**: Proved ownership of private key
2. **Sufficient Balance**: Sender has enough tokens
3. **Correct Nonce**: Matches account's current nonce

If any check fails, the transaction is rejected.

### Block Application

Blocks contain ordered transactions. All transactions must validate for the block to be applied. If any transaction fails, the entire block fails and no state changes occur.

## 🏗️ Architecture

```
Transaction (from Week 3)
    ↓
SignedTransaction (tx + signature)
    ↓ apply_tx() validation
State.update() [atomic]
    ↓
Block { prev_hash, txs, height, timestamp }
    ↓ apply_block()
State [updated with all txs]
```

## 📁 Project Structure

```
src/
├── lib.rs       # Public API exports
├── state.rs     # State, Account, apply_tx
├── block.rs     # Block, block_hash
└── chain.rs     # apply_block

tests/
└── integration_test.rs  # End-to-end workflow
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run integration test with output
cargo test test_end_to_end -- --nocapture

# Run specific test
cargo test test_apply_tx_insufficient_balance
```

## 🔗 Dependencies

- `tx-rs`: Transaction types from Week 3
- `ed25519-dalek`: Digital signatures
- `sha2`: SHA-256 hashing
- `serde`: Serialization
- `anyhow`: Error handling

## 📖 Next Steps

This STF is the foundation for:
- **Week 5**: Forks and consensus
- **Week 6-12**: ZK proofs that STF was applied correctly

## 📄 License

MIT
```

**Step 2: Update lib.rs documentation**

```rust
//! # toychain-rs
//!
//! A minimal blockchain state transition function implementation.
//!
//! ## Overview
//!
//! This library provides the core state management and transaction validation
//! logic for a simple account-based blockchain. It demonstrates how transactions
//! are validated (signature, balance, nonce) and applied to state atomically.
//!
//! ## Key Types
//!
//! - [`State`]: Manages account storage and transaction application
//! - [`Account`]: Represents a user's balance and nonce
//! - [`Block`]: Contains ordered transactions with metadata
//!
//! ## Example
//!
//! ```rust
//! use toychain_rs::{State, Account, Block, apply_block};
//! use tx_rs::{Transaction, SignedTransaction, sign};
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut state = State::new();
//! let alice = SigningKey::generate(&mut OsRng);
//!
//! state.set_account(alice.public_key(), Account::new(100, 0));
//!
//! let tx = Transaction::new(alice.public_key(), alice.public_key(), 10, 0);
//! let sig = sign(&tx, &alice);
//! let signed_tx = SignedTransaction::new(tx, sig);
//!
//! let block = Block::new([0u8; 32], vec![signed_tx], 1, 0);
//! apply_block(&mut state, &block)?;
//! # Ok(())
//! # }
//! ```

pub mod state;
pub mod block;
pub mod chain;

pub use state::{State, Account};
pub use block::{Block, block_hash};
pub use chain::apply_block;
```

**Step 3: Commit**

```bash
git add README.md src/lib.rs
git commit -m "docs: add comprehensive documentation"
```

---

## Task 13: Run Full Test Suite and Verification

**Step 1: Run all tests**

Run: `cargo test`

Expected: All tests pass (15+ tests)

**Step 2: Run clippy**

Run: `cargo clippy`

Expected: No warnings (or fix any that appear)

**Step 3: Format code**

Run: `cargo fmt`

**Step 4: Build release**

Run: `cargo build --release`

Expected: Clean release build

**Step 5: Run integration test with output**

Run: `cargo test test_end_to_end -- --nocapture`

Expected: Pass with clear state progression output

**Step 6: Final commit**

```bash
git add .
git commit -m "test: verify all tests pass and code is clean"
```

---

## Success Criteria

When complete, you should have:

✅ Working `toychain-rs` crate with:
- Account-based state management
- Transaction validation (signature, balance, nonce)
- Block structure with hashing
- End-to-end workflow test

✅ All tests passing:
- Unit tests for each component
- Integration test showing full workflow
- Edge cases covered (invalid sigs, insufficient balance, wrong nonce)

✅ Clean code:
- No clippy warnings
- Formatted with `cargo fmt`
- Comprehensive documentation

✅ Ready for Week 5:
- Clean API for adding fork logic
- State is clonable/checkpointable
- Block validation is modular
