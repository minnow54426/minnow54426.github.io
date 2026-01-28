# ZK Membership Credential System - Design Document

**Date:** 2026-01-28
**Project:** zk-capstone-membership
**Type:** Zero Knowledge Proof Capstone (Week 11)

## Overview

A production-oriented Zero Knowledge Membership Credential system built on the Semaphore pattern. Users can prove they belong to an allowlist without revealing which entry, while providing a nullifier to prevent double-proving. This system is job-relevant and follows real-world ZK engineering patterns used in applications like Tornado Cash and Semaphore.

## Architecture

### Core Components

1. **Merkle Tree Allowlist** - Public root represents the allowlist
2. **Membership Circuit** - Groth16 SNARK proving membership
3. **Prover** - Generates ZK proofs from private witness
4. **Verifier** - Validates proofs against public inputs
5. **Nullifier System** - Prevents double-proving with unique identifiers

### Module Structure

```
zk-capstone-membership/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API exports
│   ├── circuits/
│   │   ├── mod.rs          # Circuit module exports
│   │   └── membership.rs   # Membership circuit definition
│   ├── prover/
│   │   ├── mod.rs          # Prover module exports
│   │   └── prove.rs        # Proof generation logic
│   ├── verifier/
│   │   ├── mod.rs          # Verifier module exports
│   │   └── verify.rs       # Proof verification logic
│   ├── types/
│   │   ├── mod.rs          # Types module exports
│   │   ├── public.rs       # Public inputs (root, nullifier)
│   │   └── witness.rs      # Witness format (leaf, path, secret)
│   └── crypto/
│       ├── mod.rs          # Crypto utilities
│       ├── poseidon.rs     # Poseidon hash implementation
│       └── merkle.rs       # Merkle tree wrapper
├── bin/
│   └── cli.rs              # CLI tool (setup/prove/verify/demo)
├── examples/
│   └── full_demo.rs        # End-to-end demo
└── tests/
    └── integration_test.rs # Integration tests
```

## Circuit Design

### Membership Circuit Constraints

The circuit implements the Semaphore pattern with Poseidon hashing:

1. **Leaf Computation:** `leaf = Poseidon(identity_secret, timestamp)`
2. **Nullifier Derivation:** `nullifier = Poseidon(identity_secret, nullifier_key)`
3. **Merkle Path Verification:** Verify leaf exists in tree at root

### Circuit Inputs

**Private (Witness):**
- `identity_secret`: Field element - user's secret identity
- `nullifier_key`: Field element - application-specific key
- `timestamp`: u64 - when leaf was created
- `merkle_path`: Vec<(FieldElement, bool)> - sibling hashes and directions
- `leaf_index`: usize - position in tree

**Public (Inputs):**
- `merkle_root`: Field element - current allowlist root
- `nullifier`: Field element - derived nullifier (prevents double-proving)

### Circuit Pseudocode

```
computed_leaf = Poseidon(identity_secret, timestamp)
computed_nullifier = Poseidon(identity_secret, nullifier_key)

verify_merkle_proof(
    leaf: computed_leaf,
    path: merkle_path,
    indices: path_indices
) == merkle_root

enforce(computed_nullifier == nullifier)
```

## Data Structures

### Types Module

**Public Inputs (`types/public.rs`):**
```rust
pub struct PublicInputs {
    pub merkle_root: FieldElement,
    pub nullifier: FieldElement,
}

pub struct Proof {
    // Groth16 proof with serialization
}

pub struct VerifyingKey {
    // Groth16 verifying key
}
```

**Witness (`types/witness.rs`):**
```rust
pub struct MembershipWitness {
    pub identity_secret: FieldElement,
    pub nullifier_key: FieldElement,
    pub timestamp: u64,
    pub merkle_path: Vec<(FieldElement, bool)>,
    pub leaf_index: usize,
}

pub struct LeafData {
    pub identity_secret: FieldElement,
    pub timestamp: u64,
}
```

### Crypto Utilities

**Poseidon Hash (`crypto/poseidon.rs`):**
- Wrapper around `ark-crypto-primitives` Poseidon
- BN254 curve with standard parameters
- `poseidon_hash(inputs: &[FieldElement]) -> FieldElement`

**Merkle Tree (`crypto/merkle.rs`):**
- Wrapper around `merkletree` crate
- Poseidon leaf hashing
- `build_tree(leaves) -> MerkleTree`
- `get_proof(index) -> MerkleProof`
- `root() -> FieldElement`

## Prover & Verifier Flow

### Prover Operations

1. **Setup (one-time):**
   ```rust
   generate_setup(tree_depth: usize) -> (ProvingKey, VerifyingKey)
   ```
   - Creates trusted setup for circuit
   - Saves keys to disk for reuse

2. **Create Demo Data:**
   ```rust
   make_demo_data(num_leaves: usize) -> DemoData
   ```
   - Generates random identities
   - Creates Merkle tree
   - Returns witness for chosen leaf

3. **Generate Proof:**
   ```rust
   create_proof(witness, public_inputs, proving_key) -> Proof
   ```
   - Takes witness and public inputs
   - Generates Groth16 proof
   - Returns serialized proof

### Verifier Operations

1. **Verify Proof:**
   ```rust
   verify_proof(proof, public_inputs, verifying_key) -> bool
   ```
   - Takes proof, public inputs, verifying key
   - Returns true if valid

2. **Check Nullifier:**
   ```rust
   check_nullifier(nullifier, used_nullifiers) -> bool
   ```
   - Application-level check
   - Prevents double-proving

### Complete Flow

```
1. Application runs setup to generate proving/verifying keys
2. Allowlist is created from user identities
3. Merkle root is published (e.g., in smart contract or config)
4. User generates proof using their secret and Merkle path
5. User submits proof + nullifier to application
6. Application verifies proof and checks nullifier is unused
7. If valid, action is allowed and nullifier is marked as used
```

## CLI Interface

### Commands

```bash
# Setup - generate keys
zk-membership setup --tree-depth 20 --output keys/

# Prove membership
zk-membership prove \
    --keys keys/ \
    --leaf-index 42 \
    --output proof.json

# Verify proof
zk-membership verify \
    --keys keys/ \
    --proof proof.json \
    --root <root>

# Full demo
zk-membership demo --num-leaves 100
```

### CLI Features

- Uses `clap` for argument parsing
- JSON I/O for proofs and public inputs
- Clear error messages with `anyhow`
- Supports both file-based and stdin/stdout operations

## Testing Strategy

### Integration Tests

1. **Happy Path:**
   - Generate 100-leaf tree
   - Create proof for random leaf
   - Verify proof succeeds
   - Check nullifier uniqueness

2. **Failure Cases:**
   - Wrong root: Modify public input, verify fails
   - Wrong path: Tamper with Merkle path, verify fails
   - Reused nullifier: Attempt double-proving, fails

### Unit Tests

- Each module has comprehensive unit tests
- Property-based tests with `proptest`
- Circuit constraint verification
- Edge cases (empty tree, single leaf, etc.)

### Benchmarks

- Proof generation time
- Verification time
- Tree construction time
- Memory usage

## Dependencies

```toml
[dependencies]
# ZK primitives
ark-groth16 = "0.4"
ark-relations = "0.4"
ark-r1cs-std = "0.4"
ark-crypto-primitives = { version = "0.4", features = ["r1cs"] }
ark-ff = "0.4"
ark-ec = "0.4"
ark-bn254 = "0.4"
ark-std = "0.4"
ark-serialize = "0.4"

# Merkle tree
merkletree = "0.23"

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Randomness
rand = "0.8"

# CLI
clap = { version = "4.0", features = ["derive"] }

[dev-dependencies]
proptest = "1.0"
criterion = "0.5"
```

## Success Criteria

- ✓ End-to-end demo works from clean checkout
- ✓ Tests cover happy path
- ✓ Tests cover at least 2 failure cases (wrong root, wrong path)
- ✓ Clean Rust engineering following API guidelines
- ✓ Production-ready code quality

## Future Enhancements (Optional)

- **Revocation:** Change root to invalidate users
- **Batch proving:** Prove multiple memberships at once
- **Aggregation:** Combine multiple proofs into one
- **App scoping:** External nullifiers for multiple apps
- **Smart contract integration:** Solidity verifier
