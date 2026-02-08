# Fix Plan: Groth16 Field Type Mismatch

## Root Cause
QAP uses `ark_bn254::Fq` (base field) but EC operations need `ark_bn254::Fr` (scalar field).

## Recommended Fix: Option 2 (Fix Field Types)

### Step 1: Add Fr Support to Math Crate (30 min)

```rust
// crates/math/src/fields.rs
pub type FrField = ark_bn254::Fr;

// Add Fr wrapper if needed for consistency
pub type FrWrapper = FieldWrapper<ark_bn254::Fr>;
```

### Step 2: Make QAP Generic Over Field Type (1 hour)

```rust
// crates/qap/src/polynomials.rs
pub fn r1cs_to_qap<F>(
    constraints: &[R1CSConstraint<F>],
    num_variables: usize,
) -> Result<QapPolynomials<F>, QapError>
where
    F: PrimeField,  // Already generic - just use Fr instead of Fq
```

### Step 3: Update Groth16 to Use Fr (30 min)

```rust
// crates/groth16/src/prove.rs
// Change all FieldWrapper<Fq> to FieldWrapper<Fr>
// Remove fq_to_fr conversions
// Use Fr directly throughout
```

### Step 4: Update Circuits (30 min)

```rust
// crates/circuits/src/*.rs
// Change constraint fields from Fq to Fr
```

### Step 5: Test (15 min)

```bash
cargo test
# Should now pass all 18 tests
```

## Alternative: Option 3 (Quick Patch, 30 min)

Replace proof generation with arkworks:

```rust
// crates/groth16/src/prove.rs
pub fn generate_proof(...) -> Result<Proof, Groth16Error> {
    // Convert our types to arkworks types
    let ark_circuit = convert_to_arkworks_circuit(witness);
    let ark_proof = ark_groth16::Groth16::create_proof_with_reduction(
        ark_circuit,
        &pk.ark_pk,
        r, s
    )?;
    // Convert back to our types
    Ok(convert_from_arkworks_proof(ark_proof))
}
```

## My Recommendation

**Start with Option 3** (30 min) to get tests passing quickly.
Then **Option 2** (1 day) for clean, educational implementation.

Shall I implement Option 3 first to unblock you?
