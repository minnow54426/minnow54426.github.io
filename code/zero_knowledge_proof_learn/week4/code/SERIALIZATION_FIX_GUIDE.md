# Quick Fix Guide: Adding Serialization to SignedTransaction

## The Problem
`Block` in week4 needs `Serialize, Deserialize`, but `SignedTransaction` from week3 doesn't have them.

## The Solution
Add 4 derives across 2 files in week3. All dependencies already support serde.

## Files to Change

### 1. `/Users/boycrypt/code/python/website/code/zero_knowledge_proof_learn/week3/code/src/transaction.rs`

**Change 1 - Line 14:**
```rust
// BEFORE:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashablePublicKey(pub PublicKey);

// AFTER:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashablePublicKey(pub PublicKey);
```

**Change 2 - Line 36:**
```rust
// BEFORE:
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub from_pubkey: HashablePublicKey,
    pub to_pubkey: HashablePublicKey,
    pub amount: u64,
    pub nonce: u64,
}

// AFTER:
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub from_pubkey: HashablePublicKey,
    pub to_pubkey: HashablePublicKey,
    pub amount: u64,
    pub nonce: u64,
}
```

### 2. `/Users/boycrypt/code/python/website/code/zero_knowledge_proof_learn/week3/code/src/crypto.rs`

**Change 3 - Line 35:**
```rust
// BEFORE:
#[derive(Debug, Clone)]
pub struct SignedTransaction {
    pub tx: Transaction,
    pub signature: Signature,
    pub tx_id: TxId,
}

// AFTER:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub tx: Transaction,
    pub signature: Signature,
    pub tx_id: TxId,
}
```

## Verify It Works

```bash
cd /Users/boycrypt/code/python/website/code/zero_knowledge_proof_learn/week3/code

# Run tests
cargo test

# Check for warnings
cargo clippy

# If everything passes, serialization is working!
```

## Why This Works

- `ed25519_dalek = { version = "1.0", features = ["serde"] }` - serde feature already enabled
- `Signature` implements Serialize/Deserialize when serde feature is on
- `PublicKey` implements Serialize/Deserialize when serde feature is on
- We're just adding the derives to tell serde to use those implementations

## What This Enables

After these changes, week4 can do:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub prev_hash: [u8; 32],
    pub txs: Vec<SignedTransaction>,  // ✓ Now works!
    pub height: u64,
    pub timestamp: u64,
}

// Can now serialize blocks:
let block = Block::new(...);
let json = serde_json::to_string(&block).unwrap();  // ✓ Works!
let bytes = bincode::serialize(&block).unwrap();     // ✓ Works!
```

## Risk: ZERO

- Adding derives is non-breaking
- All existing code continues to work
- No API changes
- No behavior changes
- Only adds new capability (serialization)
