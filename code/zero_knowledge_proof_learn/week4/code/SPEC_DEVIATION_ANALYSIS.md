# Task 5 Spec Deviation Analysis - Complete Report

## Executive Summary

**Finding**: Both identified "deviations" are **CORRECT** and **necessary** for the code to compile and function properly with the actual dependencies being used.

**Root Cause**: The spec was written based on assumed APIs rather than the actual library implementations.

---

## Detailed Analysis

### Deviation 1: Using `tx.from_pubkey.0` instead of `tx.from_pubkey`

#### Spec Assumption
```rust
// Spec assumed from_pubkey was PublicKey directly
let sender_account = self.get_account(&tx.from_pubkey);
```

#### Actual Implementation
```rust
// Line 50 in state.rs
let sender_account = self.get_account(&tx.from_pubkey.0)  // ← Uses .0
    .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;
```

#### Why the Implementation is CORRECT

**Type Analysis**:
- `Transaction` is defined in `/week3/code/src/transaction.rs`
- The `from_pubkey` field has type `HashablePublicKey`, NOT `PublicKey`

```rust
// From week3/code/src/transaction.rs lines 14-15
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashablePublicKey(pub PublicKey);  // ← Tuple struct
```

```rust
// Lines 36-46
pub struct Transaction {
    pub from_pubkey: HashablePublicKey,  // ← Wrapper type
    pub to_pubkey: HashablePublicKey,
    pub amount: u64,
    pub nonce: u64,
}
```

**Why HashablePublicKey Exists**:
- `ed25519_dalek::PublicKey` does NOT implement `Hash` trait
- Cannot be used directly as HashMap key
- Wrapper pattern: `HashablePublicKey(pub PublicKey)` implements `Hash`
- Enables `HashMap<HashablePublicKey, Account>` usage

**Access Pattern**:
```rust
// To get the inner PublicKey from HashablePublicKey:
let pubkey: PublicKey = hashable_pubkey.0;  // ← Tuple field access
```

**Verification**:
```rust
// This would NOT compile:
let pubkey: PublicKey = tx.from_pubkey;  // ❌ Type mismatch

// This DOES compile:
let pubkey: PublicKey = tx.from_pubkey.0;  // ✅ Correct
```

---

### Deviation 2: Using `alice_key.public` instead of `alice_key.public_key`

#### Spec Assumption
```rust
// Spec assumed keypair.public_key API
let pubkey: PublicKey = alice_key.public_key;
```

#### Actual Implementation
```rust
// Line 111 in state.rs
let pubkey1: PublicKey = alice_key.public;  // ← Uses .public
state.set_account(pubkey1, Account::new(100, 0));

// Line 142
state.set_account(alice_key.public, Account::new(10, 0));

// Line 146-148
let tx = Transaction::new(
    alice_key.public,  // ← Uses .public
    bob_key.public,    // ← Uses .public
    50, 0,
);
```

#### Why the Implementation is CORRECT

**API in ed25519-dalek v1.0**:
- The `Keypair` struct has a PUBLIC field named `public`
- There is NO field named `public_key`

**Evidence from Working Code**:

1. **Week3 Examples** (`/week3/code/examples/basic_transaction.rs`):
```rust
// Lines 21-26
println!("Alice's public key: {:?}", alice_keypair.public);
println!("Bob's public key: {:?}", bob_keypair.public);

let tx = Transaction::new(
    alice_keypair.public,  // ← Uses .public
    bob_keypair.public,    // ← Uses .public
    100, 1,
);
```

2. **Week3 Tests** (`/week3/code/src/transaction.rs`):
```rust
// Lines 127-135
let keypair1: Keypair = Keypair::generate(&mut csprng);
let keypair2: Keypair = Keypair::generate(&mut csprng);

let tx = Transaction::new(
    keypair1.public,  // ← Uses .public
    keypair2.public,  // ← Uses .public
    100, 1,
);
```

3. **Week3 Examples** (`/week3/code/examples/mempool_demo.rs`):
```rust
// Multiple uses throughout:
alice_keypair.public,   // ← Uses .public
bob_keypair.public,     // ← Uses .public
charlie_keypair.public, // ← Uses .public
```

**Verification**:
```rust
// This would NOT compile:
let pubkey = alice_key.public_key;  // ❌ No field named `public_key`

// This DOES compile:
let pubkey = alice_key.public;      // ✅ Correct
```

---

## Dependency Verification

**From `/week4/code/Cargo.toml`**:
```toml
[dependencies]
tx-rs = { path = "../../week3/code" }
ed25519-dalek = { version = "1.0", features = ["serde"] }
```

**Fact Check**:
- ✅ `tx-rs` uses `HashablePublicKey` wrapper (verified in week3 source)
- ✅ `ed25519-dalek = "1.0"` uses `.public` field (verified in working examples)

---

## Test Results

**All tests pass with the "deviant" implementation**:
```bash
$ cargo test --lib
running 5 tests
test state::tests::test_account_creation ... ok
test state::tests::test_account_serialization ... ok
test state::tests::test_state_get_and_set ... ok
test state::tests::test_apply_tx_insufficient_balance ... ok
test state::tests::test_apply_tx_invalid_signature ... ok

test result: ok. 5 passed; 0 failed; 0 ignored
```

**If we changed to spec assumptions, the code would NOT compile**:
- Type mismatch errors for `HashablePublicKey` vs `PublicKey`
- Missing field errors for `.public_key` vs `.public`

---

## Conclusion

### Both Deviations are TECHNICAL NECESSITIES

1. **`tx.from_pubkey.0`** is required because:
   - `from_pubkey` has type `HashablePublicKey(pub PublicKey)`
   - Tuple struct requires `.0` to access inner value
   - Cannot use directly as `PublicKey`

2. **`alice_key.public`** is required because:
   - ed25519-dalek v1.0 API uses field name `.public`
   - There is no `.public_key` field
   - Using `.public_key` causes compilation error

### Recommendation

**The implementation is CORRECT and should NOT be changed.**

Instead, the specification should be updated to reflect:
1. The actual `HashablePublicKey` wrapper type in tx-rs
2. The actual ed25519-dalek v1.0 API (`Keypair.public`)

The spec writer made reasonable assumptions about likely APIs, but the actual dependencies have different interfaces.

---

## Supporting Documentation

**Files Created During Analysis**:
1. `/week4/code/spec_analysis_answers.md` - Q&A format analysis
2. `/week4/code/verification_test.rs` - Demonstration of compilation errors
3. `/week4/code/SPEC_DEVIATION_ANALYSIS.md` - This comprehensive report

**Evidence Sources**:
- `/week3/code/src/transaction.rs` - HashablePublicKey definition
- `/week3/code/src/lib.rs` - Public API exports
- `/week3/code/examples/basic_transaction.rs` - Working API usage
- `/week3/code/examples/mempool_demo.rs` - Working API usage
- `/week4/code/Cargo.toml` - Dependency versions
- `/week4/code/src/state.rs` - Implementation under review
