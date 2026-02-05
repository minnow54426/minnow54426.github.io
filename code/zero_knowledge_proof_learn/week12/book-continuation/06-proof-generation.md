# Chapter 6: Proof Generation

## Introduction

Chapters 1-5 built the foundation: constraint systems, polynomial transformations, pairings, and the trusted setup ceremony. Now we have all the pieces to generate zero-knowledge proofs.

**Proof generation** is where the prover creates a cryptographic proof that convinces the verifier they know a valid witness, without revealing the witness itself. This is the "zero-knowledge" in zk-SNARKs.

**Chapter Goals:**
- Understand the proof structure (A, B, C elements)
- Learn how randomization enables zero-knowledge
- See how the proving key is used to encrypt the witness
- Implement proof generation in Rust
- Understand the relationship between prover and verifier

## The Proof Structure

### Groth16 Proof Format

A Groth16 proof consists of three group elements:

```
π = (A, B, C)
```

Where:
- **A ∈ G₁** - Encrypts the evaluation of A-polynomials
- **B ∈ G₂** - Encrypts the evaluation of B-polynomials
- **C ∈ G₁** - Encrypts the evaluation of C-polynomials and the division polynomial

Each element is 48-128 bytes depending on compression, so the entire proof is quite small (~192 bytes compressed, ~256 bytes uncompressed).

### What Each Element Proves

**A proves:** "I know a witness w such that the A-polynomials evaluate correctly"

**B proves:** "Same for the B-polynomials"

**C proves:** "The QAP divisibility check passes (A·B - C is divisible by t)"

Together, they prove: "The QAP equation holds, so the computation is valid"

## From Witness to Proof

### Step 1: Generate Witness

Given the circuit and public/private inputs, generate the witness vector:

```
w = [1, public₁, public₂, ..., publicₙ, private₁, private₂, ..., privateₘ]
```

The witness includes:
- **1**: The constant (always present)
- **Public inputs**: Known to both prover and verifier
- **Private inputs**: The secret witness

### Step 2: Evaluate QAP Polynomials

Using the witness, evaluate the A, B, C polynomials at τ (from setup):

```
Aτ = A(w) = Σ wᵢ · Aᵢ(τ)
Bτ = B(w) = Σ wᵢ · Bᵢ(τ)
Cτ = C(w) = Σ wᵢ · Cᵢ(τ)
```

These are scalar field elements.

### Step 3: Compute Division Polynomial

Check if the computation is valid:

```
H(τ) = (Aτ · Bτ - Cτ) / t(τ)
```

If H(τ) exists (division succeeds), the witness is valid.

### Step 4: Encrypt with Proving Key

Use the proving key to encrypt the evaluations into group elements:

**Note:** The actual Groth16 proof formulas are more complex. The simplified formulas above show the key ideas. The actual implementation uses encrypted polynomial evaluations from the proving key (a_query, b_query, etc.) with multiple randomization factors. See the implementation in `../week11/crates/groth16/src/prove.rs` for the complete formulas.

The key insight: The proof elements encrypt polynomial evaluations in a way that allows verification via the pairing equation but doesn't reveal the underlying witness values.

## Zero-Knowledge via Randomization

### Why Randomization is Needed

If we just output A(τ), B(τ), C(τ) directly, the verifier could extract information about the witness. We need to **blind** the proof.

### The Blinding Technique

Groth16 uses multiple random blinding factors for zero-knowledge:
- **r**: Randomizes the A element (in G₁)
- **s**: Randomizes the C element (in G₁)
- **δ**: Used in G₂ elements (not shown in simplified code)

**Proof generation with blinding:**

```rust
let r = Fr::rand(&mut rng);  // Random field element
let delta = Fr::rand(&mut rng);

// Compute blinded evaluations
let a_blinded = alpha_a + r * beta_b;
let b_blinded = beta_b;
let c_blinded = beta_a + r * beta_b + delta * beta_h;
```

**Key insight:** The randomness makes it computationally infeasible to extract the witness from the proof, but the pairing equation still verifies correctly!

### Zero-Knowledge Property

**Simulator argument sketch:**
For any verifier, there exists a simulator that can generate indistinguishable proofs without knowing the witness. The simulator uses the verification key to create proofs that satisfy the pairing equation.

**Intuition:** The random blinding adds "noise" to the proof. Since the verifier can't distinguish between "real" proofs (with actual witness) and "simulated" proofs (without witness), they learn nothing about the witness itself.

## Implementation

Our proof generation lives in `../week11/crates/groth16/src/prove.rs`.

**Implementation Note:** The code examples in this section illustrate the key concepts of proof generation. They are simplified for pedagogical clarity. The actual implementation in `../week11/crates/groth16/src/prove.rs` uses encrypted polynomial queries (a_query, b_query, etc.) and efficient multi-scalar multiplication. Refer to the implementation for production-ready code.

### Data Structures

```rust
use ark_ff::Field;
use ark_bn254::{G1Projective as G1, G2Projective as G2};

pub struct Proof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
}

pub struct PublicInputs {
    pub inputs: Vec<Scalar>,
}

pub struct PrivateWitness {
    pub values: Vec<Scalar>,
}
```

### Proof Generation Function

```rust
pub fn generate_proof(
    pk: &ProvingKey,
    witness: &PrivateWitness,
    public_inputs: &PublicInputs,
) -> Result<Proof, ProofError> {
    let mut rng = rand::thread_rng();

    // Combine witness and public inputs
    let mut full_witness = witness.values.clone();
    full_witness.extend(public_inputs.inputs.iter());

    // Evaluate QAP polynomials at tau
    let tau = pk.tau;  // From setup
    let a_tau = evaluate_a_polynomials(&pk.a_query, &full_witness);
    let b_tau = evaluate_b_polynomials(&pk.b_query, &full_witness);
    let c_tau = evaluate_c_polynomials(&pk.c_query, &full_witness);

    // Compute H(tau)
    let h_tau = compute_h(tau, &a_tau, &b_tau, &c_tau);

    // Generate random blinding factors
    let r = Fr::rand(&mut rng);
    let s = Fr::rand(&mut rng);
    // Note: delta is used in the actual G2 computations

    // Compute proof elements
    let a = pk.alpha_g1.mul(a_tau).add(
        pk.beta_g1.mul(b_tau).mul(r)
    );

    let b = pk.beta_g2.mul(b_tau);

    let c = pk.beta_g1.mul(a_tau).add(
        pk.beta_g1.mul(b_tau).mul(r)
    ).add(
        pk.delta_g1.mul(h_tau).mul(s)
    );

    Ok(Proof { a, b, c })
}

fn evaluate_a_polynomials(a_query: &[G1], witness: &[Scalar]) -> Scalar {
    witness.iter()
        .zip(a_query.iter())
        .map(|(w, a)| w * a)
        .sum()
}
```

### Witness Generation

```rust
pub fn generate_witness(
    circuit: &Circuit,
    public_inputs: &[Scalar],
    private_inputs: &[Scalar],
) -> Result<PrivateWitness, WitnessError> {
    // Allocate witness vector
    let mut witness = vec![Scalar::ONE];  // Constant 1

    // Add public inputs
    witness.extend(public_inputs);

    // Add private inputs
    witness.extend(private_inputs);

    // Verify R1CS satisfaction
    let (a, b, c) = circuit.r1cs_constraints();
    let result = check_constraints(a, b, c, &witness);

    if !result {
        return Err(WitnessError::ConstraintsNotSatisfied);
    }

    Ok(PrivateWitness { values: witness })
}
```

## Worked Example: Multiplier Circuit

Let's generate a proof for our multiplier circuit: a × b = c.

**Circuit:**
- Variables: w = [1, a, b, c]
- Public input: [c] = [15]
- Private witness: [a, b] = [3, 5]

**Setup Parameters:**
```
τ = some random value (from trusted setup)
α, β = random values (from trusted setup)
Proving key contains encrypted polynomials evaluated at τ
```

**Step 1: Generate Witness**
```
w = [1, 3, 5, 15]
```

**Step 2: Evaluate QAP at τ**

Assume our QAP (from Chapter 3) gives:
```
A(τ) = a·A₁(τ) + b·A₂(τ) + c·A₃(τ)
     = 3·A₁(τ) + 5·A₂(τ) + 15·A₃(τ)
     = some value

B(τ) = b·B₁(τ) + 1·B₀(τ)
     = 5·B₁(τ) + B₀(τ)
     = some value

C(τ) = c·C₃(τ)
     = 15·C₃(τ)
     = some value
```

**Step 3: Compute H(τ)**
```
H(τ) = (A(τ) · B(τ) - C(τ)) / t(τ)
```

**Step 4: Add Randomization**
```
r = random scalar

A = α·A(τ) + r·β·B(τ)  (in G₁)
B = B(τ)                (in G₂)
C = β·A(τ) + r·β·B(τ) + H(τ)  (in G₁)
```

**Step 5: Output Proof**
```
π = (A, B, C)
```

The proof can now be verified without revealing a=3 or b=5!

## Correctness Argument

### Why the Proof Verifies

The verifier checks:
```
e(A, B) = e(α, β) · e(public·IC, γ) · e(C, δ)
```

The proof elements A, B, C are constructed such that when combined with the public inputs (via IC) and the setup parameters (α, β, γ, δ), the pairing equation holds if and only if the prover knows a valid witness.

**Intuition:** The randomization factors (r, s, δ) are carefully chosen so they "cancel out" in the pairing equation. This allows the proof to verify correctly while hiding the actual witness values.

**Connection to Previous Chapters:**
- **Chapter 3 (QAP):** Defined the A, B, C polynomials that get evaluated
- **Chapter 4 (Pairings):** Showed how bilinear pairings work
- **Chapter 5 (Setup):** Generated the encrypted polynomial evaluations (a_query, b_query, etc.)
- **This chapter:** Combines it all to create the actual proof

### Zero-Knowledge Proof

For any valid witness, the distribution of proofs is identical regardless of the actual witness values. This is because:
1. The random blinding factors (r, s, δ) are freshly generated for each proof
2. These factors make it impossible to extract the witness from A, B, C
3. But the pairing equation still verifies because the randomization is "cryptographically neutral"

## Summary

**Key Takeaways:**
1. Proofs consist of three group elements (A ∈ G₁, B ∈ G₂, C ∈ G₁)
2. The prover evaluates QAP polynomials at τ and encrypts them
3. Random blinding factors (r, s, δ) enable zero-knowledge
4. The proving key contains encrypted polynomial evaluations at τ
5. Proofs are small (~192-256 bytes depending on compression) and fast to generate
6. Zero-knowledge means proofs reveal nothing about the private witness

**Next Chapter:** We'll see how the verifier checks proofs using the pairing equation.

## Further Reading

- [Groth16 Paper Section 4.2](https://eprint.iacr.org/2016/260) - Proof generation algorithm
- [ark-groth16 prove.rs](https://docs.rs/ark-groth16/latest/ark_groth16/fn.prove.html) - API documentation
- [Zero-Knowledge Proofs](https://en.wikipedia.org/wiki/Zero-knowledge_proof) - General background
