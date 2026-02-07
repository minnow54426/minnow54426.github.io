# Introduction

Welcome to this comprehensive guide on Groth16 zero-knowledge proofs!

## Learning Objectives

After reading this chapter, you will understand:
- What problem zero-knowledge proofs solve
- Why Groth16 is a widely-used zk-SNARK protocol
- What this project teaches you
- How to use this tutorial effectively

## What Are Zero-Knowledge Proofs?

Imagine you know a secret password, but you want to prove you know it **without revealing the password itself**. This is the essence of zero-knowledge proofs.

**Formal definition**: A zero-knowledge proof is a cryptographic method where one party (the prover) can convince another party (the verifier) that a statement is true, without revealing any information beyond the truth of the statement itself.

### Real-World Analogy: Where's Waldo?

Think of the "Where's Waldo?" puzzle:

> You find Waldo quickly. You want to prove to your friend you found him, but you don't want to reveal his location (spoiling the fun).

**Zero-knowledge solution**:
1. You (prover) find Waldo
2. You cover the page with a large piece of cardboard with a small cutout
3. You position the cutout over Waldo so only Waldo is visible
4. Your friend (verifier) sees Waldo through the hole
5. Your friend is convinced Waldo exists on the page
6. But your friend learns nothing about Waldo's location!

This captures the three properties of zero-knowledge proofs:
- **Completeness**: If Waldo is really there, your friend will be convinced
- **Soundness**: If Waldo isn't there, you can't fake the proof
- **Zero-knowledge**: Your friend learns only that Waldo exists, not where

## Why Groth16?

**Groth16** is a specific zero-knowledge proof protocol introduced by Jens Groth in 2016:

> **Reference**: Groth, J. (2016). "On the Size of Pairing-based Non-Interactive Arguments" [EUROCRYPT 2016]

### Key Advantages

1. **Tiny proofs**: Only 128 bytes (regardless of circuit complexity!)
2. **Fast verification**: Constant-time verification with pairings
3. **Widely deployed**: Used by Zcash, Ethereum (Tornado Cash), and more
4. **Battle-tested**: Years of real-world use with no breaks

### Trade-offs

1. **Trusted setup**: Requires a one-time setup ceremony with "toxic waste"
2. **Per-circuit keys**: Each circuit needs its own proving/verification keys
3. **Not universal**: Unlike newer protocols (PLONK, Halo 2), Groth16 requires circuit-specific setup

## What This Project Teaches

This project takes you from zero to understanding Groth16 at a rigorous mathematical level, with working Rust code you can run yourself.

### Prerequisites

- **Rust basics**: You know how to read Rust code
- **High school math**: You know what polynomials and modular arithmetic are
- **Curiosity**: You want to understand how the magic works!

### What You'll Learn

**Part I: Foundations**
- Finite fields and modular arithmetic
- Rank-1 Constraint Systems (R1CS)
- Quadratic Arithmetic Programs (QAP)

**Part II: The Protocol**
- Elliptic curves and pairings
- Trusted setup ceremonies
- Proof generation

**Part III: Practice**
- Proof verification
- Building your own circuits
- Batch verification for performance

## How to Use This Tutorial

### Code-First Approach

Each chapter follows this pattern:
1. **Concrete example**: See working code first
2. **Theory explanation**: Understand why it works
3. **Mathematical detail**: Connect to the Groth16 paper
4. **Practice**: Run the code yourself

### Running the Examples

All code examples are runnable. From the project root:

```bash
# Run the multiplier demo
cargo run --bin multiplier-demo

# Run tests
cargo test --workspace

# Build the project
cargo build --workspace
```

### Following Along

We recommend:
1. **Read each chapter in order** - concepts build on previous ones
2. **Run the code examples** - see the cryptography in action
3. **Do the exercises** - test your understanding
4. **Consult the paper** - dive deeper when curious

## Quick Start: Hello World of ZK Proofs

Let's start with a simple example: proving you know factors of a number without revealing them.

### The Problem

> Prove you know `a` and `b` such that `a × b = 12`, without revealing `a` or `b`.

### The Solution

```rust,ignore
use groth16_circuits::multiplier::MultiplierCircuit;
use groth16::{trusted_setup, generate_proof, verify_proof};
use ark_bn254::Fr as ScalarField;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

// Private witness: we know a=3, b=4
let circuit = MultiplierCircuit::new(3, 4, 12);

// Trusted setup ceremony (one-time)
let mut rng = ChaCha20Rng::from_entropy();
let (pk, vk) = trusted_setup(circuit.clone(), &mut rng).unwrap();

// Generate proof
let proof = generate_proof(&pk, circuit, &mut rng).unwrap();

// Verify proof (only public input: 12)
let public_inputs = vec![ScalarField::from(12u64)];
let is_valid = verify_proof(&vk, &proof, &public_inputs).unwrap();

assert!(is_valid);
// Verifier knows factors exist, but not which factors!
```

### What Just Happened?

1. **Circuit**: We encoded the computation `a × b = c` as constraints
2. **Setup**: We generated proving key (pk) and verification key (vk)
3. **Proof**: We created a ~128-byte proof using our secret (a=3, b=4)
4. **Verification**: The verifier checked the proof with only the public output (c=12)

The verifier is convinced factors exist, but learns nothing about which factors!

## Project Structure

```text
groth16-demo/
├── crates/
│   ├── groth16/              # Main Groth16 implementation
│   │   └── src/
│   │       └── wrapper.rs    # Educational wrapper with detailed comments
│   ├── circuits/             # Example circuits
│   │   └── src/
│   │       └── multiplier.rs # a × b = c circuit
│   └── [other crates...]
├── book/                     # This tutorial
│   └── src/
│       ├── 00-introduction.md
│       ├── 01-math-background.md
│       └── ...
└── examples/                 # Standalone demo programs
    └── multiplier-demo.rs
```

## What's Next?

In **Chapter 1**, we'll build the mathematical foundations needed to understand Groth16:
- Finite fields and modular arithmetic
- Polynomials and interpolation
- Group theory basics

Don't worry if it seems abstract - everything connects back to practical code!

## Exercises

1. **Run the multiplier demo**:
   ```bash
   cargo run --bin multiplier-demo
   ```
   What do you see? Try modifying the circuit (different a, b, c values).

2. **Zero-knowledge property**: What happens if you create a proof with a=2, b=6 (both multiply to 12) versus a=3, b=4? Can the verifier tell the difference?

3. **Explore the code**: Open `crates/groth16/src/wrapper.rs` and read the `generate_proof` function. What parts make sense? What parts are confusing?

## Further Reading

- **Original Paper**: [Groth16](https://eprint.iacr.org/2016/260) - Sections 1-2 for motivation
- **ZK-SNARKs Explained**: [Vitalik's Blog](https://vitalik.ca/general/2021/01/26/snarks.html)
- **ZCash Protocol**: [Zcash Orchard Specification](https://zips.z.cash/protocol/protocol.pdf#orchard)

---

**Ready for the math? Continue to [Chapter 1: Mathematical Background](./01-math-background.md)**
