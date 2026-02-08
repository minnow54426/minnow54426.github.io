# Proof Generation

We have our proving key from the trusted setup. Now we can finally generate a zero-knowledge proof! This is where all the pieces come together.

## Learning Objectives

After this chapter, you will understand:
- How to generate a Groth16 proof from a witness
- The proof structure (A, B, C components)
- Random blinding and zero-knowledge
- The division polynomial H(x)
- How the proof demonstrates constraint satisfaction

## Motivating Example: Proving Knowledge

Recall our multiplier circuit: a × b = c

**Scenario**: You want to prove you know factors of 12 without revealing them.

**Goal**: Generate a proof π = (A, B, C) such that:
- The verifier can check π against public input c = 12
- The verifier learns nothing about a = 3 and b = 4
- The proof is only 128 bytes (three elliptic curve points)

## Theory Deep Dive: The Proof Structure

### The Proof Components

A Groth16 proof consists of three elliptic curve points:

```text
π = (A, B, C)
```

- **A** ∈ G₁: Proof component from A-polynomials
- **B** ∈ G₂: Proof component from B-polynomials
- **C** ∈ G₁: Proof component from C-polynomials and division polynomial

### The Proof Equations

The prover generates A, B, C to satisfy:

```text
A = [α]₁ + Σⱼ zⱼ·[Aⱼ(τ)]₁ + r·[δ]₁
B = [β]₂ + Σⱼ zⱼ·[Bⱼ(τ)]₂ + s·[δ]₂
C = Σⱼ zⱼ·[β·Aⱼ(τ) + α·Bⱼ(τ) + Cⱼ(τ)]₁ + [H(τ)]₁ + r·s·[δ]₁ - s·[β]₁ - r·[α]₁
```

Where:
- zⱼ are witness values
- r, s are random blinding factors
- [·]₁ means "encrypted in G₁" (··G₁)
- [·]₂ means "encrypted in G₂" (··G₂)

### Zero-Knowledge via Blinding

The random values **r** and **s** ensure zero-knowledge:

```text
A = ... + r·δ
B = ... + s·δ
C = ... + r·s·δ - s·β - r·α
```

**Key insight**: Different (r, s) values produce different-looking proofs for the same witness, preventing the verifier from learning anything about the witness.

### The Division Polynomial H(x)

Recall from QAP that we check:
```text
P(x) = A(x) · B(x) - C(x) = H(x) · T(x)
```

The prover computes H(x) = P(x) / T(x) and includes H(τ) in the C component.

**Why this works**:
- If the witness is valid, P(x) is divisible by T(x)
- The prover can compute H(x) = P(x) / T(x)
- H(τ)·G₁ is included in the proof
- The verifier checks that H(x) exists (using the pairing equation)

## Implementation: Proof Generation in Rust

Now let's see how proof generation is implemented.

### The Proof Structure

From `crates/groth16/src/prove.rs:24-36`:

```rust,ignore
/// Groth16 proof
///
/// A Groth16 proof consists of three group elements that demonstrate
/// knowledge of a valid witness without revealing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof {
    /// Proof component A in G₁
    pub a: G1Affine,
    /// Proof component B in G₂
    pub b: G2Affine,
    /// Proof component C in G₁
    pub c: G1Affine,
}
```

### The Main Proof Function

From `crates/groth16/src/prove.rs:89-200`:

```rust,ignore
/// Generates a Groth16 zero-knowledge proof.
pub fn generate_proof<R: Rng + ?Sized>(
    pk: &ProvingKey,
    witness: &[FieldWrapper<Fq>],
    a_polys: &[Polynomial<Fq>],
    b_polys: &[Polynomial<Fq>],
    c_polys: &[Polynomial<Fq>],
    _public_inputs: usize,
    rng: &mut R,
) -> Result<Proof, Groth16Error> {
    // Step 1: Compute A_base = Σⱼ witness[j]·Aⱼ(τ) (unblinded, without α)
    let mut a_witness_blinded = G1::zero();
    for (j, w) in witness.iter().enumerate() {
        let w_fr = field_wrapper_to_fr(w);
        let g1_point = G1::from(pk.a_query[j]);
        a_witness_blinded += g1_point * w_fr;
    }
    // Extract α contribution: α·Σ witness[j] where j=0 is the constant 1
    let alpha_sum = field_wrapper_to_fr(&witness[0]);
    let a_alpha_part = G1::from(pk.alpha_g1) * alpha_sum;
    let a_base = a_witness_blinded - a_alpha_part;

    // Step 2: Compute B_base = Σⱼ witness[j]·Bⱼ(τ) (unblinded, without β)
    let mut b_witness_g2_blinded = G2::zero();
    for (j, w) in witness.iter().enumerate() {
        let w_fr = field_wrapper_to_fr(w);
        let bg2_point = G2::from(pk.b_g2_query[j]);
        b_witness_g2_blinded += bg2_point * w_fr;
    }
    // Extract β contribution: β·Σ witness[j]
    let beta_sum = alpha_sum; // Same sum
    let b_beta_part_g2 = G2::from(pk.beta_g2) * beta_sum;
    let b_base_g2 = b_witness_g2_blinded - b_beta_part_g2;

    // Step 3: Compute C_base = Σⱼ witness[j]·Cⱼ(τ) (already has β)
    let mut c_base = G1::zero();
    for (j, w) in witness.iter().enumerate() {
        let w_fr = field_wrapper_to_fr(w);
        let g1_point = G1::from(pk.c_query[j]);
        c_base += g1_point * w_fr;
    }

    // Step 4: Generate random blinding factors
    let r = Fr::rand(rng);
    let s = Fr::rand(rng);

    // Step 5: Compute proof component A
    // A = α·G₁ + A_base + r·δ·G₁
    let delta_g1 = G1::from(pk.delta_g1);
    let a_g1 = G1::from(pk.alpha_g1) + a_base + delta_g1 * r;

    // Step 6: Compute proof component B
    // B = β·G₂ + B_base + s·δ·G₂
    let delta_g2 = G2::from(pk.delta_g2);
    let b_g2 = G2::from(pk.beta_g2) + b_base_g2 + delta_g2 * s;

    // Step 7: Compute proof component C
    // First compute the H polynomial
    let a_w_poly = compute_witness_polynomial(a_polys, witness);
    let b_w_poly = compute_witness_polynomial(b_polys, witness);
    let c_w_poly = compute_witness_polynomial(c_polys, witness);

    // Compute p(x) = A_w(x)·B_w(x) - C_w(x)
    let product_poly = a_w_poly.clone() * b_w_poly.clone();
    let diff_poly = product_poly - c_w_poly;

    // Get target polynomial t(x)
    let target_poly = groth16_qap::target_polynomial::<Fq>(num_constraints);

    // Divide to get H(x)
    let (h_poly, _remainder) = divide_polynomials(&diff_poly, &target_poly)?;

    // Compute H(τ)·G₁
    let h_at_tau = h_poly.evaluate(&tau_field);
    let h_fr = fq_to_fr(&h_at_tau.value);

    // Lookup in h_query
    let mut h_encrypted = G1::zero();
    for (j, coeff) in h_poly.coeffs.iter().enumerate() {
        if j < pk.h_query.len() {
            let h_point = G1::from(pk.h_query[j]);
            h_encrypted += h_point * fq_to_fr(&coeff.value);
        }
    }

    // C = β·A_base·s + α·B_base·r + C_base + H(τ) + r·s·δ
    let c_g1 = c_base
        + a_base.clone() * s
        + b_base_g1.clone() * r
        + h_encrypted
        + delta_g1 * (r * s)
        - G1::from(pk.beta_g1) * s
        - G1::from(pk.alpha_g1) * r;

    Ok(Proof {
        a: a_g1.into_affine(),
        b: b_g2.into_affine(),
        c: c_g1.into_affine(),
    })
}
```

### Computing the Witness Polynomial

```rust,ignore
/// Computes the witness polynomial: W(x) = Σⱼ witness[j]·Pⱼ(x)
fn compute_witness_polynomial<F>(
    polynomials: &[Polynomial<F>],
    witness: &[FieldWrapper<F>],
) -> Polynomial<F>
where
    F: PrimeField,
{
    let mut result = Polynomial::<F>::new(vec![FieldWrapper::zero()]);

    for (j, poly) in polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        result = result + scaled;
    }

    result
}
```

## Running the Code

### Example: Generating a Proof

```rust,ignore
use groth16_groth16::{trusted_setup, generate_proof};
use groth16_circuits::multiplier::MultiplierCircuit;
use groth16_qap::r1cs_to_qap;

// Create circuit
let circuit = MultiplierCircuit::new(3, 4, 12);
let witness = circuit.witness();  // [1, 12, 3, 4]

// Convert to R1CS and QAP
let constraints = circuit.to_r1cs();
let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4)?;

// Trusted setup
let (pk, vk) = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng)?;

// Generate proof
let proof = generate_proof(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &mut rng)?;

println!("Proof generated:");
println!("  A: {:?}", proof.a);
println!("  B: {:?}", proof.b);
println!("  C: {:?}", proof.c);
```

### What the Proof Contains

For the multiplier circuit (a=3, b=4, c=12):

```text
A = α·G₁ + 3·A₂(τ)·G₁ + r·δ·G₁
  = α·G₁ + 3·G₁ + r·δ·G₁  (since A₂(x) = 1)

B = β·G₂ + 4·B₃(τ)·G₂ + s·δ·G₂
  = β·G₂ + 4·G₂ + s·δ·G₂  (since B₃(x) = 1)

C = ... (complex formula involving H(τ))
```

The verifier only sees the elliptic curve points, not the witness values!

## Security Properties

### Zero-Knowledge

The random values r and s ensure that:
- Different proofs for the same witness look different
- The verifier cannot deduce the witness from the proof
- No information about a and b is leaked

### Soundness

A malicious prover cannot create a valid proof for an invalid witness because:
- The pairing equation (Chapter 7) would fail
- They would need to know α, β, γ, δ (toxic waste)
- The division polynomial H(x) wouldn't exist for invalid witnesses

### Proof Size

A Groth16 proof is only **128 bytes**:
- A ∈ G₁: 32 bytes (compressed)
- B ∈ G₂: 64 bytes (compressed)
- C ∈ G₁: 32 bytes (compressed)

This is constant regardless of circuit complexity!

## Connection to Groth16

Proof generation is the **proving phase** of Groth16:

```text
Computation
    ↓
R1CS (matrix constraints)
    ↓
QAP (polynomial divisibility)
    ↓
Elliptic Curve Pairings
    ↓
Trusted Setup
    ↓
Proof Generation ← You are here
    ↓
Proof Verification (Chapter 7)
```

## Exercises

1. **Blinding factors**:
   ```text
   What happens if r = 0 and s = 0?
   Is the proof still zero-knowledge?
   ```

2. **Division polynomial**:
   ```text
   For the multiplier circuit, what is H(x)?
   Hint: P(x) = 3·4 - 12 = 0, so H(x) = 0 / T(x) = 0
   ```

3. **Proof size**:
   ```text
   Why is the proof size constant?
   Why doesn't it grow with circuit complexity?
   ```

4. **Challenge question**:
   ```text
   Can a prover generate a proof without knowing the witness?
   What would prevent this?
   ```

## Further Reading

- **Groth16 Paper**: [On the Size of Pairing-based Non-Interactive Arguments](https://eprint.iacr.org/2016/260)
- **Zero-Knowledge Proofs**: [ZK-SNARKs Explained](https://zkp.science/)
- **Proof Systems**: [Proof Systems Survey](https://eprint.iacr.org/2019/953)

---

**Ready to verify proofs? Continue to [Chapter 7: Proof Verification](./07-proof-verification.md)**
