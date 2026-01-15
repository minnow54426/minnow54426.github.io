# Groth16 Demonstration Project - Design Document

**Date**: 2025-01-11
**Status**: Approved
**Author**: Design brainstorming session with Claude Code

## Overview

This document describes the design for a comprehensive Rust project demonstrating the Groth16 zero-knowledge proof system. The project serves three purposes:

1. **Educational Tutorial**: Step-by-step learning for beginners
2. **Practical Implementation Reference**: Working code others can use
3. **Paper Companion Guide**: Demonstrates each section of the Groth16 paper

### Key Characteristics

- **Rigorous mathematical detail**: Paper-level coverage of theorems and proofs
- **Hybrid implementation**: Use arkworks for crypto primitives, implement Groth16 logic ourselves
- **Progressive examples**: 5 example circuits of increasing complexity
- **Concept-first approach**: Build prerequisites (R1CS, QAP, pairings) before Groth16

---

## Project Structure

```
groth16-demo/
├── README.md                    # Top-level navigation and quickstart
├── Cargo.toml                   # Workspace configuration
├── docs/                        # Detailed documentation
│   ├── mathematical-background.md
│   ├── architecture.md
│   └── paper-explanations.md
│
├── crates/
│   ├── math/                    # Core mathematical primitives
│   │   ├── src/fields.rs       # Finite field operations
│   │   ├── src/pairing.rs      # Bilinear pairings (using arkworks)
│   │   └── src/polynomial.rs   # Polynomial operations
│   │
│   ├── r1cs/                    # Rank-1 Constraint System
│   │   ├── src/lib.rs          # R1CS core types and operations
│   │   ├── src/constraint.rs   # Constraint representation
│   │   └── src/witness.rs      # Witness generation and satisfaction
│   │
│   ├── qap/                     # Quadratic Arithmetic Programs
│   │   ├── src/lib.rs          # R1CS → QAP transformation
│   │   ├── src/polynomials.rs  # Target polynomials A, B, C
│   │   └── src/divisibility.rs # Division check (H divides Z)
│   │
│   ├── groth16/                 # Groth16 proving system
│   │   ├── src/lib.rs          # Main Groth16 implementation
│   │   ├── src/setup.rs        # Trusted setup (MPC possible)
│   │   ├── src/prove.rs        # Proof generation
│   │   └── src/verify.rs       # Proof verification
│   │
│   └── circuits/                # Example circuits
│       ├── src/multiplier.rs   # a * b = c
│       ├── src/cubic.rs        # ax³ + bx² + cx + d = y
│       ├── src/hash_preimage.rs
│       ├── src/merkle.rs       # Merkle membership
│       ├── src/range_proof.rs  # age ≥ 18
│       └── examples/           # Runnable demos
│
└── book/                        # Tutorial book (mdbook)
    ├── src/
    │   ├── 00-introduction.md
    │   ├── 01-math-background.md
    │   ├── 02-r1cs.md
    │   ├── 03-qap.md
    │   ├── 04-pairings.md
    │   ├── 05-groth16-setup.md
    │   ├── 06-groth16-prove.md
    │   ├── 07-groth16-verify.md
    │   ├── 08-examples/
    │   │   ├── 01-multiplier.md
    │   │   ├── 02-cubic.md
    │   │   ├── 03-hash-preimage.md
    │   │   ├── 04-merkle.md
    │   │   └── 05-range-proof.md
    │   └── 09-security-analysis.md
    └── book.toml
```

---

## Learning Progression

### Part I: Mathematical Foundations (Chapters 1-4)

**Chapter 1: Introduction**
- What problem does Groth16 solve?
- ZK-SNARK landscape and Groth16's position
- Security properties: completeness, soundness, zero-knowledge
- Trusted setup requirement and its implications
- Overview of the proving pipeline

**Chapter 2: Mathematical Background**
- Finite fields: arithmetic, prime fields, extension fields
- Polynomials over finite fields: representation, evaluation, interpolation
- Lagrange polynomials and the Vandermonde matrix
- **Code demo**: Field operations with arkworks

**Chapter 3: Rank-1 Constraint Systems (R1CS)**
- From computation to constraints
- Matrix form: Az ∘ Bz = Cz
- Constraint satisfaction checking
- **Code demo**: Build R1CS for simple circuits, verify witnesses
- **Paper connection**: Section 2 preliminaries

**Chapter 4: Quadratic Arithmetic Programs (QAP)**
- R1CS → QAP transformation via Lagrange interpolation
- Target polynomials A(x), B(x), C(x) from R1CS matrices
- The polynomial Z(x) = ∏(x - i) for divisibility check
- **Code demo**: Transform R1CS to QAP, construct target polynomials
- **Paper connection**: Section 3.1 construction

### Part II: Pairings and Elliptic Curves (Chapters 5-6)

**Chapter 5: Elliptic Curves and Pairings**
- Elliptic curve basics (high-level, focus on pairing groups)
- Bilinear maps: e(g₁^a, g₂^b) = e(g₁, g₂)^(ab)
- Groups G₁, G₂, G_T and pairing-friendly curves (BN254)
- **Code demo**: Pairing operations with arkworks
- **Paper connection**: Section 2.1 notation

**Chapter 6: The Powers of Tau and QAP Division**
- Polynomial encoding in the exponent
- The divisibility check: p(x) = h(x)·Z(x)
- **Code demo**: Encode polynomials, verify divisibility via pairing
- **Paper connection**: Section 3.1 core insight

### Part III: Groth16 Protocol (Chapters 7-9)

**Chapter 7: Trusted Setup**
- Powers of Tau ceremony conceptually
- Generating proving key (pk) and verification key (vk)
- Structure of pk: [α], [β], [δ], and encrypted terms
- Structure of vk: [α]₁, [β]₂, [γ]₂, public inputs
- **Code demo**: Setup for example circuits
- **Paper connection**: Section 3.1 algorithm

**Chapter 8: Proof Generation**
- Randomness for zero-knowledge: r, s, δ values
- Computing proof elements A, B, C
- The proof construction equation (full detail)
- **Code demo**: Generate proofs with full intermediate values
- **Paper connection**: Section 3.1 Prove algorithm

**Chapter 9: Proof Verification**
- The pairing equation: e(A, B) = e([α]₁, [β]₂) · e([β]₁, vk.γ) · e(C, vk.δ)
- Public input reconstruction
- **Code demo**: Verify proofs, reject invalid proofs
- **Paper connection**: Section 3.2 Verify algorithm

### Part IV: Examples and Applications (Chapters 10-14)

**Chapter 10: Simple Multiplier (a × b = c)**
- **ZK Property**: Verifier learns product c, but cannot determine a or b individually
- **Public inputs**: [c]
- **Private witness**: [a, b, 1]
- **What's hidden**: The factors a and b (infinite factorization ambiguity)
- **R1CS**: 3 constraints

**Chapter 11: Cubic Polynomial Evaluation**
- **ZK Property**: Verifier learns polynomial evaluates to y, but cannot determine which x
- **Public inputs**: [a, b, c, d, y] (coefficients and result)
- **Private witness**: [x, x², x³, intermediates]
- **What's hidden**: The input value x
- **R1CS**: ~8-10 constraints

**Chapter 12: Hash Preimage**
- **ZK Property**: Verifier learns you know some m with H(m) = h, but cannot learn m
- **Public inputs**: [h] (hash output)
- **Private witness**: [m, hash_intermediate_state]
- **What's hidden**: The preimage m
- **R1CS**: ~300 constraints (Poseidon hash)

**Chapter 13: Merkle Tree Membership**
- **ZK Property**: Verifier learns your leaf is in the tree, but cannot determine WHICH leaf
- **Public inputs**: [root]
- **Private witness**: [leaf, path, path_indices]
- **What's hidden**: Your specific leaf value and its position
- **R1CS**: ~2,400 constraints (depth 8 tree with Poseidon)

**Chapter 14: Range Proof / Selective Disclosure**
- **ZK Property**: Prove a constraint on private value without revealing the value itself
- **Statement**: age ≥ 18 (or value ∈ [min, max])
- **Public inputs**: [minimum_threshold]
- **Private witness**: [age, bit_decomposition, comparison_result]
- **What's hidden**: The exact age, only learn it's ≥ threshold
- **R1CS**: Variable (depends on bit-width)

---

## Example Circuits Details

### Interactive Elements for Each Example

Each chapter includes:
1. **Problem statement** in plain English
2. **R1CS construction** with explicit matrices
3. **QAP transformation** with polynomial equations
4. **Setup phase** - generate pk/vk, show all values
5. **Proof generation** - step-by-step with intermediate computations
6. **Verification** - pairing equation check
7. **Tamper testing** - what breaks verification (wrong witness, wrong public input, corrupted proof)
8. **Performance metrics** - proving time, verification time, proof size

### Zero-Knowledge Documentation

Each example explicitly documents:
1. What the verifier learns
2. What remains private
3. Why multiple witnesses satisfy the statement (the "zero-knowledge" intuition)
4. Information-theoretic vs computational zero-knowledge distinction

---

## Implementation Details

### Testing Strategy

```rust
tests/
├── unit_tests.rs           // Per-function tests
├── integration_tests.rs    // Cross-module tests
├── property_tests.rs       // Proptest for invariants
└── paper_examples.rs       // Test vectors from Groth16 paper
```

**Test categories**:
1. **Correctness**: Valid witnesses satisfy constraints
2. **Soundness**: Invalid witnesses fail verification
3. **ZK property**: Distribution of proofs for different witnesses is indistinguishable (conceptual)
4. **Edge cases**: Boundary values, zero inputs, large values
5. **Paper test vectors**: Replicate examples from Groth16 paper

### Documentation Approach

**Three-tier documentation**:

1. **Code comments** (rustdoc):
   - Document every public function with `///`
   - Include mathematical explanations in comments
   - Provide examples in doc tests

2. **Tutorial book** (mdbook):
   - Narrative explanations with equations
   - Inline code examples
   - Diagrams (using mermaid or ASCII art)
   - "What you've learned" summaries

3. **Paper explanations**:
   - Map paper theorems to code modules
   - Step-by-step algorithm walkthroughs
   - Security proof explanations (intuitive level)

### Code Quality Standards

```bash
# Pre-commit checks
cargo fmt --check              # Formatting
cargo clippy -- -D warnings    # Linting
cargo test                     # All tests pass
cargo doc --no-deps            # Documentation builds
```

**Rust patterns**:
- Error handling: `anyhow::Result<T>` for application errors
- Newtype pattern for cryptographic types
- `Debug` and `Clone` derives for testing
- Public APIs use clear, descriptive names

### Performance Considerations

```rust
benches/
├── r1cs_benchmark.rs         // Constraint system operations
├── qap_benchmark.rs          // Polynomial operations
├── setup_benchmark.rs        // Key generation time
├── prove_benchmark.rs        // Proof generation time
└── verify_benchmark.rs       // Verification time
```

**Metrics to track**:
- Setup time (one-time cost)
- Proving time (user-side)
- Verification time (verifier-side)
- Proof size in bytes
- Memory usage during operations

### Dependencies

```toml
[workspace.dependencies]
# Core crypto primitives
ark-ff = "0.4"              # Finite fields
ark-ec = "0.4"              # Elliptic curves
ark-bn254 = "0.4"           # BN254 pairing-friendly curve
ark-poly = "0.4"            # Polynomial operations

# Pairing operations
ark-groth16 = "0.4"         # Reference implementation (for comparison)

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Testing
proptest = "1.0"            # Property-based testing
criterion = "0.5"           # Benchmarking

# Documentation
mdbook = "0.4"              # Tutorial book
```

**Why arkworks**:
- Modern, well-maintained Rust ecosystem
- Modular design (use only what you need)
- Excellent documentation
- Widely used in production (Aztec, Mina, etc.)

**Hybrid approach**:
- Use arkworks for: field arithmetic, elliptic curves, pairings
- Implement ourselves: Groth16 protocol logic, R1CS/QAP transformations
- Benefit: Learn the protocol while avoiding reimplementation of crypto primitives

---

## Success Criteria

The project succeeds when:

**For learners**:
- Can follow tutorial from zero background to working Groth16 implementation
- Understand WHY each step works (not just WHAT to do)
- Can implement their own simple circuits after completing the tutorial
- Pass comprehension quizzes at end of each chapter

**For practitioners**:
- Can use code as reference for building production ZK systems
- Understand trade-offs between different proving systems
- Know when to use Groth16 vs alternatives
- Can extend examples to their own use cases

**For researchers**:
- Can map paper theorems to concrete code
- Understand security assumptions and trusted setup implications
- See clear connection between mathematical definitions and implementation

---

## Potential Pitfalls and Mitigations

**Pitfall 1: Math overload**
- **Risk**: Readers get bogged down in equations
- **Mitigation**: Provide intuition before formalism, use diagrams, include "skip ahead" markers

**Pitfall 2: Dependency hell**
- **Risk**: arkworks version conflicts, breaking changes
- **Mitigation**: Pin exact versions, provide CI configuration, document setup clearly

**Pitfall 3: Testing blind spots**
- **Risk**: Tests pass but implementation is wrong (e.g., wrong pairing equation)
- **Mitigation**: Cross-check against ark-groth16 reference implementation, include paper test vectors

**Pitfall 4: Performance confusion**
- **Risk**: Learners think slow implementation is "wrong"
- **Mitigation**: Set expectations, benchmark reference implementation, emphasize correctness over speed

---

## Future Extensions (Optional / Out of Scope)

**Advanced topics** to add later:
- **MPC ceremonies**: Distributed trusted setup (Powers of Tau)
- **Recursive proofs**: Proof composition, SNARKs inside SNARKs
- **Alternative proving systems**: PLONK, Halo2, STARKs comparison
- **Optimization techniques**: Constraint reduction, batch verification
- **Formal verification**: Proving correctness of implementation
- **Wasm compilation**: Run prover in browser

**Integration paths**:
- On-chain verification (Ethereum smart contract)
- Prover as web service
- Hardware acceleration (FPGA/GPU)

---

## Next Steps

**Immediate actions**:

1. Create project structure and initialize workspace
   ```bash
   mkdir -p groth16-demo/{crates/{math,r1cs,qap,groth16,circuits},book/src,docs}
   cd groth16-demo
   cargo init --workspace
   ```

2. Initialize git repository and set up commits

3. Set up first crate (math) with field operations demo

4. Write Chapter 1 (Introduction) in mdbook format

5. Implement Example 1 end-to-end as proof of concept

---

## Appendix: Quick Reference

### Key Files and Their Purposes

| File/Directory | Purpose |
|---------------|---------|
| `crates/math/` | Finite fields, pairings, polynomials |
| `crates/r1cs/` | Constraint system representation |
| `crates/qap/` | R1CS → QAP transformation |
| `crates/groth16/` | Setup, prove, verify algorithms |
| `crates/circuits/` | Example circuit implementations |
| `book/` | Tutorial documentation |
| `docs/` | Supplementary documentation |

### Paper Section Mapping

| Paper Section | Code Module |
|---------------|-------------|
| Section 2 (Preliminaries) | `crates/math/`, `crates/r1cs/` |
| Section 3.1 (Construction) | `crates/qap/`, `crates/groth16/src/setup.rs` |
| Section 3.1 (Prove) | `crates/groth16/src/prove.rs` |
| Section 3.2 (Verify) | `crates/groth16/src/verify.rs` |
| Section 4 (Security Analysis) | `docs/paper-explanations.md` |

---

**Design approved: 2025-01-11**
**Ready for implementation planning phase**
