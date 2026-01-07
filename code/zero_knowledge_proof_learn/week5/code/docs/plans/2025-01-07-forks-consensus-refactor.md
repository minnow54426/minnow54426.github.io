# Forks + Consensus + Refactor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor Week 4's toychain into a modular architecture with fork handling and longest-chain consensus.

**Architecture:**
- Split monolithic `toychain-rs` into clean modules: `core/types`, `core/state`, `core/tx`, `core/block`
- Add `Blockchain` struct that stores blocks in a `HashMap` by hash
- Implement fork-choice rule: select chain with highest cumulative height
- Support multiple competing chain tips

**Tech Stack:**
- Rust 2021 edition
- Existing dependencies: `tx-rs`, `ed25519-dalek`, `sha2`, `serde`, `anyhow`
- Data structures: `HashMap<[u8; 32], Block>` for block storage

---

## Pre-Implementation Checklist

**Understanding Week 4 Codebase:**
- **Existing files (Week 4):**
  - `src/lib.rs` - Public API exports
  - `src/state.rs` - Account/state management
  - `src/block.rs` - Block struct and hashing
  - `src/chain.rs` - `apply_block()` function
  - `tests/integration_test.rs` - End-to-end test
  - `Cargo.toml` - Dependencies

**What Week 4 Does:**
1. Manages account balances and nonces in `State`
2. Creates blocks with transactions
3. Applies blocks sequentially (linear chain)
4. No concept of forks or competing chains

**What Week 5 Will Add:**
1. Modular architecture (split into `core/` submodules)
2. Store ALL blocks (including forks) in a map
3. Track multiple chain tips
4. Implement "longest chain" fork-choice rule
5. Reorg capability (switch to better chain)

---

## Task 1: Create New Modular Directory Structure

**Files:**
- Create: `src/core/mod.rs`
- Create: `src/core/types.rs`
- Create: `src/core/state.rs`
- Create: `src/core/block.rs`
- Create: `src/core/tx.rs` (wrapper around tx-rs)
- Create: `src/core/chain.rs` (new Blockchain struct)
- Modify: `src/lib.rs` (update exports)
- Delete: `src/state.rs`, `src/block.rs`, `src/chain.rs` (move to core/)

**Step 1: Create core/mod.rs**

This file will declare all core submodules.

```rust
//! Core blockchain data structures and logic
//!
//! This module contains the fundamental types and operations for the
//! toy blockchain, including state, transactions, blocks, and chain management.

pub mod block;
pub mod chain;
pub mod state;
pub mod tx;
pub mod types;
```

Create file: `src/core/mod.rs`

**Step 2: Verify compiliation**

Run: `cargo check`
Expected: ERROR - modules don't exist yet (this is expected, we'll create them next)

**Step 3: Create core/types.rs**

This will hold common types used across modules.

```rust
//! Common types used throughout the blockchain

use serde::{Deserialize, Serialize};

/// A hash value (32 bytes)
pub type Hash = [u8; 32];

/// Block height (starts at 0 for genesis, 1 for first block, etc.)
pub type Height = u64;

/// Timestamp in seconds since Unix epoch
pub type Timestamp = u64;

/// Account balance
pub type Balance = u64;

/// Transaction nonce (for replay protection)
pub type Nonce = u64;
```

Create file: `src/core/types.rs`

**Step 4: Verify compilation**

Run: `cargo check`
Expected: ERROR - other modules still missing (expected)

**Step 5: Commit structure setup**

```bash
git add src/core/mod.rs src/core/types.rs
git commit -m "feat: create core module structure with common types

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Move and Refactor State Module

**Files:**
- Move: `src/state.rs` → `src/core/state.rs`
- Modify: `src/core/state.rs` (use core::types imports)

**Step 1: Read existing state.rs content**

The file exists at `/Users/boycrypt/code/python/website/code/zero_knowledge_proof_learn/week4/code/src/state.rs`

Content to copy (we'll refactor imports):
- Keep entire `Account` struct
- Keep entire `State` struct
- Keep all tests
- Update imports to use `crate::core::types`

**Step 2: Create new state.rs in core/ with refactored imports**

```rust
// State module - Account and State management

use anyhow::Result;
use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tx_rs::SignedTransaction;

// Import common types
use super::types::{Balance, Nonce};

#[allow(unused_imports)]
use tx_rs::{Transaction, sign};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub balance: Balance,
    pub nonce: Nonce,
}

impl Account {
    pub fn new(balance: Balance, nonce: Nonce) -> Self {
        Self { balance, nonce }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    accounts: HashMap<[u8; 32], Account>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn get_account(&self, pubkey: &PublicKey) -> Option<&Account> {
        self.accounts.get(pubkey.as_bytes())
    }

    pub fn set_account(&mut self, pubkey: PublicKey, account: Account) {
        self.accounts.insert(*pubkey.as_bytes(), account);
    }

    pub fn apply_tx(&mut self, signed_tx: &SignedTransaction) -> Result<()> {
        // Verify signature
        if !signed_tx.verify() {
            return Err(anyhow::anyhow!("Invalid signature"));
        }

        let tx = &signed_tx.tx;

        // Check sender has sufficient balance
        let sender_account = self
            .get_account(&tx.from_pubkey.0)
            .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;

        if sender_account.balance < tx.amount {
            return Err(anyhow::anyhow!("Insufficient balance"));
        }

        // Check nonce matches
        if sender_account.nonce != tx.nonce {
            return Err(anyhow::anyhow!(
                "Invalid nonce: expected {}, got {}",
                sender_account.nonce,
                tx.nonce
            ));
        }

        // === Update state ===

        // Deduct from sender and increment nonce
        let sender_key_bytes = tx.from_pubkey.0.as_bytes();
        let sender_account = self.accounts.get_mut(sender_key_bytes).unwrap();
        sender_account.balance -= tx.amount;
        sender_account.nonce += 1;

        // Add to recipient (create account if needed)
        let recipient_key_bytes = tx.to_pubkey.0.as_bytes();
        let recipient_account = self
            .accounts
            .entry(*recipient_key_bytes)
            .or_insert_with(|| Account::new(0, 0));
        recipient_account.balance += tx.amount;

        Ok(())
    }
}

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

    #[test]
    fn test_state_get_and_set() {
        use ed25519_dalek::SecretKey;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let key = SecretKey::generate(&mut OsRng);
        let pubkey: PublicKey = (&key).into();

        // Get non-existent account
        let account = state.get_account(&pubkey);
        assert_eq!(account, None);

        // Set account
        state.set_account(pubkey, Account::new(100, 0));

        // Get account
        let account = state.get_account(&pubkey);
        assert_eq!(account, Some(&Account::new(100, 0)));
    }

    #[test]
    fn test_apply_tx_invalid_signature() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Fund Alice
        let pubkey1: PublicKey = alice_key.public;
        state.set_account(pubkey1, Account::new(100, 0));

        // Create transaction from Alice to Bob
        let pubkey2: PublicKey = bob_key.public;
        let tx = Transaction::new(pubkey1, pubkey2, 50, 0);

        // Sign with WRONG key (Bob signs instead of Alice)
        let signature = sign(&tx, &bob_key);
        let signed_tx = SignedTransaction::new(tx, signature);

        // Try to apply - should fail
        let result = state.apply_tx(&signed_tx);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_tx_insufficient_balance() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Fund Alice with only 10 tokens
        state.set_account(alice_key.public, Account::new(10, 0));

        // Try to send 50 tokens
        let tx = Transaction::new(alice_key.public, bob_key.public, 50, 0);

        let signature = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, signature);

        // Should fail - insufficient balance
        let result = state.apply_tx(&signed_tx);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_tx_incorrect_nonce() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Fund Alice with nonce = 5
        state.set_account(alice_key.public, Account::new(100, 5));

        // Try to send with nonce = 3 (wrong, should be 5)
        let tx = Transaction::new(
            alice_key.public,
            bob_key.public,
            200,
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
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Fund Alice with nonce = 5
        state.set_account(alice_key.public, Account::new(100, 5));

        // Send with correct nonce = 5
        let tx = Transaction::new(
            alice_key.public,
            bob_key.public,
            50,
            5, // Correct nonce
        );

        let signature = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, signature);

        // Should pass validation
        let result = state.apply_tx(&signed_tx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_tx_updates_state() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Setup: Alice has 100, Bob has 50
        state.set_account(alice_key.public, Account::new(100, 0));
        state.set_account(bob_key.public, Account::new(50, 0));

        // Alice sends 30 to Bob
        let tx = Transaction::new(alice_key.public, bob_key.public, 30, 0);

        let signature = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, signature);

        // Apply transaction
        state.apply_tx(&signed_tx).unwrap();

        // Verify: Alice has 70, nonce 1
        let alice_account = state.get_account(&alice_key.public);
        assert_eq!(alice_account.unwrap().balance, 70);
        assert_eq!(alice_account.unwrap().nonce, 1);

        // Verify: Bob has 80, nonce 0
        let bob_account = state.get_account(&bob_key.public);
        assert_eq!(bob_account.unwrap().balance, 80);
        assert_eq!(bob_account.unwrap().nonce, 0);

        // Verify: Can't replay same transaction
        let result = state.apply_tx(&signed_tx);
        assert!(result.is_err());
    }
}
```

Create file: `src/core/state.rs`

**Step 3: Delete old state.rs**

```bash
rm src/state.rs
```

**Step 4: Verify compilation**

Run: `cargo check`
Expected: ERROR - block and chain modules still missing (expected)

**Step 5: Commit state module refactor**

```bash
git add src/core/state.rs
git rm src/state.rs
git commit -m "refactor: move state module to core/ with updated imports

- Move State and Account to src/core/state.rs
- Use core::types type aliases
- All existing tests preserved

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Move and Refactor Block Module

**Files:**
- Move: `src/block.rs` → `src/core/block.rs`
- Modify: `src/core/block.rs` (use core::types)

**Step 1: Create refactored block.rs in core/**

```rust
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tx_rs::SignedTransaction;

use super::types::{Hash, Height, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub prev_hash: Hash,
    pub txs: Vec<SignedTransaction>,
    pub height: Height,
    pub timestamp: Timestamp,
}

impl Block {
    pub fn new(
        prev_hash: Hash,
        txs: Vec<SignedTransaction>,
        height: Height,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            prev_hash,
            txs,
            height,
            timestamp,
        }
    }

    /// Get the hash of this block
    pub fn hash(&self) -> Hash {
        block_hash(self)
    }

    /// Check if this is a genesis block (prev_hash is all zeros)
    pub fn is_genesis(&self) -> bool {
        self.prev_hash == [0u8; 32]
    }
}

pub fn block_hash(block: &Block) -> Hash {
    let serialized = serde_json::to_vec(block).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    let result = hasher.finalize();

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            [0u8; 32],  // prev_hash
            vec![],     // empty transactions
            1,          // height
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

    #[test]
    fn test_block_hash() {
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

    #[test]
    fn test_block_hash_method() {
        let block = Block::new([0u8; 32], vec![], 1, 1234567890);
        let hash1 = block.hash();
        let hash2 = block_hash(&block);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_genesis_detection() {
        let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);
        assert!(genesis.is_genesis());

        let non_genesis = Block::new([1u8; 32], vec![], 1, 1234567890);
        assert!(!non_genesis.is_genesis());
    }
}
```

Create file: `src/core/block.rs`

**Step 2: Delete old block.rs**

```bash
rm src/block.rs
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: ERROR - chain module still missing (expected)

**Step 4: Commit block module refactor**

```bash
git add src/core/block.rs
git rm src/block.rs
git commit -m "refactor: move block module to core/ with helper methods

- Move Block and block_hash to src/core/block.rs
- Add Block::hash() method
- Add Block::is_genesis() helper
- Use core::types type aliases

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Create TX Module (Light Wrapper)

**Files:**
- Create: `src/core/tx.rs`

**Step 1: Create tx.rs as re-export module**

```rust
//! Transaction module
//!
//! This module re-exports types from the `tx-rs` crate for convenience.
//! The actual transaction logic lives in the Week 3 `tx-rs` crate.

pub use tx_rs::{sign, SignedTransaction, Transaction};
```

Create file: `src/core/tx.rs`

**Step 2: Verify compilation**

Run: `cargo check`
Expected: ERROR - chain module still missing (expected)

**Step 3: Commit tx module**

```bash
git add src/core/tx.rs
git commit -m "feat: add tx module as tx-rs re-export

Provides convenient access to transaction types from Week 3.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Create New Chain Module with Fork Support

**Files:**
- Move: `src/chain.rs` → `src/core/chain.rs` (keep apply_block)
- Create: Add new `Blockchain` struct with fork handling
- Modify: `src/core/chain.rs`

**Step 1: Write test for Blockchain struct first (TDD)**

We'll create the test first in `src/core/chain.rs`:

```rust
#[cfg(test)]
mod blockchain_tests {
    use super::*;
    use crate::core::{Block, State};
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;
    use tx_rs::{sign, SignedTransaction, Transaction};

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.len(), 0);
        assert!(blockchain.get_tip().is_none());
    }

    #[test]
    fn test_add_genesis_block() {
        let mut blockchain = Blockchain::new();
        let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);

        let genesis_hash = genesis.hash();
        blockchain.add_block(genesis).unwrap();

        assert_eq!(blockchain.len(), 1);
        assert_eq!(blockchain.get_tip(), Some(&genesis_hash));
    }

    #[test]
    fn test_add_block_creates_fork() {
        let mut blockchain = Blockchain::new();

        // Add genesis
        let genesis = Block::new([0u8; 32], vec![], 0, 1000);
        let genesis_hash = genesis.hash();
        blockchain.add_block(genesis).unwrap();

        // Add block 1a (height 1)
        let block1a = Block::new(genesis_hash, vec![], 1, 2000);
        let hash1a = block1a.hash();
        blockchain.add_block(block1a).unwrap();

        assert_eq!(blockchain.get_tip(), Some(&hash1a));

        // Add block 1b (another block at height 1 - FORK!)
        let block1b = Block::new(genesis_hash, vec![], 1, 2001);
        let hash1b = block1b.hash();
        blockchain.add_block(block1b).unwrap();

        // Both blocks should exist
        assert!(blockchain.get_block(&hash1a).is_some());
        assert!(blockchain.get_block(&hash1b).is_some());

        // Tip should still be 1a (first one wins when heights tie)
        assert_eq!(blockchain.get_tip(), Some(&hash1a));
    }

    #[test]
    fn test_longest_chain_fork_choice() {
        let mut blockchain = Blockchain::new();

        // Genesis -> 1a -> 2a -> 3a (height 3)
        //        ↘ 1b -> 2b     (height 2)

        let genesis = Block::new([0u8; 32], vec![], 0, 1000);
        let genesis_hash = genesis.hash();
        blockchain.add_block(genesis).unwrap();

        // Branch A
        let block1a = Block::new(genesis_hash, vec![], 1, 2000);
        let hash1a = block1a.hash();
        blockchain.add_block(block1a).unwrap();

        let block2a = Block::new(hash1a, vec![], 2, 3000);
        let hash2a = block2a.hash();
        blockchain.add_block(block2a).unwrap();

        let block3a = Block::new(hash2a, vec![], 3, 4000);
        let hash3a = block3a.hash();
        blockchain.add_block(block3a).unwrap();

        assert_eq!(blockchain.get_tip(), Some(&hash3a));

        // Branch B (competitor)
        let block1b = Block::new(genesis_hash, vec![], 1, 2001);
        let hash1b = block1b.hash();
        blockchain.add_block(block1b).unwrap();

        let block2b = Block::new(hash1b, vec![], 2, 3001);
        let hash2b = block2b.hash();
        blockchain.add_block(block2b).unwrap();

        // Tip should still be hash3a (height 3 > height 2)
        assert_eq!(blockchain.get_tip(), Some(&hash3a));

        // Extend branch B to height 4
        let block3b = Block::new(hash2b, vec![], 3, 4001);
        let hash3b = block3b.hash();
        blockchain.add_block(block3b).unwrap();

        // Still 3a (heights tie, first wins)
        assert_eq!(blockchain.get_tip(), Some(&hash3a));

        let block4b = Block::new(hash3b, vec![], 4, 5000);
        let hash4b = block4b.hash();
        blockchain.add_block(block4b).unwrap();

        // Now 4b wins (height 4 > height 3)
        assert_eq!(blockchain.get_tip(), Some(&hash4b));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test blockchain_tests --lib`
Expected: COMPILE ERROR - `Blockchain` doesn't exist yet

**Step 3: Implement Blockchain struct**

Add this above the test in `src/core/chain.rs`:

```rust
use anyhow::Result;
use std::collections::HashMap;

use super::types::Hash;
use super::block::Block;
use super::state::State;

/// Blockchain stores all blocks and tracks the canonical chain tip
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// All blocks indexed by their hash
    blocks: HashMap<Hash, Block>,

    /// Current canonical chain tip (hash of the tip block)
    tip: Option<Hash>,
}

impl Blockchain {
    /// Create a new empty blockchain
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            tip: None,
        }
    }

    /// Get the number of blocks in storage
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Check if blockchain is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Get the current canonical chain tip
    pub fn get_tip(&self) -> Option<&Hash> {
        self.tip.as_ref()
    }

    /// Get a block by its hash
    pub fn get_block(&self, hash: &Hash) -> Option<&Block> {
        self.blocks.get(hash)
    }

    /// Add a block to the blockchain
    ///
    /// This implements a simple "longest chain" fork-choice rule:
    /// - If the new block extends the current tip and has higher height, update tip
    /// - If heights are equal, keep current tip (first-to-arrive wins)
    /// - All blocks are stored regardless of fork choice
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        let block_hash = block.hash();

        // Store the block
        self.blocks.insert(block_hash, block.clone());

        // Update tip using fork-choice rule
        self.update_tip(&block);

        Ok(())
    }

    /// Update the tip using longest-chain fork-choice rule
    fn update_tip(&mut self, new_block: &Block) {
        let new_block_hash = new_block.hash();

        match self.tip {
            None => {
                // First block - becomes tip
                self.tip = Some(new_block_hash);
            }
            Some(current_tip_hash) => {
                // Get current tip block
                if let Some(current_tip) = self.get_block(&current_tip_hash) {
                    // Compare heights
                    if new_block.height > current_tip.height {
                        // New block is higher - switch to it
                        self.tip = Some(new_block_hash);
                    }
                    // If heights are equal, keep current tip (no reorg)
                }
            }
        }
    }

    /// Get the canonical chain from genesis to tip
    /// Returns blocks in order from genesis to tip
    pub fn get_canonical_chain(&self) -> Vec<Block> {
        let mut chain = Vec::new();

        // Start from tip and work backwards
        let mut current_hash = self.tip;

        while let Some(hash) = current_hash {
            if let Some(block) = self.get_block(hash) {
                chain.push(block.clone());

                // Move to parent
                if block.is_genesis() {
                    break;
                } else {
                    current_hash = Some(block.prev_hash);
                }
            } else {
                break;
            }
        }

        // Reverse to get genesis -> tip order
        chain.reverse();
        chain
    }
}

/// Apply a block to the state (legacy function from Week 4)
///
/// This applies transactions sequentially and updates state atomically.
/// If any transaction fails, the entire block application fails.
pub fn apply_block(state: &mut State, block: &Block) -> Result<()> {
    for signed_tx in &block.txs {
        state.apply_tx(signed_tx)?;
    }
    Ok(())
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test blockchain_tests --lib`
Expected: PASS (all 4 tests pass)

**Step 5: Delete old chain.rs**

```bash
rm src/chain.rs
```

**Step 6: Verify full compilation**

Run: `cargo check`
Expected: SUCCESS

**Step 7: Commit chain module with fork support**

```bash
git add src/core/chain.rs
git rm src/chain.rs
git commit -m "feat: add Blockchain struct with fork support

- Add Blockchain struct that stores all blocks in HashMap
- Implement longest-chain fork-choice rule
- Support multiple competing chain tips
- Add get_canonical_chain() to reconstruct chain
- Keep apply_block() from Week 4 for backward compatibility
- Tests cover: creation, forks, longest-chain selection

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Update lib.rs Public API

**Files:**
- Modify: `src/lib.rs`

**Step 1: Write failing test for public API**

Add to `src/lib.rs`:

```rust
#[cfg(test)]
mod public_api_tests {
    use super::*;

    #[test]
    fn test_core_modules_are_accessible() {
        // Test that all core types are accessible
        let _block = Block::new([0u8; 32], vec![], 0, 0);
        let _state = State::new();
        let _blockchain = Blockchain::new();
    }

    #[test]
    fn test_legacy_functions_still_work() {
        use tx_rs::Transaction;
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice = Keypair::generate(&mut OsRng);

        state.set_account(alice.public, Account::new(100, 0));

        let tx = Transaction::new(alice.public, alice.public, 10, 0);
        let sig = crate::core::sign(&tx, &alice);
        let signed_tx = SignedTransaction::new(tx, sig);

        let block = Block::new([0u8; 32], vec![signed_tx], 1, 0);

        // Legacy apply_block still works
        assert!(apply_block(&mut state, &block).is_ok());
    }
}
```

**Step 2: Run tests**

Run: `cargo test public_api_tests`
Expected: FAIL - imports don't match new structure

**Step 3: Update lib.rs to expose new API**

```rust
//! # toychain-rs
//!
//! A minimal blockchain with fork support and consensus concepts.
//!
//! ## Overview
//!
//! This library provides a toy blockchain implementation that demonstrates:
//! - State transition functions (transactions, blocks)
//! - Fork handling and chain selection
//! - Longest-chain consensus rule
//! - Modular architecture with clear separation of concerns
//!
//! ## Architecture
//!
//! The codebase is organized into core modules:
//! - [`core::types`] - Common type definitions
//! - [`core::state`] - Account and state management
//! - [`core::block`] - Block structure and hashing
//! - [`core::tx`] - Transaction types (re-exported from tx-rs)
//! - [`core::chain`] - Blockchain storage and fork handling
//!
//! ## Example
//!
//! ```rust
//! use toychain_rs::{Blockchain, State, apply_block, Block};
//! use toychain_rs::core::{Account, block_hash};
//! use ed25519_dalek::Keypair;
//! use rand::rngs::OsRng;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create blockchain and state
//! let mut blockchain = Blockchain::new();
//! let mut state = State::new();
//!
//! // Setup accounts
//! let alice = Keypair::generate(&mut OsRng);
//! state.set_account(alice.public, Account::new(100, 0));
//!
//! // Create and add genesis block
//! let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);
//! blockchain.add_block(genesis)?;
//!
//! // Create a block with transactions
//! // ... (create signed transactions)
//! // let block = Block::new(...);
//! // blockchain.add_block(block)?;
//!
//! # Ok(())
//! # }
//! ```

pub mod core;

// Public API exports - core types
pub use core::{
    block::{block_hash, Block},
    chain::{apply_block, Blockchain},
    state::{Account, State},
    tx::{sign, SignedTransaction, Transaction},
};

// Public API exports - type aliases
pub use core::types::{Hash, Height, Timestamp, Balance, Nonce};
```

**Step 4: Run tests to verify they pass**

Run: `cargo test public_api_tests`
Expected: PASS

**Step 5: Commit lib.rs refactor**

```bash
git add src/lib.rs
git commit -m "refactor: update lib.rs for new modular architecture

- Re-export core modules with clean public API
- Maintain backward compatibility with Week 4 API
- Add comprehensive documentation
- Tests verify public API accessibility

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Update Integration Test for Fork Scenario

**Files:**
- Modify: `tests/integration_test.rs`

**Step 1: Add fork simulation test to integration tests**

```rust
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
```

**Step 2: Run integration tests**

Run: `cargo test --test integration_test`
Expected: PASS (all 3 tests: original workflow + fork resolution + reorg)

**Step 3: Commit integration test updates**

```bash
git add tests/integration_test.rs
git commit -m "test: add fork simulation and reorg tests

- test_fork_resolution: create two competing forks at same height
- test_chain_reorg_on_longer_fork: demonstrate chain reorg to longer fork
- Both tests verify fork-choice rule and canonical chain reconstruction
- Original end-to-end test preserved

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Update Documentation

**Files:**
- Modify: `README.md`

**Step 1: Create comprehensive README**

```markdown
# toychain-rs: Fork-Aware Toy Blockchain in Rust

A minimal blockchain implementation demonstrating fork handling, longest-chain consensus, and modular architecture.

## Overview

This project (Week 5 of the ZK learning journey) extends Week 4's state transition function with:
- **Fork support**: Store multiple competing chains
- **Consensus**: Longest-chain fork-choice rule
- **Reorgs**: Automatic switching to better chains
- **Modular architecture**: Clean separation of concerns

## Architecture

The codebase is organized into core modules:

```
src/
├── core/
│   ├── mod.rs       # Module declarations
│   ├── types.rs     # Common type aliases (Hash, Height, etc.)
│   ├── state.rs     # Account and State management
│   ├── block.rs     # Block structure and hashing
│   ├── tx.rs        # Transaction types (re-exports tx-rs)
│   └── chain.rs     # Blockchain storage and fork handling
└── lib.rs           # Public API exports
```

### Key Concepts

**Forks**: When two blocks are mined at the same height, both are stored. The blockchain maintains a "tip" representing the canonical chain.

**Fork-Choice Rule**: Simple "longest chain" rule - the chain with the highest height wins. On ties, first-to-arrive wins.

**Reorgs**: When a longer chain is discovered, the tip updates to point to it. This simulates how real blockchains handle forks.

## Usage

### Creating a Blockchain

```rust
use toychain_rs::Blockchain;

let mut blockchain = Blockchain::new();

// Add genesis block
let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);
blockchain.add_block(genesis)?;
```

### Creating Forks

```rust
// Both block1a and block1b extend genesis at height 1
let block1a = Block::new(genesis_hash, vec![], 1, 2000);
let block1b = Block::new(genesis_hash, vec![], 1, 2001);

blockchain.add_block(block1a)?;
blockchain.add_block(block1b)?;

// Tip will be block1a (first to arrive)
assert_eq!(blockchain.get_tip(), Some(&hash1a));
```

### Chain Reorganization

```rust
// Extend fork A to height 2
let block2a = Block::new(hash1a, vec![], 2, 3000);
blockchain.add_block(block2a)?;
assert_eq!(blockchain.get_tip(), Some(&hash2a));

// Extend fork B to height 3 (wins!)
let block2b = Block::new(hash1b, vec![], 2, 3001);
blockchain.add_block(block2b)?;

let block3b = Block::new(hash2b, vec![], 3, 4000);
blockchain.add_block(block3b)?;

// Tip reorgs to block3b (height 3 > height 2)
assert_eq!(blockchain.get_tip(), Some(&hash3b));
```

### Getting the Canonical Chain

```rust
let chain = blockchain.get_canonical_chain();
// Returns: [genesis, ..., ..., tip]
for block in &chain {
    println!("Height {}: {}", block.height, hex::encode(block.hash()));
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_fork_resolution

# Run integration tests
cargo test --test integration_test

# Check code
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Fork Handling Details

### Storage
All blocks are stored in a `HashMap<Hash, Block>`, regardless of whether they're on the canonical chain. This preserves fork history.

### Tip Selection
The tip is updated using these rules:
- New block height > current tip height → switch to new block
- New block height == current tip height → keep current tip (no reorg)
- New block height < current tip height → ignore (not canonical)

### Canonical Chain Reconstruction
`get_canonical_chain()` starts from the tip and follows `prev_hash` links backwards to genesis, then reverses the list.

## Limitations (Toy vs. Production)

This is a **toy** blockchain for learning. Key simplifications:

- **No proof-of-work**: Any block can be added
- **No difficulty adjustment**: All blocks are valid
- **No finality**: Tips can change arbitrarily
- **No validation**: Blocks aren't validated beyond basic structure
- **Simple consensus**: Longest chain only (no total difficulty, no GHOST)
- **No networking**: Single-machine only
- **No persistence**: In-memory only (lost on restart)

## What This Teaches

By working through this codebase, you'll learn:
1. **How forks happen**: Concurrent block creation at same height
2. **Fork-choice rules**: Selecting the "best" chain
3. **Chain reorganization**: Switching to a better chain
4. **Modular design**: Separating types, state, blocks, and chain logic
5. **Test-driven development**: Comprehensive tests for fork scenarios

## Next Steps (Week 6+)

This foundation prepares you for:
- ZK foundations (statements, witnesses, relations)
- Constraint systems (R1CS)
- SNARK proving systems
- ZK applications (Merkle membership, rollups)

## Dependencies

- `tx-rs`: Transaction types from Week 3
- `ed25519-dalek`: Digital signatures
- `sha2`: SHA-256 hashing
- `serde`: Serialization
- `anyhow`: Error handling

## License

Educational use only.
```

Create file: `README.md` (replace existing)

**Step 2: Commit documentation**

```bash
git add README.md
git commit -m "docs: comprehensive README for fork-aware toychain

- Explain architecture and module organization
- Document fork handling and reorg behavior
- Provide usage examples for forks and chain reorganization
- List limitations (toy vs. production)
- Link to learning goals and next steps

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Verify Everything Works

**Step 1: Run full test suite**

```bash
cargo test --all
```

Expected: PASS (all tests: unit + integration)

**Step 2: Run clippy**

```bash
cargo clippy -- -D warnings
```

Expected: SUCCESS (zero warnings)

**Step 3: Format code**

```bash
cargo fmt
```

Expected: Reformats files (if needed)

**Step 4: Build documentation**

```bash
cargo doc --open
```

Expected: Opens browser with documentation

**Step 5: Final verification commit**

```bash
git add .
git commit -m "test: verify all tests pass and code is clean

- cargo test --all: PASS
- cargo clippy: Zero warnings
- cargo fmt: Code formatted
- cargo doc --open: Documentation builds

Week 5 implementation complete!

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Create Architecture Diagram (Optional)

**Files:**
- Create: `docs/architecture.md`

**Step 1: Create architecture documentation**

```markdown
# toychain-rs Architecture

## Module Dependency Graph

```
┌─────────────────────────────────────────┐
│              lib.rs                     │
│         (Public API Exports)            │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│           core/mod.rs                   │
└──────────────┬──────────────────────────┘
               │
      ┌────────┴────────┐
      │                 │
      ▼                 ▼
┌──────────┐      ┌──────────┐
│ types.rs │      │ block.rs │
│ (Hash,   │◄─────┤ (Block,  │
│  Height, │      │  hash)   │
│  etc.)   │      └──────────┘
└──────────┘             │
      │                 │
      │                 ▼
      │          ┌──────────┐
      │          │  tx.rs   │
      │          │ (re-exports│
      │          │  tx-rs)  │
      │          └──────────┘
      │
      ▼
┌──────────┐      ┌──────────┐
│ state.rs │◄─────┤ chain.rs │
│(Account, │      │(Blockchain│
│ State)   │─────►│ + Forks) │
└──────────┘      └──────────┘
      ▲                 │
      └─────────────────┘
         (State updates)
```

## Data Flow

### Adding a Block

```
Block
  │
  ├─► block.hash() → Hash
  │
  └─► blockchain.add_block(block)
        │
        ├─► blocks.insert(hash, block)
        │
        └─► update_tip()
              │
              ├─► Compare heights
              │
              └─► Update tip if higher
```

### Fork Resolution

```
Block1a (height 1)
  └─► Tip = 1a

Block1b (height 1)
  └─► Tip = 1a (tie, first wins)

Block2a (height 2, extends 1a)
  └─► Tip = 2a (2 > 1)

Block2b (height 2, extends 1b)
Block3b (height 3, extends 2b)
  └─► Tip = 3b (3 > 2, REORG!)
```

### Canonical Chain Reconstruction

```
get_canonical_chain()
  │
  ├─► Start at tip
  │
  ├─► Follow prev_hash backwards
  │     Until genesis (prev_hash == 0)
  │
  └─► Reverse list
        Result: [genesis, ..., tip]
```

## Key Design Decisions

### 1. HashMap Storage
**Decision**: Store all blocks in `HashMap<Hash, Block>`
**Rationale**:
- O(1) lookup by hash
- Keeps fork history
- Simple implementation

### 2. Longest-Chain Rule
**Decision**: Tip = block with maximum height
**Rationale**:
- Standard Bitcoin-like consensus
- Simple to understand
- Deterministic (ties broken by arrival order)

### 3. Separate Tip Tracking
**Decision**: Store `tip: Option<Hash>` separately from blocks
**Rationale**:
- O(1) tip access
- Easy to update
- Clear separation of data vs. metadata

### 4. Lazy Chain Reconstruction
**Decision**: `get_canonical_chain()` builds chain on-demand
**Rationale**:
- No redundant storage
- Always returns current state
- Simple implementation

## Trade-offs

### Simplicity vs. Features
We chose simplicity over production features:
- ❌ No proof-of-work
- ❌ No difficulty adjustment
- ❌ No total difficulty (just height)
- ❌ No uncle blocks
- ✅ Easy to understand
- ✅ Good for learning

### Performance vs. Clarity
We chose code clarity over micro-optimizations:
- ❌ Cloning blocks in `get_canonical_chain()`
- ❌ Multiple hash lookups
- ✅ Readable code
- ✅ Safe Rust (no unsafe)

## Extension Ideas

To make this more production-like:
1. Add proof-of-work validation
2. Track total difficulty instead of height
3. Implement GHOST fork-choice rule
4. Add block validation rules
5. Persist blocks to disk (sled database)
6. Add networking (libp2p)
```

Create file: `docs/architecture.md`

**Step 2: Commit architecture docs**

```bash
git add docs/architecture.md
git commit -m "docs: add architecture diagram and design decisions

- Module dependency graph
- Data flow diagrams
- Key design decisions with rationale
- Trade-offs (simplicity vs features)
- Extension ideas for production

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Summary

This plan refactors Week 4's toychain into a modular, fork-aware blockchain with:

**Architecture (10 tasks):**
1. ✅ Create `core/` module structure
2. ✅ Move/refactor state module
3. ✅ Move/refactor block module
4. ✅ Create tx wrapper module
5. ✅ Implement Blockchain with fork support
6. ✅ Update lib.rs public API
7. ✅ Add fork simulation tests
8. ✅ Write comprehensive README
9. ✅ Verify and test everything
10. ✅ Document architecture

**Key Features:**
- Modular `src/core/` structure (types, state, block, tx, chain)
- `Blockchain` struct storing all blocks in HashMap
- Longest-chain fork-choice rule
- Chain reorg support
- `get_canonical_chain()` for chain reconstruction
- Comprehensive tests (unit + integration)

**Learning Outcomes:**
- How forks occur and are handled
- Fork-choice rules (longest chain)
- Chain reorganizations
- Modular architecture design
- Test-driven development
