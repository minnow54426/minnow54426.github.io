# Proof Verification

We've generated our proof. Now the verifier needs to check that it's valid without learning anything about the witness. This is where the pairing equation comes in!

## Learning Objectives

After this chapter, you will understand:
- The Groth16 verification equation
- How pairings enable efficient verification
- Constant-time verification regardless of circuit size
- Batch verification for efficiency
- Security properties of verification

## Motivating Example: The Verification Check

The prover sends:
- Proof π = (A, B, C)
- Public inputs (e.g., c = 12 for the multiplier)

The verifier needs to check:
1. The proof was generated correctly
2. The public inputs match the witness
3. The witness satisfies all constraints
4. No information about the witness is leaked

**All in constant time!**

## Theory Deep Dive: The Verification Equation

### The Pairing Equation

A Groth16 proof is valid if and only if:

```text
e(A, B) = e(α, β) · e(Σ publicᵢ·ICᵢ, γ) · e(C, δ)
```

Where:
- **e(·, ·)** is the bilinear pairing
- **A, B, C** are the proof components
- **α, β, γ, δ** are from the verification key
- **ICᵢ** are the input consistency elements
- **publicᵢ** are the public inputs

### Expanding the Equation

Let's expand what each component means:

**Left side**:
```text
e(A, B) = e(α·G₁ + A_witness + r·δ·G₁, β·G₂ + B_witness + s·δ·G₂)
```

**Right side**:
```text
e(α, β) · e(Σ public·IC, γ) · e(C, δ)
```

When the proof is correctly generated, these two sides are equal!

### Why This Works

The verification equation checks three things:

1. **Proof structure**: A and B use the correct secrets (α, β, δ)
2. **Public input consistency**: The witness matches the claimed public inputs
3. **QAP satisfaction**: The division polynomial H(x) exists

If any of these fail, the pairing equation won't hold!

### The Public Input Term

The term **e(Σ publicᵢ·ICᵢ, γ)** ensures public input consistency:

```text
IC[0] = β·G₁ (for the constant 1)
IC[i] = β·Aᵢ(τ)·G₁ for i = 1..num_inputs

Σ publicᵢ·ICᵢ = public₀·IC[0] + public₁·IC[1] + ...
              = 1·β·G₁ + c·β·A₁(τ)·G₁
              = β·(1 + c·A₁(τ))·G₁
```

This ensures the witness used the correct public inputs!

## Implementation: Verification in Rust

Now let's see how verification is implemented.

### The Main Verification Function

From `crates/groth16/src/verify.rs:66-167`:

```rust,ignore
/// Verifies a Groth16 zero-knowledge proof.
pub fn verify_proof(
    vk: &VerificationKey,
    proof: &Proof,
    public_inputs: &[FieldWrapper<Fq>],
) -> Result<bool, Groth16Error> {
    // Helper function to convert FieldWrapper<Fq> to Fr
    fn fq_to_fr(fq: &Fq) -> <Bn254 as Pairing>::ScalarField {
        let bytes = fq.into_bigint().to_bytes_be();
        let mut padded = [0u8; 32];
        let start = 32usize.saturating_sub(bytes.len());
        padded[start..].copy_from_slice(&bytes);
        <Bn254 as Pairing>::ScalarField::from_be_bytes_mod_order(&padded)
    }

    // Step 1: Compute left side of verification equation
    // Left side: e(A, B)
    let left = Bn254::pairing(proof.a, proof.b);

    // Step 2: Compute right side components
    // Component 1: e(α, β)
    let alpha_beta = Bn254::pairing(vk.alpha_g1, vk.beta_g2);

    // Component 2: e(Σpublic_i·IC_i, γ)
    // Check if IC includes the constant (IC length = public_inputs + 1)
    let has_constant = vk.ic.len() == public_inputs.len() + 1;

    let mut public_acc = G1::zero();

    if has_constant {
        // IC[0] is for constant 1, IC[1..] are for public inputs
        if !vk.ic.is_empty() {
            public_acc += G1::from(vk.ic[0]);
        }
        for (i, input) in public_inputs.iter().enumerate() {
            if i + 1 < vk.ic.len() {
                let input_scalar = fq_to_fr(&input.value);
                let ic_point = G1::from(vk.ic[i + 1]);
                public_acc += ic_point * input_scalar;
            }
        }
    }

    let public_gamma = Bn254::pairing(public_acc, vk.gamma_g2);

    // Component 3: e(C, δ)
    let c_delta = Bn254::pairing(proof.c, vk.delta_g2);

    // Right side: e(α, β) · e(Σpublic·IC, γ) · e(C, δ)
    let right_field = alpha_beta.0 * public_gamma.0 * c_delta.0;

    // Step 3: Check if verification equation holds
    let is_valid = left.0 == right_field;

    Ok(is_valid)
}
```

### The Verification Algorithm

1. **Compute left side**: `e(A, B)` using the pairing
2. **Compute right side**:
   - `e(α, β)` from the verification key
   - `e(Σpublic·IC, γ)` by combining public inputs with IC elements
   - `e(C, δ)` from the proof and verification key
3. **Multiply the right side components**: `e(α, β) · e(Σpublic·IC, γ) · e(C, δ)`
4. **Compare**: Check if left equals right

## Running the Code

### Example: Verifying a Proof

```rust,ignore
use groth16_groth16::{trusted_setup, generate_proof, verify_proof};
use groth16_circuits::multiplier::MultiplierCircuit;
use groth16_qap::r1cs_to_qap;

// Setup and proof generation
let circuit = MultiplierCircuit::new(3, 4, 12);
let witness = circuit.witness();
let constraints = circuit.to_r1cs();
let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4)?;
let (pk, vk) = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng)?;
let proof = generate_proof(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &mut rng)?;

// Verify with public input c=12
let public_inputs = vec![FieldWrapper::<Fq>::from(12u64)];
let is_valid = verify_proof(&vk, &proof, &public_inputs)?;

assert!(is_valid);  // Proof should be valid
println!("Proof verification: {}", is_valid);
```

### Example: Invalid Proof

```rust,ignore
// Try to verify with wrong public input
let wrong_public_inputs = vec![FieldWrapper::<Fq>::from(13u64)];  // Should be 12
let is_valid = verify_proof(&vk, &proof, &wrong_public_inputs)?;

assert!(!is_valid);  // Proof should be invalid
```

## Efficiency

### Constant-Time Verification

Verification is **O(1)** regardless of circuit size:
- 3 pairing operations (constant time)
- O(n) field multiplications for public inputs (usually small n)
- No computation over constraints!

For a circuit with 1 million constraints:
- Proving time: O(n) ~ several seconds
- Verification time: O(1) ~ a few milliseconds

### Batch Verification

We can verify multiple proofs efficiently using random linear combination:

From `crates/groth16/src/verify.rs:169-240`:

```rust,ignore
/// Batch verifies multiple Groth16 proofs efficiently using random linear combination.
///
/// Instead of O(n) pairing operations for n proofs, we use O(1) pairings total.
pub fn batch_verify<R: Rng>(
    vk: &VerificationKey,
    proofs: &[Proof],
    public_inputs_list: &[Vec<FieldWrapper<Fq>>],
    rng: &mut R,
) -> Result<bool, Groth16Error>
{
    // Generate random coefficients for each proof
    let mut coeffs = Vec::with_capacity(proofs.len());
    for _ in 0..proofs.len() {
        let r = Fr::rand(rng);
        coeffs.push(r);
    }

    // Compute combined left side: e(Σ rᵢ·Aᵢ, Bᵢ)
    let mut combined_a = G1::zero();
    for (i, proof) in proofs.iter().enumerate() {
        combined_a += G1::from(proof.a) * coeffs[i];
    }
    let left = Bn254::pairing(combined_a, proofs[0].b);

    // Compute combined right side
    // ... (similar combination for right side)

    // Single pairing check for all proofs
    Ok(left.0 == right.0)
}
```

**Speedup**: For n proofs:
- Individual: ~3n pairings
- Batch: ~3 pairings (constant!)
- Speedup: Approximately n× faster

## Security Properties

### Soundness

A false proof will be rejected with overwhelming probability:
- The pairing equation would fail
- Cannot forge without knowing toxic waste
- Cryptographic assumptions: ECDLP, SDH

### Zero-Knowledge

Verification reveals nothing about the witness:
- Only public inputs and the proof are visible
- Pairings don't leak information about discrete logs
- Blinding factors (r, s) ensure proof diversity

### Succinctness

Verification is incredibly efficient:
- Constant-time regardless of circuit complexity
- Small proof size (128 bytes)
- Fast verification (~milliseconds)

## Connection to Groth16

Verification is the **final step** of Groth16:

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
Proof Generation
    ↓
Proof Verification ← You are here
```

## Exercises

1. **Verification equation**:
   ```text
   Why do we need e(α, β) on the right side?
   What would happen if we removed it?
   ```

2. **Public input consistency**:
   ```text
   How does the IC vector ensure public inputs are correct?
   What happens if the prover lies about public inputs?
   ```

3. **Batch verification**:
   ```text
   Why does batch verification work?
   What's the security requirement for the random coefficients?
   ```

4. **Challenge question**:
   ```text
   Can we verify a proof without the verification key?
   What parts of the VK are essential?
   ```

## Further Reading

- **Pairing Verification**: [Pairing-Based Cryptography](https://www.cryptologie.net/article/328/pairing-based-cryptography/)
- **Batch Verification**: [Batch Verification Techniques](https://eprint.iacr.org/2020/1628)
- **Groth16 Security**: [Groth16 Security Proof](https://eprint.iacr.org/2016/260)

---

**Ready to build circuits? Continue to [Chapter 8: Building Your Own Circuits](./08-building-circuits.md)**
