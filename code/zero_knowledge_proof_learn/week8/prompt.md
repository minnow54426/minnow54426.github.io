# Week 8 Design: First End-to-End Groth16 SNARK Library

## Project Overview

**Repository**: `zk-groth16-snark` (new standalone project)

**Purpose**: Demonstrate practical Groth16 SNARK implementation across three real-world applications, showcasing production-grade Rust engineering with clean abstractions, comprehensive testing, and portfolio-ready documentation.

**Three Circuit Themes**:

1. **Identity Circuit** (Hash Preimage): Prove knowledge of a secret value `x` where `sha256(x) = public_hash`. Applications: Password authentication, commitment schemes, digital identity verification.

2. **Membership Circuit** (Merkle Tree): Prove a leaf exists in a Merkle tree with known root without revealing which leaf or the full path. Applications: Allowlist proofs, anonymous credentials, privacy-preserving membership.

3. **Privacy Circuit** (Range Proof): Prove a value is within a specific range without revealing the actual value. Applications: Financial privacy, age verification, tiered access control.

**Learning Goals**:
- Master the complete Groth16 pipeline: trusted setup → prove → verify
- Design reusable trait-based abstractions for ZK circuits
- Build production-ready Rust libraries with proper error handling
- Compare different constraint patterns and their performance

**Key Deliverables**:
- Trait-based circuit API that all three circuits implement
- Comprehensive test suite with property-based testing
- Criterion benchmarks comparing proving/verification time and proof sizes
- Full documentation suite (README + 3 circuit-specific docs)
- Extensive examples demonstrating each circuit in context

---

## Code Architecture and Organization

### Library Structure

```
zk-groth16-snark/
├── Cargo.toml
├── README.md
├── docs/
│   ├── identity-circuit.md
│   ├── membership-circuit.md
│   ├── privacy-circuit.md
│   └── error-handling.md
├── src/
│   ├── lib.rs              # Public API exports
│   ├── circuit.rs          # Groth16Circuit trait definition
│   ├── groth16.rs          # Setup/prove/verify infrastructure
│   ├── error.rs            # Error types and ErrorKind enum
│   ├── identity/
│   │   ├── mod.rs          # IdentityCircuit implementation
│   │   └── circuit.rs      # Hash preimage constraints
│   ├── membership/
│   │   ├── mod.rs          # MembershipCircuit implementation
│   │   └── circuit.rs      # Merkle tree constraints
│   ├── privacy/
│   │   ├── mod.rs          # PrivacyCircuit implementation
│   │   └── circuit.rs      # Range proof constraints
│   └── utils/
│       ├── serialization.rs
│       └── fields.rs
├── examples/
│   ├── identity_proof.rs
│   ├── membership_proof.rs
│   ├── privacy_proof.rs
│   └── full_demo.rs
├── benches/
│   ├── circuit_benchmarks.rs
│   └── comparison_benchmarks.rs
└── tests/
    ├── identity_tests.rs
    ├── membership_tests.rs
    ├── privacy_tests.rs
    └── integration_tests.rs
```

### Core Trait Design

**`Groth16Circuit` Trait** (in `src/circuit.rs`):

```rust
pub trait Groth16Circuit<F: Field> {
    /// Circuit identifier for debugging/serialization
    fn circuit_name() -> &'static str;

    /// Public inputs for verification
    type PublicInputs: Clone + Serialize + Deserialize;

    /// Private witness (known only to prover)
    type Witness: Clone + Serialize + Deserialize;

    /// Generate constraint system
    fn generate_constraints(
        cs: ConstraintMetavariable<F>,
        witness: &Self::Witness,
    ) -> Result<(), CircuitError>;

    /// Create witness from private inputs
    fn generate_witness(&self) -> Result<Self::Witness, Error>;

    /// Extract public inputs from witness
    fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs;
}
```

This trait abstraction allows all three circuits to share common setup/prove/verify logic while maintaining their specific constraint implementations.

---

## Groth16 Pipeline Implementation

### Shared Infrastructure (`src/groth16.rs`)

The core Groth16 lifecycle (setup → prove → verify) is implemented once and reused by all circuits through the trait.

**Setup Phase**:

```rust
pub fn setup<C, F>(
    circuit: &C,
) -> Result<(ProvingKey<F>, VerifyingKey<F>), SetupError>
where
    C: Groth16Circuit<F>,
    F: Field,
{
    let mut cs = ConstraintMetavar::new();
    C::generate_constraints(cs, &circuit.generate_witness()?)?;

    // Groth16 trusted setup using ark-groth16
    let pk = Groth16::generate_random_parameters_with_reduction(circuit)?;
    let vk = pk.vk.clone();

    // Serialize and save to disk
    save_params(&pk, &vk, C::circuit_name())?;

    Ok((pk, vk))
}
```

**Prove Phase**:

```rust
pub fn prove<C, F>(
    pk: &ProvingKey<F>,
    witness: &C::Witness,
) -> Result<Proof<F>, ProveError>
where
    C: Groth16Circuit<F>,
    F: Field,
{
    let public_inputs = C::public_inputs(witness);

    let proof = Groth16::prove(
        pk,
        circuit_with_witness(witness),
        &public_inputs,
        &mut rng(),
    )?;

    Ok(proof)
}
```

**Verify Phase**:

```rust
pub fn verify<F>(
    vk: &VerifyingKey<F>,
    public_inputs: &F::Vector,
    proof: &Proof<F>,
) -> Result<bool, VerifyError>
where
    F: Field,
{
    let valid = Groth16::verify(
        vk,
        public_inputs,
        proof,
    )?;

    Ok(valid)
}
```

**Serialization Support**: All artifacts (PK, VK, proofs) support binary serialization via `serde` + `bincode`, with helper functions for saving/loading from disk. Each circuit stores its parameters in a dedicated directory (e.g., `params/identity/`).

---

## Circuit Implementations

### 1. Identity Circuit (Hash Preimage)

**Statement**: Prove knowledge of `x` where `sha256(x) = public_hash` without revealing `x`.

**Constraints** (`src/identity/circuit.rs`):
- Uses `ark-crypto-primitives` SHA-256 gadget
- Creates 256-bit hash input as witness (private)
- Public input: 256-bit hash output
- Constraint complexity: ~25,000 constraints (SHA-256 is heavy)

**Public Inputs**: `[F; 32]` (hash output)
**Witness**: `[u8; 32]` (preimage, converted to field elements)

**Use Case Examples**:
- Password authentication: Prove you know a password without transmitting it
- Commitment schemes: Reveal a value you committed to earlier
- Digital identity: Prove ownership of a secret identifier

### 2. Membership Circuit (Merkle Tree)

**Statement**: Prove a leaf belongs to a Merkle tree with known root, without revealing the leaf or full path.

**Constraints** (`src/membership/circuit.rs`):
- Fixed-depth Merkle tree (depth = 8 or 16 for learning)
- Hash path verification in-circuit using Poseidon or SHA-256
- Public inputs: root hash + optional leaf index
- Witness: leaf value + sibling hashes along path
- Constraint complexity: ~depth × hash_constraints

**Public Inputs**: `[F; 32]` (root) + optional `F` (index)
**Witness**: `[u8; 32]` (leaf) + `[[F; 32]; depth]` (path)

**Use Case Examples**:
- Allowlist proofs: Prove you're whitelisted without revealing which address
- Anonymous credentials: Show membership without revealing identity
- Privacy-preserving voting: Verify eligibility without revealing voter

### 3. Privacy Circuit (Range Proof)

**Statement**: Prove `value` is in range `[min, max]` without revealing `value`.

**Constraints** (`src/privacy/circuit.rs`):
- Decompose value into binary representation
- Check each bit is 0 or 1 (range constraints)
- Compare against min/max bounds
- Public inputs: min, max bounds
- Witness: actual value
- Constraint complexity: ~bit_width × 4 constraints

**Public Inputs**: `F` (min) + `F` (max)
**Witness**: `F` (secret value)

**Use Case Examples**:
- Age verification: Prove you're 18+ without revealing birthdate
- Financial privacy: Prove sufficient funds without revealing balance
- Tiered access: Prove qualification level without revealing exact score

---

## Documentation Structure

### Main README.md

**Sections**:
1. **Project Overview** - What this library demonstrates and why Groth16 matters
2. **Quick Start** - One command to run the demo (`cargo run --example full_demo`)
3. **Circuit Overview** - Brief description of all three circuits and their use cases
4. **Installation** - Dependencies and platform requirements
5. **Library API** - How to use the library in your own code
6. **Running Examples** - Commands for each circuit demonstration
7. **Testing** - How to run tests and benchmarks
8. **Performance Notes** - Expected proving/verification times
9. **Further Reading** - Links to Groth16 paper, arkworks docs, etc.

### Circuit-Specific Documentation

**`docs/identity-circuit.md`**:
- **Problem Statement**: What hash preimage proofs solve and why they matter
- **Circuit Design**: How SHA-256 constraints work in R1CS
- **Public vs Private**: Clear breakdown of what's revealed and what stays secret
- **Constraint Analysis**: Why SHA-256 is expensive (~25K constraints) and optimization considerations
- **Real-World Applications**: Password auth, commitments, digital identity
- **Example Walkthrough**: Line-by-line explanation of `examples/identity_proof.rs`
- **Limitations**: Hash choice, input size, future improvements

**`docs/membership-circuit.md`**:
- **Problem Statement**: Merkle membership proofs and privacy use cases
- **Circuit Design**: In-circuit Merkle path verification, fixed-depth rationale
- **Public vs Private**: Root revealed, leaf and path hidden
- **Constraint Analysis**: Depth complexity, hash function trade-offs (Poseidon vs SHA)
- **Real-World Applications**: Allowlists, Semaphore-like anonymity, voting
- **Example Walkthrough**: Full flow from tree construction to verification
- **Security Considerations**: What assumptions the verifier must trust

**`docs/privacy-circuit.md`**:
- **Problem Statement**: Range proofs and privacy-preserving verification
- **Circuit Design**: Binary decomposition, bit constraints, comparison logic
- **Public vs Private**: Bounds revealed, value hidden
- **Constraint Analysis**: Linear scaling with bit width, optimization opportunities
- **Real-World Applications**: Age verification, financial privacy, tiered access
- **Example Walkthrough**: Proving age ≥ 18 without revealing birth year
- **Extensions**: How to support more complex predicates (e.g., value ranges)

**`docs/error-handling.md`**:
- Complete catalog of error types with causes and fixes
- Common debugging scenarios (e.g., "Why does my proof fail to verify?")
- Troubleshooting guide for setup/prove/verify failures

---

## Testing and Benchmarking Strategy

### Test Coverage (`tests/`)

**Unit Tests** (per circuit):
- **Happy path**: Valid witness generates valid proof that verifies
- **Wrong witness**: Invalid witness fails to generate proof or verification fails
- **Tampered public inputs**: Modifying public inputs breaks verification
- **Edge cases**: Boundary values (e.g., min/max for range proofs)
- **Serialization**: PK/VK/proof serialize and deserialize correctly

**Integration Tests** (`tests/integration_tests.rs`):
- **Full pipeline**: Setup → prove → serialize proof → deserialize → verify
- **Cross-circuit**: Ensure different circuits don't interfere
- **Parameter reuse**: Load saved PK/VK and use for multiple proofs
- **Error recovery**: Verify error messages are helpful and accurate

**Property-Based Tests** (using `proptest`):
- **Identity**: For random preimages, hash(preimage) = public_hash always verifies
- **Membership**: Random trees with random valid paths always verify
- **Privacy**: Random values within range always verify, outside always fail

### Benchmark Suite (`benches/`)

**Performance Metrics**:
- Setup time (ms) - one-time cost per circuit
- Proving time (ms) - per-proof generation
- Verification time (μs) - per-proof verification
- Proof size (bytes) - network/storage cost
- PK size (bytes) - storage overhead
- VK size (bytes) - verifier storage

**Criterion Benchmarks** (`benches/circuit_benchmarks.rs`):
```rust
bench_identity_setup!(...);     // Benchmark setup phase
bench_identity_prove!(...);     // Benchmark proving
bench_identity_verify!(...);    // Benchmark verification
// Repeat for membership and privacy
```

**Comparative Benchmarks** (`benches/comparison_benchmarks.rs`):
- Proving time comparison across all three circuits
- Verification time comparison
- Constraint count vs proving time correlation
- Proof size comparison (useful for rollup contexts)

### Test Organization

Each test file follows this pattern:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_proof_verifies() {
        // Test setup, execution, assertion
    }

    #[test]
    fn invalid_witness_fails() {
        // Test error conditions
    }

    // Property tests
    proptest! {
        #[test]
        fn prop_random_preimages_verify(preimage in any::<[u8; 32]>()) {
            // Property-based verification
        }
    }
}
```

---

## Error Handling System

### Error Type Hierarchy (`src/error.rs`)

**Main Error Enum**:
```rust
pub enum Error {
    Circuit(CircuitError),
    Setup(SetupError),
    Prove(ProveError),
    Verify(VerifyError),
    Serialization(SerializationError),
    Io(io::Error),
}
```

**ErrorKind for Categorization**:
```rust
pub enum ErrorKind {
    // Circuit errors
    InvalidWitness,
    ConstraintViolation,
    PublicInputMismatch,

    // Setup errors
    ParametersAlreadyExist,
    InsufficientEntropy,
    SetupFailed,

    // Prove errors
    WitnessGenerationFailed,
    ProofCreationFailed,

    // Verify errors
    InvalidProof,
    ProofVerificationFailed,
    PublicInputsIncorrect,

    // Serialization errors
    DeserializationFailed,
    VersionMismatch,
}
```

**Circuit-Specific Errors**:
```rust
pub enum CircuitError {
    Identity(IdentityError),
    Membership(MembershipError),
    Privacy(PrivacyError),
}

pub enum IdentityError {
    InvalidPreimageLength,
    HashMismatch,
}

pub enum MembershipError {
    InvalidPathLength,
    RootMismatch,
    LeafNotFound,
}

pub enum PrivacyError {
    ValueOutOfRange,
    InvalidBitWidth,
}
```

### Error Documentation (`docs/error-handling.md`)

Each error entry includes:
- **Error**: `SetupError::ParametersAlreadyExist`
- **Meaning**: Setup parameters already exist for this circuit
- **Common Cause**: Calling `setup()` twice without clearing `params/` directory
- **Fix**: Delete existing parameters or use `load_params()` instead
- **Example**: Shows the code that triggers it and how to handle it

### User-Facing Error Messages

```rust
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Prove(ProveError::WitnessGenerationFailed) => {
                write!(f, "Failed to generate witness from inputs. \
                           Check that all private inputs are valid and correctly formatted.")
            }
            Error::Verify(VerifyError::InvalidProof) => {
                write!(f, "Proof verification failed. The proof may have been \
                           tampered with or does not match the public inputs.")
            }
            // ... detailed messages for all errors
        }
    }
}
```

### Error Handling Examples

All example code demonstrates proper error handling:
```rust
match setup(&circuit) {
    Ok((pk, vk)) => {
        println!("Setup complete in {:?}", elapsed);
    }
    Err(e) if e.kind() == ErrorKind::ParametersAlreadyExist => {
        println!("Loading existing parameters...");
        let (pk, vk) = load_params("identity")?;
        // ...
    }
    Err(e) => {
        eprintln!("Setup failed: {}", e);
        eprintln!("Debug info: {:?}", e);
        return Err(e);
    }
}
```

---

## Examples and Showcase

### Individual Circuit Examples (`examples/`)

**`examples/identity_proof.rs`**:
```rust
// Scenario: Password authentication
// Prove you know a password without transmitting it

fn main() -> Result<()> {
    // Setup: Define password hash (public)
    let password_hash = hash("my_secret_password");

    // Prover: Generate proof of knowledge
    let proof = prove_identity(password_hash, "my_secret_password")?;

    // Verifier: Check proof without learning password
    let is_valid = verify_identity(password_hash, &proof)?;

    assert!(is_valid);
    println!("✓ Password verified without transmission!");
}
```

**`examples/membership_proof.rs`**:
```rust
// Scenario: Anonymous allowlist verification
// Prove you're whitelisted without revealing which address

fn main() -> Result<()> {
    // Setup: Create Merkle tree of whitelisted addresses
    let tree = MerkleTree::from_addresses([...]);

    // Prover: Generate membership proof
    let (leaf, path) = tree.get_proof("0x123..."); // Your address
    let proof = prove_membership(tree.root(), leaf, path)?;

    // Verifier: Check membership without learning which address
    let is_member = verify_membership(tree.root(), &proof)?;

    println!("✓ Verified whitelist membership anonymously!");
}
```

**`examples/privacy_proof.rs`**:
```rust
// Scenario: Age verification without revealing age
// Prove you're 18+ without showing birthdate

fn main() -> Result<()> {
    // Setup: Define age range [18, 150]
    let min_age = 18;
    let max_age = 150;

    // Prover: Prove age is within range
    let actual_age = 27; // Private!
    let proof = prove_range(min_age, max_age, actual_age)?;

    // Verifier: Check age requirement is met
    let is_valid = verify_range(min_age, max_age, &proof)?;

    println!("✓ Age verified (≥18) without revealing actual age!");
}
```

### Full Demo (`examples/full_demo.rs`)

A comprehensive showcase that runs all three circuits in sequence:
1. Generates fresh parameters for all circuits (with timing)
2. Creates and verifies proofs for each circuit
3. Prints performance metrics and proof sizes
4. Demonstrates error cases (tampered proof, wrong witness)
5. Saves all artifacts to disk for inspection

**Output**:
```
=== Groth16 SNARK Library Demo ===

[1/3] Identity Circuit (Hash Preimage)
  Setup:    245ms (PK: 12.3 KB, VK: 1.2 KB)
  Prove:    89ms (Proof: 288 bytes)
  Verify:   2.1ms ✓

[2/3] Membership Circuit (Merkle Tree)
  Setup:    312ms (PK: 15.1 KB, VK: 1.4 KB)
  Prove:    124ms (Proof: 288 bytes)
  Verify:   2.8ms ✓

[3/3] Privacy Circuit (Range Proof)
  Setup:    178ms (PK: 10.2 KB, VK: 1.1 KB)
  Prove:    67ms (Proof: 288 bytes)
  Verify:   1.9ms ✓

All proofs verified successfully! 🎉
```

### Showcase Features

**Quick Start** (in README):
```bash
# Clone and run demo
git clone https://github.com/yourusername/zk-groth16-snark
cd zk-groth16-snark
cargo run --example full_demo

# Run individual examples
cargo run --example identity_proof
cargo run --example membership_proof
cargo run --example privacy_proof

# Run tests
cargo test

# Run benchmarks
cargo bench
```

**Portfolio Presentation Tips**:
- Run `full_demo` first - shows complete pipeline in action
- Then walk through individual examples to explain use cases
- Show benchmark results to discuss performance trade-offs
- Mention all tests pass with zero warnings (`cargo clippy`)

---

## Implementation Notes

### Dependencies (Cargo.toml)

```toml
[dependencies]
# ZK primitives
ark-groth16 = "0.4"
ark-relations = "0.4"
ark-r1cs-std = "0.4"
ark-crypto-primitives = "0.4"
ark-ff = "0.4"
ark-ec = "0.4"
ark-bls12-381 = "0.4"

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

[dev-dependencies]
# Testing
proptest = "1.0"
criterion = "0.5"

[features]
default = ["std"]
std = []
```

### Week 7 Integration

Relevant circuit constraint logic from Week 7 (`zk-arkworks-lab`) will be copied and adapted with clear `// Adapted from week7/circuits/src/...` comments. This makes Week 8 self-contained while showing the learning progression.

### Success Criteria

**Done When**:
- ✅ All three circuits implement `Groth16Circuit` trait
- ✅ Full pipeline works: setup → prove → verify for each circuit
- ✅ All tests pass (`cargo test`)
- ✅ Zero clippy warnings (`cargo clippy`)
- ✅ Benchmarks run successfully (`cargo bench`)
- ✅ All examples run without errors
- ✅ README and circuit docs are complete
- ✅ Error handling documentation covers all error types
- ✅ Code is formatted (`cargo fmt`)

**Portfolio Ready**:
- Clean, idiomatic Rust code
- Well-documented with doc comments
- Comprehensive test coverage
- Real-world use case examples
- Performance benchmarks
- Professional README

---

## Next Steps

1. **Review this design** - Ensure it aligns with learning goals
2. **Create implementation plan** - Break down into concrete tasks
3. **Set up git worktree** - Isolated development environment
4. **Implement core trait** - `Groth16Circuit` trait and infrastructure
5. **Implement circuits** - One at a time, starting with identity
6. **Write tests** - Unit, integration, and property-based
7. **Add benchmarks** - Criterion suite for all circuits
8. **Write documentation** - README + circuit-specific docs
9. **Create examples** - Demonstrations of each circuit
10. **Final polish** - Clippy, formatting, portfolio review

This Week 8 design provides a strong foundation for Weeks 9-12, where you'll build on this Groth16 knowledge to implement more advanced ZK applications and capstone projects.
