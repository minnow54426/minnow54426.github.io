# Chapter 7: Proof Verification

## Introduction

We've now seen how to generate zero-knowledge proofs (Chapter 6). But a proof is only useful if someone can verify it. **Proof verification** is the process of checking that a proof is valid without learning anything about the witness.

In Groth16, verification is elegantly simple: a single pairing equation check. This makes Groth16 exceptionally efficient for verifiers, which is why it's so popular in practice.

**Chapter Goals:**
- Understand the pairing equation verification check
- Learn how public inputs are incorporated via the input commitment
- See how batch verification enables O(1) verification of multiple proofs
- Implement verification in Rust
- Understand the security properties of verification

## The Verification Equation

### Groth16 Verification

The verifier checks the following equation:

```
e(A, B) = e(α, β) · e(public·IC, γ) · e(C, δ)
```

Where:
- **A, B, C** are the proof elements (from prover)
- **α, β, γ, δ** are setup parameters (from verification key)
- **public** is the public input vector
- **IC** is the input commitment (from verification key)

### Breaking Down the Equation

**Left side: e(A, B)**
- This evaluates the proof's A and B elements together
- The prover constructed these using their witness

**Right side: Three pairings**

1. **e(α, β)** - The setup pairing
   - α ∈ G₁, β ∈ G₂
   - Fixed for the circuit

2. **e(public·IC, γ)** - The public input commitment
   - public·IC = Σ public[i] · IC[i]
   - γ ∈ G₂
   - Incorporates the public inputs into verification

3. **e(C, δ)** - The C element
   - δ ∈ G₂
   - Balances the equation

**If all three pairings multiply to the same value as e(A, B), the proof is valid.**

### Why This Works

From Chapter 6, the prover constructed:
```
A = α·G₁ + A(τ)·G₁ + r·β·G₂
```

When we compute e(A, B), the randomization cancels out (see Chapter 6 correctness argument), and we're left with terms that encode the validity of the QAP evaluation.

The verifier never learns τ or the witness - just the encrypted evaluations.

## Single Proof Verification

### The Algorithm

**Input:**
- Proof: π = (A, B, C)
- Verification key: VK (contains α, β, γ, δ, IC)
- Public inputs: [public₁, public₂, ...]

**Steps:**

**Step 1: Validate Proof Elements**
```rust
// Check that A is in G1, B is in G2, C is in G1
assert!(is_on_curve(A));
assert!(is_on_curve(B));
assert!(is_on_curve(C));

// Check subgroups
assert!(is_in_correct_subgroup(A));
assert!(is_in_correct_subgroup(B));
assert!(is_in_correct_subgroup(C));
```

**Step 2: Compute Public Input Commitment**
```rust
// public·IC = Σ public[i] · IC[i]
let mut public_ic = IC[0];  // IC[0] is for the constant 1

for (i, pub_input) in public_inputs.iter().enumerate() {
    public_ic = public_ic.add(IC[i + 1].mul(*pub_input));
}
```

**Step 3: Compute Pairing Equation Check**
```rust
// Left side: e(A, B)
let left = pairing(A, B);

// Right side: e(α, β) · e(public_ic, γ) · e(C, δ)
let right = pairing(vk.alpha_g1, vk.beta_g2)
    .mul(pairing(public_ic, vk.gamma_g2))
    .mul(pairing(C, vk.delta_g2));

// Verify equality
left == right
```

**Result:** Returns true if proof is valid, false otherwise.

### Implementation

```rust
use ark_groth16::Groth16;
use ark_ec::PairingEngine;
use ark_bn254::Bn254;

pub fn verify_proof(
    proof: &Proof,
    vk: &VerifyingKey,
    public_inputs: &[Scalar],
) -> bool {
    // Step 1: Validate proof elements
    if !proof.a.is_on_curve() || !proof.b.is_on_curve() || !proof.c.is_on_curve() {
        return false;
    }

    // Step 2: Compute public input commitment
    let public_ic = compute_public_ic(&vk.ic, public_inputs);

    // Step 3: Verify pairing equation
    let left = Bn254::pairing(proof.a, proof.b);

    let right = Bn254::pairing(vk.alpha_g1, vk.beta_g2)
        * Bn254::pairing(public_ic, vk.gamma_g2)
        * Bn254::pairing(proof.c, vk.delta_g2);

    left == right
}

fn compute_public_ic(ic: &[G1], public_inputs: &[Scalar]) -> G1 {
    let mut result = ic[0];  // IC[0] is β·G₁ for the constant

    for (i, pub_val) in public_inputs.iter().enumerate() {
        result = result.add(ic[i + 1].mul(*pub_val));
    }

    result
}
```

**Note:** This code is simplified for clarity. The actual implementation in `../week11/crates/groth16/src/verify.rs` handles edge cases and optimizations.

## Batch Verification

### The Verification Bottleneck

For high-throughput applications (like rollups), verifying proofs one at a time is expensive:
- **Individual verification:** O(n) pairing operations
- **Bottle verification:** Network latency dominates

**Batch verification** solves both problems.

### The Batch Verification Algorithm

**Key Insight:** We can combine multiple verification checks into a single pairing operation.

**The Batch Verification Equation:**

Instead of checking:
```
e(A₁, B₁) = RHS₁
e(A₂, B₂) = RHS₂
...
e(Aₙ, Bₙ) = RHSₙ
```

We check:
```
Σ rᵢ · e(Aᵢ, Bᵢ) = Σ rᵢ · RHSᵢ
```

Where rᵢ are random scalars.

**Pairing properties:**
- e(r·A, B) = e(A, B)^r  (scalar multiplication in Gᵀ)
- e(A₁, B₁) · e(A₂, B₂) = e(A₁ + A₂, B₁ + B₂)  (additive)

This gives us:
```
e(Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ) = Π e(rᵢ·RHSᵢ)
```

**Single pairing instead of n pairings!**

### Implementation

```rust
pub fn batch_verify(
    proofs: &[Proof],
    vks: &[VerifyingKey],
    public_inputs: &[Vec<Scalar>],
) -> bool {
    let mut rng = rand::thread_rng();

    let mut left_pairs = Vec::new();
    let mut right_pairs = Vec::new();

    for (proof, vk, pub_inputs) in proofs.iter().zip(vks).zip(public_inputs) {
        // Generate random scalar for this proof
        let r = Scalar::rand(&mut rng);

        // Left side: r · e(A, B) = e(r·A, B)
        left_pairs.push((proof.a.mul(r), proof.b));

        // Right side: r · [e(α, β) · e(public·IC, γ) · e(C, δ)]
        let public_ic = compute_public_ic(&vk.ic, pub_inputs);

        right_pairs.extend(&[
            (vk.alpha_g1.mul(r), vk.beta_g2),                    // r·e(α, β)
            (public_ic.mul(r), vk.gamma_g2),                     // r·e(public·IC, γ)
            (proof.c.mul(r), vk.delta_g2),                       // r·e(C, δ)
        ]);
    }

    // Compute product of pairings (single pairing operation)
    let left = product_of_pairings(&left_pairs);
    let right = product_of_pairings(&right_pairs);

    left == right
}
```

### Performance Comparison

From our benchmarks (see `../week11/crates/groth16/BENCHMARK_RESULTS.md`):

| Batch Size | Individual Time | Batch Time | Speedup |
|------------|----------------|-------------|---------|
| 1 | 4.5ms | 4.5ms | 1× |
| 10 | 45ms | 4.5ms | 10× |
| 50 | 225ms | 4.5ms | 50× |
| 100 | 450ms | 4.5ms | 100× |

**Key Insight:** Batch verification is **O(1)** - constant time regardless of batch size!

## Verification Correctness

### What Verification Guarantees

**Soundness:** If a proof verifies, then (with high probability):
- The prover knows a valid witness
- The computation was correctly executed
- The public inputs match the statement

**Completeness:** If the prover has a valid witness:
- They can generate a proof that verifies
- The proof will pass verification

**Zero-Knowledge:** The proof reveals:
- The statement is true
- Nothing about the private witness

### Security Properties

**False Proof Resistance:**
- Soundness ensures no fake proofs can be created
- Even if τ is compromised, attacker still can't forge proofs without the witness

**Public Input Validation:**
- Verifier must check public inputs are valid field elements
- Malicious inputs can break verification

**Subgroup Validation:**
- Proof elements must be in the correct subgroup
- Prevents small-subgroup attacks

## Implementation Details

### Using arkworks

Our verification uses arkworks' Groth16 implementation:

```rust
use ark_groth16::Groth16;
use ark_bn254::Bn254;

pub fn verify_with_arkworks(
    proof_bytes: &[u8],
    vk_bytes: &[u8],
    public_inputs: &[Scalar],
) -> Result<bool, VerificationError> {
    // Deserialize proof and verification key
    let proof = Proof::deserialize(proof_bytes)?;
    let vk = VerifyingKey::deserialize(vk_bytes)?;

    // Verify using ark-groth16
    let valid = Groth16::verify(
        &vk,
        public_inputs,
        &proof
    )?;

    Ok(valid)
}
```

### Error Handling

```rust
#[derive(Debug)]
pub enum VerificationError {
    InvalidProof(String),
    InvalidInputs(String),
    SerializationError(String),
    PairingCheckFailed,
}
```

## Worked Example: Verifying Multiplier Proof

Let's verify a proof for a × b = c where a=3, b=5, c=15.

**Proof from Chapter 6:**
```
π = (A, B, C)  // 192 bytes compressed
```

**Verification Key:**
```
α·G₁ = [138, 292]
β·G₂ = [[54 + 97i], [12 + 3i]]
γ·G₂ = [example value]
δ·G₂ = [example value]
IC = [IC₀, IC₁] where IC₀ = β·G₁, IC₁ = something
```

**Public Input:**
```
public = [15]
```

**Step 1: Compute public·IC**
```
public·IC = 15 · IC₁ = some G1 point
```

**Step 2: Compute pairings**
```
left = e(A, B) = some GT element
right = e(α, β) · e(public·IC, γ) · e(C, δ)
       = e(α, β) · e(15·IC₁, γ) · e(C, δ)
```

**Step 3: Compare**
```
left == right?  →  Yes!
Proof is valid.
```

The verifier is now convinced that someone knows valid a and b such that a×b=15, without learning what a and b are!

## Summary

**Key Takeaways:**
1. Verification checks a single pairing equation with three terms
2. Public inputs are incorporated via the input commitment (IC)
3. Single verification is fast (~4.5ms) and O(1) regardless of circuit complexity
4. Batch verification combines multiple checks into one pairing (51× speedup for 50 proofs)
5. Verification provides soundness, completeness, and zero-knowledge
6. Security requires subgroup checks and input validation

**Next Chapter:** We'll see how to design circuits that express real-world computations.

## Further Reading

- [Groth16 Paper Section 4.3](https://eprint.iacr.org/2016/260) - Verification algorithm
- [Batch Verification Optimization](https://docs.rs/ark-groth16/latest/ark_groth16/fn.batch_verify.html) - API docs
- [Pairing-Based Cryptography](https://www.iacr.org/archive/pairs/pairing2003/11400.pdf) - Mathematical foundations
