# Analysis of Task 5 Deviations

## Context
The spec reviewer identified two deviations in the Task 5 implementation:
1. Code uses `tx.from_pubkey.0` instead of `tx.from_pubkey`
2. Test uses `alice_key.public` instead of `alice_key.public_key`

## Investigation Results

### Question 1: What is the actual type of `tx.from_pubkey`?

**Answer**: It is `HashablePublicKey`, a wrapper type.

**Evidence from `/week3/code/src/transaction.rs`**:
```rust
/// Wrapper around PublicKey that implements Hash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashablePublicKey(pub PublicKey);  // ← Tuple struct wrapping PublicKey

impl Hash for HashablePublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_bytes().hash(state);  // ← Access inner PublicKey via .0
    }
}
```

**Transaction structure**:
```rust
pub struct Transaction {
    pub from_pubkey: HashablePublicKey,  // ← Wrapper type, not PublicKey
    pub to_pubkey: HashablePublicKey,
    pub amount: u64,
    pub nonce: u64,
}
```

**Why `tx.from_pubkey.0` is necessary**:
- `from_pubkey` is of type `HashablePublicKey`
- To get the inner `PublicKey`, we must access tuple field `.0`
- This is a **technical necessity** due to the wrapper type design

---

### Question 2: What is the correct way to access public key from `Keypair` in ed25519-dalek v1.0?

**Answer**: `keypair.public` (NOT `keypair.public_key`)

**Evidence from working examples in `/week3/code/examples/`**:

From `basic_transaction.rs` (lines 21-27):
```rust
println!("Alice's public key: {:?}", alice_keypair.public);  // ← .public
println!("Bob's public key: {:?}", bob_keypair.public);      // ← .public

let tx = Transaction::new(
    alice_keypair.public,  // ← .public
    bob_keypair.public,    // ← .public
    100,
    1,
);
```

From `mempool_demo.rs` (multiple occurrences):
```rust
alice_keypair.public,   // ← Uses .public
bob_keypair.public,     // ← Uses .public
charlie_keypair.public, // ← Uses .public
```

**From week3 test suite** (`transaction.rs` lines 127-135):
```rust
let keypair1: Keypair = Keypair::generate(&mut csprng);
let keypair2: Keypair = Keypair::generate(&mut csprng);

let tx = Transaction::new(
    keypair1.public,   // ← Uses .public
    keypair2.public,   // ← Uses .public
    100,
    1,
);
```

**Dependency version from `/week4/code/Cargo.toml`**:
```toml
[dependencies]
ed25519-dalek = { version = "1.0", features = ["serde"] }
```

**Conclusion**: In ed25519-dalek v1.0, the correct API is `keypair.public`

---

### Question 3: Are these technical necessities or avoidable deviations?

## Deviation 1: `tx.from_pubkey.0` instead of `tx.from_pubkey`

**Status**: ✅ **TECHNICAL NECESSITY** - This is CORRECT

**Reasoning**:
1. `Transaction.from_pubkey` has type `HashablePublicKey`, not `PublicKey`
2. `HashablePublicKey` is a tuple struct: `pub struct HashablePublicKey(pub PublicKey)`
3. The only way to access the inner `PublicKey` is via `.0`
4. The spec writer likely misunderstood the actual type structure

**The code in `/week4/code/src/state.rs` line 50**:
```rust
let sender_account = self.get_account(&tx.from_pubkey.0)  // ← CORRECT
    .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;
```

This is **necessary** because `get_account` expects `&PublicKey`:
```rust
pub fn get_account(&self, pubkey: &PublicKey) -> Option<&Account> {
    self.accounts.get(pubkey.as_bytes())
}
```

---

## Deviation 2: `alice_key.public` instead of `alice_key.public_key`

**Status**: ✅ **TECHNICAL NECESSITY** - This is CORRECT

**Reasoning**:
1. The actual API in ed25519-dalek v1.0 is `Keypair.public`
2. The field is literally named `public`, not `public_key`
3. This is documented in the library's API and used throughout the codebase
4. Using `public_key` would be a **compilation error**

**Evidence**: All working examples in week3 use `keypair.public`:
- `examples/basic_transaction.rs`
- `examples/mempool_demo.rs`
- `src/transaction.rs` tests

---

## Summary

| Deviation | Status | Reason |
|-----------|--------|--------|
| `tx.from_pubkey.0` | ✅ CORRECT | `from_pubkey` is `HashablePublicKey(pub PublicKey)`, tuple struct requires `.0` access |
| `alice_key.public` | ✅ CORRECT | ed25519-dalek v1.0 API uses `.public` field, not `.public_key` |

## Root Cause Analysis

The spec was written based on **assumed APIs** rather than the **actual dependencies**:

1. **Spec assumption**: Transaction fields are direct `PublicKey` type
   - **Reality**: They use `HashablePublicKey` wrapper for HashMap compatibility

2. **Spec assumption**: Keypair uses `.public_key` field
   - **Reality**: ed25519-dalek v1.0 uses `.public` field

## Recommendation

**The implementation is CORRECT**. The deviations are technical necessities required by:
1. The `HashablePublicKey` wrapper type design in tx-rs
2. The actual ed25519-dalek v1.0 API

The spec should be updated to reflect the actual APIs being used.
