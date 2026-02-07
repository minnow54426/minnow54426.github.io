# Groth16 Zero-Knowledge Proofs: From Theory to Implementation

A comprehensive Rust project demonstrating the Groth16 zero-knowledge proof system with rigorous mathematical detail and practical implementation.

## 🎯 Project Overview

This project serves three purposes:

1. **Educational Tutorial**: Step-by-step learning for beginners
2. **Practical Implementation Reference**: Working code others can use
3. **Paper Companion Guide**: Demonstrates each section of the Groth16 paper

## 📚 Table of Contents

- [Quick Start](#quick-start)
- [Project Structure](#project-structure)
- [Learning Path](#learning-path)
- [Example Circuits](#example-circuits)
- [Development](#development)
- [Resources](#resources)

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- Git
- (Optional) mdbook for reading the tutorial locally

### Build the Project

```bash
# Clone the repository
git clone <repository-url>
cd groth16-demo

# Build all crates
cargo build

# Run tests
cargo test

# Run an example
cargo run --bin multiplier-demo
```

### Read the Tutorial

```bash
# Install mdbook
cargo install mdbook

# Build and serve the book locally
cd book
mdbook build
mdbook serve --open

# Or open the pre-built HTML directly
open book/book/index.html  # On macOS
xdg-open book/book/index.html  # On Linux
```

The tutorial book is structured as follows:
- **Chapter 0**: Introduction to ZK and Groth16 ✅
- **Chapter 1**: Mathematical Background ✅
- **Chapters 2-8**: Coming soon (marked as future work)

Current implementation status:
- ✅ Batch verification optimization implemented
- ✅ Complete R1CS and QAP infrastructure
- ✅ Trusted setup, proof generation, and verification
- ✅ Example circuits (multiplier, cubic, hash preimage, Merkle, range proof)
- 📖 Tutorial book (Chapters 0-1 complete, remaining chapters outlined)

## 📁 Project Structure

```
groth16-demo/
├── Cargo.toml                   # Workspace configuration
├── README.md                    # This file
├── docs/                        # Supplementary documentation
│   └── plans/                   # Design documents
├── crates/
│   ├── math/                    # Core mathematical primitives
│   │   └── src/
│   │       ├── fields.rs       # Finite field operations
│   │       ├── pairing.rs      # Bilinear pairings
│   │       └── polynomial.rs   # Polynomial operations
│   ├── r1cs/                    # Rank-1 Constraint System
│   │   └── src/
│   │       ├── constraint.rs   # Constraint representation
│   │       └── witness.rs      # Witness generation
│   ├── qap/                     # Quadratic Arithmetic Programs
│   │   └── src/
│   │       └── polynomials.rs  # R1CS → QAP transformation
│   ├── groth16/                 # Groth16 proving system
│   │   └── src/
│   │       ├── setup.rs        # Trusted setup
│   │       ├── prove.rs        # Proof generation
│   │       └── verify.rs       # Proof verification
│   └── circuits/                # Example circuits
│       └── src/
│           ├── multiplier.rs   # a × b = c
│           ├── cubic.rs        # ax³ + bx² + cx + d = y
│           ├── hash_preimage.rs
│           ├── merkle.rs       # Merkle membership
│           └── range_proof.rs  # age ≥ 18
└── book/                        # Tutorial (mdbook)
    └── src/
        ├── 00-introduction.md
        ├── 01-math-background.md
        ├── 02-r1cs.md
        ├── 03-qap.md
        ├── 04-pairings.md
        ├── 05-groth16-setup.md
        ├── 06-groth16-prove.md
        ├── 07-groth16-verify.md
        └── 08-examples/
```

## 🎓 Learning Path

The tutorial follows a **concept-first approach**, building mathematical foundations before instantiating Groth16:

### Part I: Mathematical Foundations
1. **Introduction** - What problem does Groth16 solve?
2. **Mathematical Background** - Finite fields, polynomials, Lagrange interpolation
3. **Rank-1 Constraint Systems (R1CS)** - From computation to constraints
4. **Quadratic Arithmetic Programs (QAP)** - R1CS → QAP transformation

### Part II: Pairings and Elliptic Curves
5. **Elliptic Curves and Pairings** - Bilinear maps and pairing-friendly curves
6. **The Powers of Tau and QAP Division** - Polynomial encoding in the exponent

### Part III: Groth16 Protocol
7. **Trusted Setup** - Generating proving and verification keys
8. **Proof Generation** - Creating zero-knowledge proofs
9. **Proof Verification** - Verifying proofs with pairing equations

### Part IV: Examples and Applications
10. **Simple Multiplier** - a × b = c
11. **Cubic Polynomial** - ax³ + bx² + cx + d = y
12. **Hash Preimage** - Prove knowledge of preimage
13. **Merkle Tree Membership** - Privacy-preserving membership proof
14. **Range Proof** - Prove age ≥ 18 without revealing age

## 🔢 Example Circuits

All examples demonstrate **zero-knowledge** - the verifier learns the statement is true but learns nothing about the private witness.

### Example 1: Simple Multiplier

**Statement**: Prove knowledge of a, b such that a × b = c (public c)

- **Public inputs**: [c]
- **Private witness**: [a, b, 1]
- **Zero-knowledge**: Verifier learns product, but not the factors
- **Constraints**: 3

### Example 2: Cubic Polynomial

**Statement**: Prove knowledge of x such that ax³ + bx² + cx + d = y

- **Public inputs**: [a, b, c, d, y]
- **Private witness**: [x, x², x³, intermediates]
- **Zero-knowledge**: Verifier learns polynomial evaluates to y, but not x
- **Constraints**: ~8-10

### Example 3: Hash Preimage

**Statement**: Prove knowledge of m such that H(m) = h

- **Public inputs**: [h]
- **Private witness**: [m, hash_intermediate_state]
- **Zero-knowledge**: Verifier learns you know preimage, but not what it is
- **Constraints**: ~300 (Poseidon hash)

### Example 4: Merkle Tree Membership

**Statement**: Prove leaf is in Merkle tree with public root

- **Public inputs**: [root]
- **Private witness**: [leaf, path, path_indices]
- **Zero-knowledge**: Verifier learns leaf is in tree, but not which leaf
- **Constraints**: ~2,400 (depth 8 tree)

### Example 5: Range Proof

**Statement**: Prove age ≥ 18 without revealing exact age

- **Public inputs**: [threshold]
- **Private witness**: [age, bit_decomposition, comparison_result]
- **Zero-knowledge**: Verifier learns constraint satisfied, not the value
- **Constraints**: Variable (depends on bit-width)

## 💻 Development

### Running Tests

```bash
# Test all crates
cargo test

# Test specific crate
cargo test -p groth16-math
cargo test -p groth16-r1cs
cargo test -p groth16-qap
cargo test -p groth16

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_r1cs_satisfaction
```

### Code Quality

```bash
# Format code
cargo fmt

# Check linting
cargo clippy -- -D warnings

# Build documentation
cargo doc --no-deps --open
```

### Running Examples

```bash
# Simple multiplier demo
cargo run --bin multiplier-demo

# Cubic polynomial demo
cargo run --bin cubic-demo

# Hash preimage demo
cargo run --bin hash-preimage-demo

# Merkle membership demo
cargo run --bin merkle-demo

# Range proof demo
cargo run --bin range-proof-demo
```

## 📖 Resources

### Papers
- [Groth16 Paper](https://eprint.iacr.org/2016/260) - "On the Size of Pairing-based Non-Interactive Arguments"
- [Pinocchio Protocol](https://eprint.iacr.org/2013/279) - Predecessor to Groth16

### Libraries
- [arkworks-rs](https://github.com/arkworks-rs) - Rust cryptography library
- [ark-groth16](https://docs.rs/ark-groth16/) - Reference Groth16 implementation
- [ark-relations](https://docs.rs/ark-relations/) - Constraint system traits

### Learning Materials
- [ZKProof](https://zkproof.org/) - Zero-Knowledge Proof standards and resources
- [Vitalik's Blog on ZK-SNARKs](https://vitalik.ca/general/2017/11/09/starks_part_1.html)
- [Awesome Zero-Knowledge Proofs](https://github.com/matter-labs/awesome-zero-knowledge-proofs)

## 🎯 Success Criteria

After completing this tutorial, you should be able to:

- ✅ Understand the Groth16 protocol at a rigorous mathematical level
- ✅ Implement R1CS and QAP transformations
- ✅ Generate and verify zero-knowledge proofs
- ✅ Build your own circuits for practical applications
- ✅ Understand security assumptions and trusted setup implications

## 🤝 Contributing

This is a learning project. Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- Jens Groth for the Groth16 protocol
- The arkworks-rs team for excellent cryptography libraries
- The ZK community for educational resources

---

**Ready to learn? Start with [Chapter 0: Introduction](book/src/00-introduction.md)**
