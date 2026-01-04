# Task 5 Spec Deviation - Final Summary

## Questions Asked

**Question 1**: What is the actual type of `tx.from_pubkey`?
**Question 2**: What is the correct way to access public key from `Keypair` in ed25519-dalek v1.0?
**Question 3**: Are these technical necessities or avoidable deviations?

---

## Answers

### Question 1: Actual type of `tx.from_pubkey`

**Answer**: `HashablePublicKey` - a tuple wrapper struct

**Location**: `/week3/code/src/transaction.rs`
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashablePublicKey(pub PublicKey);  // ← Tuple struct

pub struct Transaction {
    pub from_pubkey: HashablePublicKey,  // ← Not PublicKey directly!
    pub to_pubkey: HashablePublicKey,
    pub amount: u64,
    pub nonce: u64,
}
```

**Why this design**: `PublicKey` doesn't implement `Hash`, so cannot be used as HashMap key directly. The wrapper enables `HashMap<HashablePublicKey, Account>`.

---

### Question 2: Correct way to access public key from `Keypair`

**Answer**: `keypair.public` (NOT `keypair.public_key`)

**Evidence**: All working code in the codebase uses `.public`:

From `/week3/code/examples/basic_transaction.rs`:
```rust
let alice_keypair = Keypair::generate(&mut csprng);
let tx = Transaction::new(
    alice_keypair.public,  // ← Uses .public
    bob_keypair.public,    // ← Uses .public
    100, 1,
);
```

From `/week4/code/src/state.rs` tests:
```rust
let alice_key = Keypair::generate(&mut OsRng);
state.set_account(alice_key.public, Account::new(100, 0));  // ← Uses .public
```

---

### Question 3: Technical necessities or avoidable?

**Answer**: Both are TECHNICAL NECESSITIES - the implementation is CORRECT

#### Deviation 1: `tx.from_pubkey.0`

**Status**: ✅ NECESSARY - Type system requires it

**Reasoning**:
```rust
// from_pubkey type: HashablePublicKey(pub PublicKey)
// To get inner PublicKey, must use .0
let pubkey: PublicKey = tx.from_pubkey.0;  // ← Required by type system
```

If we tried to use `tx.from_pubkey` directly:
```rust
// ❌ COMPILATION ERROR:
let sender = self.get_account(&tx.from_pubkey);
//    type mismatch: expected `PublicKey`, found `HashablePublicKey`
```

#### Deviation 2: `alice_key.public`

**Status**: ✅ NECESSARY - Library API requires it

**Reasoning**:
```rust
// ed25519-dalek v1.0 defines:
pub struct Keypair { pub public: PublicKey, ... }
//                           ^^^^^^ field name is "public"

let pubkey = alice_key.public;  // ← Required by library API
```

If we tried to use `.public_key`:
```rust
// ❌ COMPILATION ERROR:
let pubkey = alice_key.public_key;
//    no field named `public_key` on type `Keypair`
```

---

## Proof of Correctness

### All tests pass:
```bash
$ cargo test --lib
running 5 tests
test state::tests::test_account_creation ... ok
test state::tests::test_account_serialization ... ok
test state::tests::test_state_get_and_set ... ok
test state::tests::test_apply_tx_invalid_signature ... ok
test state::tests::test_apply_tx_insufficient_balance ... ok

test result: ok. 5 passed; 0 failed
```

### If spec assumptions were used:
- Code would NOT compile
- Type mismatch errors
- Missing field errors

---

## Root Cause

The spec was written based on **assumed** APIs:
- Assumed `Transaction` fields are direct `PublicKey`
- Assumed `Keypair` uses `.public_key` field

Reality differs:
- `Transaction` uses `HashablePublicKey(pub PublicKey)` wrapper
- `Keypair` uses `.public` field (ed25519-dalek v1.0)

---

## Recommendation

**DO NOT CHANGE THE IMPLEMENTATION**

The code is correct for the actual dependencies being used:
- `tx-rs` with `HashablePublicKey` wrapper
- `ed25519-dalek v1.0` with `.public` field

Instead, update the spec to reflect the actual APIs.

---

## Documentation Created

1. **`spec_analysis_answers.md`** - Q&A format analysis
2. **`SPEC_DEVIATION_ANALYSIS.md`** - Comprehensive technical report
3. **`TYPE_STRUCTURE_DIAGRAM.md`** - Visual type relationship diagrams
4. **`verification_test.rs`** - Demonstration of compilation errors
5. **`FINAL_SUMMARY.md`** - This document

All files located in: `/week4/code/`
