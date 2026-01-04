# Quick Reference - Task 5 Deviations

## TL;DR

**Both deviations are CORRECT and NECESSARY**

## Deviation 1: `tx.from_pubkey.0`

### What the spec said:
```rust
let account = state.get_account(&tx.from_pubkey);
```

### What the code does:
```rust
let account = state.get_account(&tx.from_pubkey.0);  // ← .0 required
```

### Why?
- `tx.from_pubkey` type is `HashablePublicKey`, NOT `PublicKey`
- `HashablePublicKey` is a tuple struct: `struct HashablePublicKey(pub PublicKey)`
- Must use `.0` to access inner `PublicKey`
- This is REQUIRED by the type system

---

## Deviation 2: `alice_key.public`

### What the spec said:
```rust
let pubkey = alice_key.public_key;
```

### What the code does:
```rust
let pubkey = alice_key.public;  // ← .public not .public_key
```

### Why?
- ed25519-dalek v1.0 API uses field name `.public`
- There is NO field named `.public_key`
- This is REQUIRED by the library API

---

## Evidence

### Type Definitions
```rust
// From /week3/code/src/transaction.rs
pub struct HashablePublicKey(pub PublicKey);  // ← Tuple struct

pub struct Transaction {
    pub from_pubkey: HashablePublicKey,  // ← Wrapper type
    pub to_pubkey: HashablePublicKey,
    ...
}
```

### Working Examples
```rust
// From /week3/code/examples/basic_transaction.rs
let alice_keypair = Keypair::generate(&mut csprng);
let tx = Transaction::new(
    alice_keypair.public,  // ← Uses .public
    bob_keypair.public,    // ← Uses .public
    100, 1,
);
```

### Test Results
```bash
$ cargo test --lib
test result: ok. 5 passed; 0 failed  ✅
```

---

## What Would Happen If We Changed to Spec?

### Change 1: Remove `.0`
```rust
let account = state.get_account(&tx.from_pubkey);
// ❌ COMPILATION ERROR: type mismatch
//    expected `PublicKey`, found `HashablePublicKey`
```

### Change 2: Use `.public_key`
```rust
let pubkey = alice_key.public_key;
// ❌ COMPILATION ERROR: no field named `public_key`
```

---

## Conclusion

✅ Implementation is CORRECT
✅ Tests PASS
✅ Code COMPILES
✅ Matches actual library APIs

❌ Spec assumptions don't match reality
❌ Following spec literally causes compilation errors

**Recommendation**: Update spec to match actual APIs, NOT the other way around.
