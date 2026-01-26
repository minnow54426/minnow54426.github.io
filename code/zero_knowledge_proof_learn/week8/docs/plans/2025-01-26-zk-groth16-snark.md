# zk-groth16-snark Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a production-ready Groth16 SNARK library with three circuits (identity, membership, privacy) demonstrating real-world ZK applications with comprehensive tests, benchmarks, and documentation.

**Architecture:** Trait-based circuit abstraction with shared Groth16 infrastructure. Three independent circuit implementations (hash preimage, Merkle membership, range proof) using arkworks-rs libraries. Clean separation between core infrastructure (trait, error handling, Groth16 pipeline) and circuit-specific logic.

**Tech Stack:** Rust 2021, arkworks-rs (ark-groth16, ark-relations, ark-r1cs-std, ark-crypto-primitives), serde/bincode for serialization, thiserror/anyhow for errors, criterion for benchmarks, proptest for property-based testing.

---

## Task 1: Core Error Types

**Files:**
- Modify: `code/src/error.rs`

**Step 1: Write test for error creation and display**

Create: `code/tests/error_tests.rs`

```rust
use zk_groth16_snark::{Error, ErrorKind};

#[test]
fn test_error_display() {
    let err = Error::Setup(SetupError::ParametersAlreadyExist);
    assert!(err.to_string().contains("Setup"));
}

#[test]
fn test_error_kind() {
    let err = Error::Verify(VerifyError::InvalidProof);
    assert!(matches!(err.kind(), ErrorKind::InvalidProof));
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test error_tests`

Expected: COMPILER ERROR - types not defined yet

**Step 3: Implement error types**

Modify: `code/src/error.rs`

```rust
use std::fmt;
use thiserror::Error;

/// Main error type for the library
#[derive(Error, Debug)]
pub enum Error {
    #[error("Circuit error: {0}")]
    Circuit(#[from] CircuitError),

    #[error("Setup error: {0}")]
    Setup(#[from] SetupError),

    #[error("Prove error: {0}")]
    Prove(#[from] ProveError),

    #[error("Verify error: {0}")]
    Verify(#[from] VerifyError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] SerializationError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::Circuit(CircuitError::Identity(_)) => ErrorKind::InvalidWitness,
            Error::Setup(SetupError::ParametersAlreadyExist) => ErrorKind::ParametersAlreadyExist,
            Error::Prove(ProveError::ProofCreationFailed) => ErrorKind::ProofCreationFailed,
            Error::Verify(VerifyError::InvalidProof) => ErrorKind::InvalidProof,
            _ => ErrorKind::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    InvalidWitness,
    ConstraintViolation,
    PublicInputMismatch,
    ParametersAlreadyExist,
    InsufficientEntropy,
    SetupFailed,
    WitnessGenerationFailed,
    ProofCreationFailed,
    InvalidProof,
    ProofVerificationFailed,
    PublicInputsIncorrect,
    DeserializationFailed,
    VersionMismatch,
    Unknown,
}

#[derive(Error, Debug)]
pub enum CircuitError {
    #[error("Identity circuit error: {0}")]
    Identity(#[from] IdentityError),

    #[error("Membership circuit error: {0}")]
    Membership(#[from] MembershipError),

    #[error("Privacy circuit error: {0}")]
    Privacy(#[from] PrivacyError),
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Parameters already exist")]
    ParametersAlreadyExist,

    #[error("Insufficient entropy")]
    InsufficientEntropy,

    #[error("Setup failed")]
    SetupFailed,
}

#[derive(Error, Debug)]
pub enum ProveError {
    #[error("Witness generation failed")]
    WitnessGenerationFailed,

    #[error("Proof creation failed")]
    ProofCreationFailed,
}

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Invalid proof")]
    InvalidProof,

    #[error("Proof verification failed")]
    ProofVerificationFailed,

    #[error("Public inputs incorrect")]
    PublicInputsIncorrect,
}

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Deserialization failed")]
    DeserializationFailed,

    #[error("Version mismatch")]
    VersionMismatch,
}

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Invalid preimage length")]
    InvalidPreimageLength,

    #[error("Hash mismatch")]
    HashMismatch,
}

#[derive(Error, Debug)]
pub enum MembershipError {
    #[error("Invalid path length")]
    InvalidPathLength,

    #[error("Root mismatch")]
    RootMismatch,

    #[error("Leaf not found")]
    LeafNotFound,
}

#[derive(Error, Debug)]
pub enum PrivacyError {
    #[error("Value out of range")]
    ValueOutOfRange,

    #[error("Invalid bit width")]
    InvalidBitWidth,
}

pub type Result<T> = std::result::Result<T, Error>;
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test error_tests`

Expected: PASS

**Step 5: Commit**

```bash
cd code
git add src/error.rs tests/error_tests.rs
git commit -m "feat: implement core error types with thiserror"
```

---

## Task 2: Groth16Circuit Trait Definition

**Files:**
- Modify: `code/src/circuit.rs`

**Step 1: Write test for trait usage**

Create: `code/tests/circuit_trait_tests.rs`

```rust
use zk_groth16_snark::circuit::Groth16Circuit;
use ark_bls12_381::Fr;

#[test]
fn test_circuit_name() {
    struct DummyCircuit;
    impl Groth16Circuit<Fr> for DummyCircuit {
        fn circuit_name() -> &'static str {
            "dummy"
        }

        type PublicInputs = Vec<Fr>;
        type Witness = Vec<Fr>;

        fn generate_constraints(
            _cs: ark_relations::r1cs::ConstraintMetavariable<Fr>,
            _witness: &Self::Witness,
        ) -> zk_groth16_snark::Result<()> {
            Ok(())
        }

        fn generate_witness(&self) -> zk_groth16_snark::Result<Self::Witness> {
            Ok(vec![])
        }

        fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
            vec![]
        }
    }

    assert_eq!(DummyCircuit::circuit_name(), "dummy");
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test circuit_trait_tests`

Expected: COMPILER ERROR - trait not defined

**Step 3: Implement Groth16Circuit trait**

Modify: `code/src/circuit.rs`

```rust
use ark_ff::Field;
use ark_relations::r1cs::ConstraintMetavariable;
use serde::{Deserialize, Serialize};
use crate::{CircuitError, Result};

/// Core trait that all Groth16 circuits must implement
pub trait Groth16Circuit<F: Field> {
    /// Circuit identifier for debugging/serialization
    fn circuit_name() -> &'static str;

    /// Public inputs for verification
    type PublicInputs: Clone + Serialize + for<'de> Deserialize<'de>;

    /// Private witness (known only to prover)
    type Witness: Clone + Serialize + for<'de> Deserialize<'de>;

    /// Generate constraint system
    fn generate_constraints(
        cs: ConstraintMetavariable<F>,
        witness: &Self::Witness,
    ) -> Result<()>;

    /// Create witness from private inputs
    fn generate_witness(&self) -> Result<Self::Witness>;

    /// Extract public inputs from witness
    fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs;
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test circuit_trait_tests`

Expected: PASS

**Step 5: Commit**

```bash
cd code
git add src/circuit.rs tests/circuit_trait_tests.rs
git commit -m "feat: define Groth16Circuit trait"
```

---

## Task 3: Serialization Utilities

**Files:**
- Create: `code/src/utils/serialization.rs`
- Create: `code/src/utils/mod.rs`

**Step 1: Write test for serialization**

Create: `code/tests/serialization_tests.rs`

```rust
use zk_groth16_snark::utils::serialization::{serialize_proof, deserialize_proof};

#[test]
fn test_proof_serialization() {
    let proof_bytes = vec![1u8; 288]; // Mock proof
    let serialized = serialize_proof(&proof_bytes).unwrap();
    let deserialized = deserialize_proof(&serialized).unwrap();
    assert_eq!(proof_bytes, deserialized);
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test serialization_tests`

Expected: COMPILER ERROR - module not found

**Step 3: Implement serialization utilities**

Create: `code/src/utils/serialization.rs`

```rust
use crate::{Result, SerializationError};
use bincode::{deserialize, serialize};

/// Serialize proof to bytes
pub fn serialize_proof<T: serde::Serialize>(proof: &T) -> Result<Vec<u8>> {
    serialize(proof).map_err(|e| SerializationError::DeserializationFailed.into())
}

/// Deserialize proof from bytes
pub fn deserialize_proof<T: for<'de> serde::Deserialize<'de>>(bytes: &[u8]) -> Result<T> {
    deserialize(bytes).map_err(|e| SerializationError::DeserializationFailed.into())
}

/// Serialize proving key
pub fn serialize_pk<T: serde::Serialize>(pk: &T) -> Result<Vec<u8>> {
    serialize(pk).map_err(|e| SerializationError::DeserializationFailed.into())
}

/// Deserialize proving key
pub fn deserialize_pk<T: for<'de> serde::Deserialize<'de>>(bytes: &[u8]) -> Result<T> {
    deserialize(bytes).map_err(|e| SerializationError::DeserializationFailed.into())
}
```

Create: `code/src/utils/mod.rs`

```rust
pub mod serialization;
pub mod fields;

pub use serialization::*;
```

Create: `code/src/utils/fields.rs`

```rust
use ark_ff::Field;
use ark_std::vec::Vec;

/// Convert bytes to field elements
pub fn bytes_to_field_elements<F: Field>(bytes: &[u8]) -> Vec<F> {
    // TODO: Implement proper conversion
    vec![]
}

/// Convert field elements to bytes
pub fn field_elements_to_bytes<F: Field>(elements: &[F]) -> Vec<u8> {
    // TODO: Implement proper conversion
    vec![]
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test serialization_tests`

Expected: PASS

**Step 5: Commit**

```bash
cd code
git add src/utils/ tests/serialization_tests.rs
git commit -m "feat: add serialization utilities"
```

---

## Task 4: Groth16 Infrastructure - Setup Function

**Files:**
- Modify: `code/src/groth16.rs`

**Step 1: Write test for setup**

Create: `code/tests/groth16_setup_tests.rs`

```rust
use zk_groth16_snark::groth16::setup;
use ark_bls12_381::Fr;

struct TestCircuit;

impl zk_groth16_snark::Groth16Circuit<Fr> for TestCircuit {
    fn circuit_name() -> &'static str {
        "test"
    }

    type PublicInputs = Vec<Fr>;
    type Witness = Vec<Fr>;

    fn generate_constraints(
        _cs: ark_relations::r1cs::ConstraintMetavariable<Fr>,
        _witness: &Self::Witness,
    ) -> zk_groth16_snark::Result<()> {
        Ok(())
    }

    fn generate_witness(&self) -> zk_groth16_snark::Result<Self::Witness> {
        Ok(vec![])
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        vec![]
    }
}

#[test]
#[ignore] // Ignore until we have proper implementation
fn test_setup() {
    let circuit = TestCircuit;
    let (_pk, _vk) = setup(&circuit).unwrap();
    // TODO: Verify keys are generated
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test groth16_setup_tests`

Expected: COMPILER ERROR - setup function not defined

**Step 3: Implement setup function stub**

Modify: `code/src/groth16.rs`

```rust
use crate::{Error, Groth16Circuit, Result, SetupError};
use ark_ec::PairingEngine;
use ark_groth16::{ProvingKey, VerifyingKey};
use ark_std::marker::PhantomData;

/// Generate Groth16 proving and verifying keys for a circuit
pub fn setup<C, F>(
    circuit: &C,
) -> Result<(ProvingKey<F>, VerifyingKey<F>)>
where
    C: Groth16Circuit<F>,
    F: PairingEngine,
{
    // TODO: Implement actual Groth16 setup
    // For now, return error to guide implementation
    Err(Error::Setup(SetupError::SetupFailed))
}
```

**Step 4: Run test to verify it compiles**

Run: `cd code && cargo test groth16_setup_tests -- --ignored`

Expected: COMPILES but test fails (we'll implement properly in next task)

**Step 5: Commit**

```bash
cd code
git add src/groth16.rs tests/groth16_setup_tests.rs
git commit -m "feat: add Groth16 setup function stub"
```

---

## Task 5: Identity Circuit - Basic Structure

**Files:**
- Modify: `code/src/identity/mod.rs`

**Step 1: Write test for IdentityCircuit creation**

Create: `code/tests/identity_basic_tests.rs`

```rust
use zk_groth16_snark::identity::IdentityCircuit;
use ark_bls12_381::Fr;

#[test]
fn test_identity_circuit_creation() {
    let hash = [0u8; 32];
    let circuit = IdentityCircuit::new(hash);
    assert_eq!(circuit.circuit_name(), "identity");
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test identity_basic_tests`

Expected: COMPILER ERROR - IdentityCircuit not defined

**Step 3: Implement IdentityCircuit structure**

Modify: `code/src/identity/mod.rs`

```rust
use crate::{Groth16Circuit, Result};
use ark_bls12_381::Fr as Fp;
use ark_ff::Field;
use ark_std::vec::Vec;

pub struct IdentityCircuit {
    pub hash: [u8; 32],
}

impl IdentityCircuit {
    pub fn new(hash: [u8; 32]) -> Self {
        Self { hash }
    }
}

impl Groth16Circuit<Fp> for IdentityCircuit {
    fn circuit_name() -> &'static str {
        "identity"
    }

    type PublicInputs = [Fp; 32];
    type Witness = Vec<u8>;

    fn generate_constraints(
        _cs: ark_relations::r1cs::ConstraintMetavariable<Fp>,
        _witness: &Self::Witness,
    ) -> Result<()> {
        // TODO: Implement SHA-256 constraints
        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        // TODO: Generate witness from preimage
        Ok(vec![])
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        // TODO: Convert hash to field elements
        [Fp::from(0u32); 32]
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test identity_basic_tests`

Expected: PASS

**Step 5: Commit**

```bash
cd code
git add src/identity/mod.rs tests/identity_basic_tests.rs
git commit -m "feat: add IdentityCircuit basic structure"
```

---

## Task 6: Privacy Circuit - Basic Structure

**Files:**
- Modify: `code/src/privacy/mod.rs`

**Step 1: Write test for PrivacyCircuit creation**

Create: `code/tests/privacy_basic_tests.rs`

```rust
use zk_groth16_snark::privacy::PrivacyCircuit;
use ark_bls12_381::Fr;

#[test]
fn test_privacy_circuit_creation() {
    let min = Fr::from(0u32);
    let max = Fr::from(100u32);
    let circuit = PrivacyCircuit::new(min, max);
    assert_eq!(circuit.circuit_name(), "privacy");
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test privacy_basic_tests`

Expected: COMPILER ERROR - PrivacyCircuit not defined

**Step 3: Implement PrivacyCircuit structure**

Modify: `code/src/privacy/mod.rs`

```rust
use crate::{Groth16Circuit, Result};
use ark_bls12_381::Fr as Fp;
use ark_ff::Field;

pub struct PrivacyCircuit {
    pub min: Fp,
    pub max: Fp,
}

impl PrivacyCircuit {
    pub fn new(min: Fp, max: Fp) -> Self {
        Self { min, max }
    }
}

impl Groth16Circuit<Fp> for PrivacyCircuit {
    fn circuit_name() -> &'static str {
        "privacy"
    }

    type PublicInputs = (Fp, Fp); // (min, max)
    type Witness = Fp; // secret value

    fn generate_constraints(
        _cs: ark_relations::r1cs::ConstraintMetavariable<Fp>,
        _witness: &Self::Witness,
    ) -> Result<()> {
        // TODO: Implement range proof constraints
        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        // TODO: Generate witness from secret value
        Ok(Fp::from(0u32))
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        (self.min, self.max)
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test privacy_basic_tests`

Expected: PASS

**Step 5: Commit**

```bash
cd code
git add src/privacy/mod.rs tests/privacy_basic_tests.rs
git commit -m "feat: add PrivacyCircuit basic structure"
```

---

## Task 7: Membership Circuit - Basic Structure

**Files:**
- Modify: `code/src/membership/mod.rs`

**Step 1: Write test for MembershipCircuit creation**

Create: `code/tests/membership_basic_tests.rs`

```rust
use zk_groth16_snark::membership::MembershipCircuit;
use ark_bls12_381::Fr;

#[test]
fn test_membership_circuit_creation() {
    let root = [0u8; 32];
    let circuit = MembershipCircuit::new(root);
    assert_eq!(circuit.circuit_name(), "membership");
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test membership_basic_tests`

Expected: COMPILER ERROR - MembershipCircuit not defined

**Step 3: Implement MembershipCircuit structure**

Modify: `code/src/membership/mod.rs`

```rust
use crate::{Groth16Circuit, Result};
use ark_bls12_381::Fr as Fp;
use ark_ff::Field;
use ark_std::vec::Vec;

pub struct MembershipCircuit {
    pub root: [u8; 32],
}

impl MembershipCircuit {
    pub fn new(root: [u8; 32]) -> Self {
        Self { root }
    }
}

impl Groth16Circuit<Fp> for MembershipCircuit {
    fn circuit_name() -> &'static str {
        "membership"
    }

    type PublicInputs = [Fp; 32]; // root hash
    type Witness = (Vec<u8>, Vec<Vec<u8>>); // (leaf, path)

    fn generate_constraints(
        _cs: ark_relations::r1cs::ConstraintMetavariable<Fp>,
        _witness: &Self::Witness,
    ) -> Result<()> {
        // TODO: Implement Merkle path verification constraints
        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        // TODO: Generate witness from leaf and path
        Ok((vec![], vec![]))
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        // TODO: Convert root to field elements
        [Fp::from(0u32); 32]
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test membership_basic_tests`

Expected: PASS

**Step 5: Commit**

```bash
cd code
git add src/membership/mod.rs tests/membership_basic_tests.rs
git commit -m "feat: add MembershipCircuit basic structure"
```

---

## Task 8: First Example - Identity Proof Demo

**Files:**
- Create: `code/examples/identity_proof.rs`

**Step 1: Create example file**

Create: `code/examples/identity_proof.rs`

```rust
use zk_groth16_snark::identity::IdentityCircuit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Identity Circuit Demo ===");
    println!("Prove knowledge of a password without revealing it\n");

    // Setup: Define the public hash
    let password = "my_secret_password";
    let password_hash = sha256(password.as_bytes());
    println!("Password hash: {:02x?}", password_hash);

    // Create circuit
    let circuit = IdentityCircuit::new(password_hash);
    println!("Circuit name: {}", circuit.circuit_name());

    println!("\n✓ Identity circuit created successfully!");
    println!("  (Full implementation coming soon)")

    // TODO: Add setup, prove, verify when implemented

    Ok(())
}

fn sha256(data: &[u8]) -> [u8; 32] {
    use sha2::Sha256;
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
```

**Step 2: Test the example runs**

Run: `cd code && cargo run --example identity_proof`

Expected: RUNS successfully (even though full functionality isn't implemented)

**Step 3: Commit**

```bash
cd code
git add examples/identity_proof.rs
git commit -m "feat: add identity proof example"
```

---

## Task 9: Full Demo Example

**Files:**
- Create: `code/examples/full_demo.rs`

**Step 1: Create full demo**

Create: `code/examples/full_demo.rs`

```rust
use zk_groth16_snark::{identity::IdentityCircuit, Groth16Circuit};
use ark_bls12_381::Fr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Groth16 SNARK Library Demo ===\n");

    // Identity Circuit
    println!("[1/3] Identity Circuit (Hash Preimage)");
    let hash = [0u8; 32];
    let circuit = IdentityCircuit::new(hash);
    println!("  Circuit: {}", circuit.circuit_name());
    println!("  Status: Structure complete, constraints pending\n");

    // Membership Circuit
    println!("[2/3] Membership Circuit (Merkle Tree)");
    // TODO: Add when implemented
    println!("  Status: Coming soon\n");

    // Privacy Circuit
    println!("[3/3] Privacy Circuit (Range Proof)");
    // TODO: Add when implemented
    println!("  Status: Coming soon\n");

    println!("Demo complete! 🎉");
    println!("(Full functionality under development)");

    Ok(())
}
```

**Step 2: Test the demo runs**

Run: `cd code && cargo run --example full_demo`

Expected: RUNS successfully

**Step 3: Commit**

```bash
cd code
git add examples/full_demo.rs
git commit -m "feat: add full demo example"
```

---

## Task 10: Verify Build and Tests

**Files:**
- None (verification step)

**Step 1: Run all tests**

Run: `cd code && cargo test`

Expected: All basic tests PASS

**Step 2: Check clippy**

Run: `cd code && cargo clippy -- -D warnings`

Expected: May have warnings (acceptable at this stage)

**Step 3: Format code**

Run: `cd code && cargo fmt`

Expected: Reformats code

**Step 4: Commit final cleanup**

```bash
cd code
git add -A
git commit -m "chore: format code and verify basic structure"
```

---

## Remaining Implementation (Future Tasks)

These tasks should be completed in order, each following the same TDD pattern:

### Phase 2: Complete Groth16 Infrastructure
- Implement actual `setup()` using ark-groth16
- Add `prove()` function
- Add `verify()` function
- Add parameter serialization/deserialization
- Test full pipeline end-to-end

### Phase 3: Complete Identity Circuit
- Implement SHA-256 constraint generation
- Add proper witness generation
- Write comprehensive tests (valid, invalid cases)
- Add property-based tests with proptest

### Phase 4: Complete Privacy Circuit
- Implement range proof constraints (binary decomposition)
- Add bit constraint gadgets
- Test boundary values
- Add benchmarks

### Phase 5: Complete Membership Circuit
- Implement Merkle path verification constraints
- Add hash gadgets (start with SHA-256, consider Poseidon)
- Test with various tree depths
- Add performance benchmarks

### Phase 6: Examples and Documentation
- Complete `identity_proof.rs` with full pipeline
- Complete `membership_proof.rs`
- Complete `privacy_proof.rs`
- Add inline documentation to all public APIs
- Verify README examples work

### Phase 7: Testing
- Add property-based tests for all circuits
- Add integration tests for full pipeline
- Add serialization tests
- Test error conditions

### Phase 8: Benchmarks
- Implement `circuit_benchmarks.rs`
- Implement `comparison_benchmarks.rs`
- Document performance characteristics
- Update docs with benchmark results

### Phase 9: Final Polish
- Run `cargo clippy` - fix all warnings
- Ensure `cargo test` passes 100%
- Verify `cargo doc --open` builds cleanly
- Update all documentation
- Final review against design document

---

## Implementation Notes

**Key Dependencies:**
- `ark-groth16 0.4` - Groth16 proving system
- `ark-relations 0.4` - Constraint system traits
- `ark-r1cs-std 0.4` - R1CS standard library
- `ark-crypto-primitives 0.4` - Cryptographic gadgets (SHA-256, Poseidon, etc.)
- `ark-ff 0.4`, `ark-ec 0.4`, `ark-bls12-381 0.4` - Field and curve primitives
- `serde 1.0`, `bincode 1.3` - Serialization
- `thiserror 1.0`, `anyhow 1.0` - Error handling
- `proptest 1.0` - Property-based testing
- `criterion 0.5` - Benchmarking

**Testing Strategy:**
1. Write test first (TDD)
2. Verify test fails
3. Implement minimal code to pass
4. Verify test passes
5. Commit immediately
6. Run `cargo test` frequently

**Common Patterns:**
- Use `?` operator for error propagation
- Use `thiserror` for error derives
- Use `serde` derives for serializable types
- Add doc comments to all public items
- Commit frequently (every 2-5 minutes)

**Troubleshooting:**
- If compilation fails: check arkworks version compatibility
- If tests fail: check constraint generation logic
- If benchmarks fail: ensure criterion is properly configured
- If serialization fails: verify serde derives are present

---

**Next Steps:**
After this plan is ready, choose execution approach:
1. Subagent-driven (this session) - use @superpowers:subagent-driven-development
2. Parallel session (separate) - use @superpowers:executing-plans in new session
