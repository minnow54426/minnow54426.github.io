# BIP340 Schnorr Signatures - Design Document

**Date:** 2025-01-23
**Author:** Claude Code + User
**Status:** Design Complete

## Overview

This document describes the design and implementation of a production-ready BIP340 Schnorr signature library in Rust, targeting secp256k1 for Bitcoin compatibility.

## Goals

- **Production-ready library**: High-quality, well-tested implementation
- **BIP340 compliance**: Compatible with Bitcoin's Schnorr specification
- **Educational value**: Clear implementation for learning cryptographic protocols
- **Cross-validation**: Verify against existing implementations (secp256k1 crate)
- **Comprehensive testing**: Test vectors, edge cases, property-based tests

## Architecture

### Technology Stack

- **Curve**: secp256k1 (Bitcoin standard)
- **Crypto primitives**: `k256` crate (battle-tested elliptic curve operations)
- **Hash function**: SHA-256 (via `sha2` crate)
- **Randomness**: `rand_core` for nonce generation
- **Validation**: `secp256k1` crate for cross-library testing

### Design Approach

We use **Approach 2**: Stand on tested foundations while implementing protocol logic.

- **Use `k256` for**: Field arithmetic, point operations, scalar math
- **Implement ourselves**: Schnorr protocol, nonce generation, signatures, verification
- **Benefit**: Safe low-level operations + deep protocol understanding

## Component Design

### 1. Core Types

**SecretKey:**
- Wraps `k256::Scalar` (256-bit scalar in [1, n-1])
- Never serialized (security)
- Zeroized on drop

**PublicKey:**
- Wraps `k256::AffinePoint`
- Compressed SEC encoding (33 bytes)
- Supports precomputation for batch verification

**Signature:**
- Struct containing `(r: [u8; 32], s: [u8; 32])`
- `r`: x-coordinate of R = k*G
- `s`: s = k + H(R||P||m)*x

**KeyPair:**
- Combines SecretKey + PublicKey
- From `P = x*G` derivation

### 2. Nonce Generation (Security-Critical!)

**Algorithm:**
```rust
nonce = SHA256("BIP340/nonce" || secret_key || message || aux_rand)
```

**Security Properties:**
- **Deterministic**: Prevents nonce reuse bugs
- **Unpredictable**: Derived from secret key
- **Domain separation**: Tag prevents cross-protocol attacks
- **Defense-in-depth**: Auxiliary randomness adds safety margin
- **Zero-check**: Ensures nonce ≠ 0 (would leak secret key!)

**Why This Matters:**
Nonce reuse is catastrophic: if same `k` used twice, attacker can compute secret key `x = (s₁ - s₂) / (e₂ - e₁)`

### 3. Signing Algorithm

**Schnorr Equation:**
```
Given: secret key x, public key P = x*G, message m

1. Generate nonce k
2. Compute R = k*G
3. Extract r = x-coordinate of R
4. Compute e = H(R || P || m)
5. Compute s = k + e*x mod n
6. Output signature (r, s)
```

**Challenge Computation:**
```
e = SHA256("BIP340/challenge" || r || P || m)
```

Domain separation tag prevents cross-protocol attacks.

### 4. Verification Algorithm

**Verification Equation:**
```
Given: signature (r, s), public key P, message m

1. Compute e = H(R || P || m)
2. Verify: s*G = R + e*P
```

**Mathematical Correctness:**
```
If valid: s = k + e*x
         s*G = (k + e*x)*G
         s*G = k*G + e*x*G
         s*G = R + e*P ✓
```

### 5. Batch Verification (Key Advantage!)

Schnorr's linearity enables fast batch verification:

**Batch Equation:**
```
Individual: sᵢ*G = Rᵢ + eᵢ*Pᵢ
Batch:      Σ(aᵢ*sᵢ)*G = Σ(aᵢ*Rᵢ) + Σ(aᵢ*eᵢ*Pᵢ)
```

Where `aᵢ` are random coefficients (prevents fraud proofs).

**Performance:**
- Single signature: ~3-4 scalar multiplications
- Batch of N: ~3 scalar multiplications (Straus algorithm)
- Speedup: 5-10x faster for large batches

**Why Random Coefficients?**
Without them, attacker could craft invalid signatures that cancel out in batch sum.

## Security Considerations

### Critical Requirements

1. **Constant-time operations**: All secret-dependent operations must run in constant time
2. **Zeroization**: Securely clear secrets from memory when dropped
3. **Nonce uniqueness**: Deterministic generation prevents reuse
4. **Input validation**: All public inputs must be validated (r, s ranges)
5. **No timing leaks**: Verification must be constant-time

### Security Properties

✅ **EU-CMA secure**: Existentially unforgeable under chosen message attacks
✅ **Strong unforgeability**: Can't create new signature for signed message
✅ **Random oracle model**: Security proof assumes SHA256 is a random oracle

### Known Limitations

⚠️ **Not zero-knowledge**: Signatures reveal message was signed
⚠️ **No replay protection**: Must add at application layer
⚠️ **Batch verification**: Doesn't identify which signature failed

### Security Audit Checklist

- [ ] All secret operations are constant-time
- [ ] Secrets are zeroized on drop
- [ ] Nonce is never reused
- [ ] Nonce is never zero
- [ ] All public inputs validated
- [ ] No timing leaks
- [ ] BIP340 test vectors pass
- [ ] Cross-library validation passes
- [ ] Property tests cover edge cases

## Testing Strategy

### Three-Tier Approach

**Tier 1: BIP340 Test Vectors**
- Golden tests from specification
- Verifies compliance with standard
- Tests round-trip (sign + verify)

**Tier 2: Edge Cases and Property Tests**
- `proptest` for property-based testing
- Empty messages, large messages
- Tampered signatures/keys
- Serialization round-trips

**Tier 3: Cross-Library Validation**
- Sign with our library
- Verify with `secp256k1` crate
- Ensures interoperability

### Performance Benchmarks

Using Criterion:
- `sign`: Single signing operation
- `verify`: Single verification
- `batch_verify`: Batch of 100 signatures

## Project Structure

```
schnorr-rs/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Public API exports
│   ├── keypair.rs          # KeyPair, SecretKey, PublicKey
│   ├── signature.rs        # Signature struct
│   ├── sign.rs             # Signing logic
│   ├── verify.rs           # Single and batch verification
│   ├── nonce.rs            # CRITICAL: Secure nonce generation
│   ├── challenge.rs        # BIP340 challenge computation
│   └── error.rs            # Error types
├── tests/
│   ├── integration_test.rs
│   ├── bip340_vectors.rs   # Specification compliance
│   ├── edge_cases.rs       # Boundary conditions
│   ├── cross_validation.rs # External library compatibility
│   └── property_tests.rs   # Property-based testing
├── benches/
│   └── schnorr_bench.rs    # Performance benchmarks
└── examples/
    ├── basic_sign.rs       # Simple sign/verify demo
    └── batch_verify.rs     # Batch verification demo
```

## Dependencies

```toml
[dependencies]
k256 = { version = "0.13", features = ["arithmetic", "serde"] }
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"], optional = true }
rand_core = "0.6"
zeroize = "1.5"  # For secure memory clearing
subtle = "2.4"   # For constant-time operations

[dev-dependencies]
rand = "0.8"
hex = "0.4"
proptest = "1.0"       # Property-based testing
criterion = "0.5"      # Benchmarking
secp256k1 = "0.27"     # Cross-validation only
```

## Public API

```rust
// Key generation
let keypair = KeyPair::new(&mut rng);
let public_key = keypair.public_key();

// Signing
let signature = keypair.sign(message);

// Verification
let is_valid = public_key.verify(message, &signature);

// Batch verification
let items = vec![
    (msg1, pub1, sig1),
    (msg2, pub2, sig2),
];
let is_valid = PublicKey::verify_batch(&items)?;
```

## Implementation Phases

### Phase 1: Foundation
- [ ] Set up project structure
- [ ] Implement core types (SecretKey, PublicKey, Signature)
- [ ] Implement error handling

### Phase 2: Cryptography Core
- [ ] Implement nonce generation
- [ ] Implement challenge computation
- [ ] Implement signing algorithm

### Phase 3: Verification
- [ ] Implement single signature verification
- [ ] Implement batch verification

### Phase 4: Testing
- [ ] Add BIP340 test vectors
- [ ] Add edge case tests
- [ ] Add property-based tests
- [ ] Add cross-library validation

### Phase 5: Polish
- [ ] Add benchmarks
- [ ] Write comprehensive README
- [ ] Add usage examples
- [ ] Security audit checklist

## Success Criteria

- [ ] All tests pass (unit, integration, property)
- [ ] BIP340 test vectors pass
- [ ] Cross-library validation passes
- [ ] `cargo clippy` clean
- [ ] `cargo fmt` applied
- [ ] Benchmarks show expected performance
- [ ] README includes security considerations
- [ ] Examples demonstrate key features

## References

- BIP340 Specification: https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki
- Schnorr Signature Paper: https://www.cs.berkeley.edu/~aurosch/notes/schnorr.pdf
- k256 Crate: https://docs.rs/k256/
- secp256k1 Crate: https://docs.rs/secp256k1/

## Appendix: Schnorr vs Other Schemes

**vs ECDSA:**
- Schnorr: Linear equation enables batch verification
- ECDSA: Non-linear, no batching
- Schnorr: Simpler security proofs
- Schnorr: Supports key aggregation (MuSig2)

**vs EdDSA:**
- Schnorr: More flexible (batching, adaptors)
- EdDSA: Faster (Edwards curves)
- Schnorr: Better for Bitcoin compatibility

**vs Schnorr Identification:**
- Signatures: Non-interactive (Fiat-Shamir)
- Identification: Interactive (zero-knowledge proof)
- Signatures: For authentication
- Identification: For proving knowledge of secret
