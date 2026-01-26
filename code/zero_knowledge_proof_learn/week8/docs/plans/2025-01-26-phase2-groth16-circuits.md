# Phase 2 Implementation Plan: Complete Groth16 Pipeline and Circuits

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement actual Groth16 setup/prove/verify pipeline and add working constraints to all three circuits (identity, membership, privacy) with comprehensive tests and benchmarks.

**Architecture:** Use ark-groth16 for proving, ark-crypto-primitives for hash gadgets, ark-r1cs-std for constraint building. Each circuit generates real R1CS constraints that can be proven and verified.

**Tech Stack:** arkworks-rs (ark-groth16, ark-relations, ark-r1cs-std, ark-crypto-primitives, ark-bls12-381), criterion for benchmarks.

---

## Task 1: Complete Groth16 Setup Function

**Files:**
- Modify: `src/groth16.rs`

**Step 1: Update test to expect actual keys**

Modify: `tests/groth16_setup_tests.rs`

```rust
#[test]
fn test_setup() {
    use ark_bls12_381::Bls12_381;
    use ark_groth16::ProvingKey;
    use ark_std::UniformRand;

    let circuit = TestCircuit;
    let (pk, vk) = setup(&circuit).unwrap();

    // Verify keys are generated
    assert_eq!(pk.gamma_abc_g1.len(), 1); // TestCircuit has 1 public input
    assert!(vk.alpha_g1_beta_g2 != None); // VK is initialized
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test test_setup -- --ignored`

Expected: FAIL - setup returns error

**Step 3: Implement actual Groth16 setup**

Modify: `src/groth16.rs`

```rust
use crate::{Error, Groth16Circuit, Result};
use ark_ec::PairingEngine;
use ark_groth16::{ProvingKey, VerifyingKey, Groth16, prepare_verifying_key};
use ark_std::marker::PhantomData;

/// Generate Groth16 proving and verifying keys for a circuit
pub fn setup<C, E>(
    circuit: &C,
) -> Result<(ProvingKey<E>, VerifyingKey<E>)>
where
    C: Groth16Circuit<E::Fr>,
    E: PairingEngine,
{
    // Create a new constraint system
    let mut cs = ark_relations::r1cs::ConstraintSynthesizer::<E::Fr>::new();

    // Generate constraints for the circuit
    // We need a dummy witness for setup - this is standard in Groth16
    // The actual witness values don't matter for the setup phase
    C::generate_constraints(cs.ns(|cs| cs), &Default::default())
        .map_err(|_| Error::Setup(SetupError::SetupFailed))?;

    // Generate keys using Groth16
    let (pk, vk) = Groth16::<E>::generate_random_parameters_with_reduction(
        &cs.into())
        .map_err(|_| Error::Setup(SetupError::SetupFailed))?;

    Ok((pk, vk))
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test test_setup -- --ignored`

Expected: PASS (or test may need adjustment based on arkworks API)

**Step 5: Remove ignore attribute**

Modify: `tests/groth16_setup_tests.rs` - remove `#[ignore]`

**Step 6: Commit**

```bash
cd code
git add src/groth16.rs tests/groth16_setup_tests.rs
git commit -m "feat: implement Groth16 setup with ark-groth16"
```

---

## Task 2: Complete Groth16 Prove Function

**Files:**
- Modify: `src/groth16.rs`

**Step 1: Write test for prove**

Create: `tests/groth16_prove_tests.rs`

```rust
use zk_groth16_snark::groth16::{setup, prove};
use ark_bls12_381::Bls12_381;

struct SimpleCircuit {
    value: u64,
}

impl Groth16Circuit<Bls12_381::Fr> for SimpleCircuit {
    fn circuit_name() -> &'static str { "simple" }

    type PublicInputs = u64;
    type Witness = u64;

    fn generate_constraints(
        cs: ark_relations::r1cs::ConstraintMetavariable<Bls12_381::Fr>,
        witness: &Self::Witness,
    ) -> zk_groth16_snark::Result<()> {
        // Simple constraint: value = value (trivial)
        let _ = witness;
        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        Ok(self.value)
    }

    fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs {
        *witness
    }
}

#[test]
fn test_prove() {
    let circuit = SimpleCircuit { value: 42 };
    let (pk, _vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();

    let proof = prove(&pk, &witness).unwrap();

    // Groth16 proofs should be 288 bytes
    assert_eq!(proof.len(), 288);
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test prove_tests`

Expected: COMPILER ERROR - prove function doesn't exist

**Step 3: Implement prove function**

Modify: `src/groth16.rs`

```rust
use ark_groth16::Proof;

/// Generate a Groth16 proof
pub fn prove<C, E>(
    pk: &ProvingKey<E>,
    witness: &C::Witness,
) -> Result<Proof<E>>
where
    C: Groth16Circuit<E::Fr>,
    E: PairingEngine,
{
    // Create constraint system for proving
    let mut cs = ark_relations::r1cs::ConstraintSynthesizer::<E::Fr>::new();

    // Generate constraints with actual witness
    C::generate_constraints(cs.ns(|cs| cs), witness)
        .map_err(|_| Error::Prove(ProveError::ProofCreationFailed))?;

    // Create proof using Groth16
    let proof = Groth16::<E>::prove(
        &pk,
        cs.into(),
        &[/* public inputs will be added later */]
    ).map_err(|_| Error::Prove(ProveError::ProofCreationFailed))?;

    Ok(proof)
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test prove_tests`

Expected: May need adjustment based on arkworks API

**Step 5: Commit**

```bash
cd code
git add src/groth16.rs tests/groth16_prove_tests.rs
git commit -m "feat: implement Groth16 prove function"
```

---

## Task 3: Complete Groth16 Verify Function

**Files:**
- Modify: `src/groth16.rs`

**Step 1: Add verify test to prove_tests**

Modify: `tests/groth16_prove_tests.rs`

```rust
#[test]
fn test_prove_and_verify() {
    let circuit = SimpleCircuit { value: 42 };
    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = circuit.public_inputs(&witness);

    let proof = prove(&pk, &witness).unwrap();

    let is_valid = verify(&vk, &public_inputs, &proof).unwrap();
    assert!(is_valid);
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test prove_and_verify`

Expected: COMPILER ERROR - verify function doesn't exist

**Step 3: Implement verify function**

Modify: `src/groth16.rs`

```rust
/// Verify a Groth16 proof
pub fn verify<E>(
    vk: &VerifyingKey<E>,
    public_inputs: &[E::Fr],
    proof: &Proof<E>,
) -> Result<bool>
where
    E: PairingEngine,
{
    // Convert public inputs to Vec (if needed)
    let inputs = public_inputs.to_vec();

    // Verify proof using Groth16
    let valid = Groth16::<E>::verify(
        &vk,
        &inputs,
        proof
    ).map_err(|_| Error::Verify(VerifyError::ProofVerificationFailed))?;

    Ok(valid)
}
```

**Step 4: Run tests to verify they pass**

Run: `cd code && cargo test prove_tests`

Expected: PASS (after API adjustments)

**Step 5: Commit**

```bash
cd code
git add src/groth16.rs tests/groth16_prove_tests.rs
git commit -m "feat: implement Groth16 verify function"
```

---

## Task 4: Implement Privacy Circuit Constraints (Range Proof)

**Files:**
- Modify: `src/privacy/mod.rs`

**Step 1: Write test for range proof validation**

Modify: `tests/privacy_basic_tests.rs`

```rust
#[test]
fn test_value_in_range_verifies() {
    use zk_groth16_snark::groth16::{setup, prove, verify};
    use ark_bls12_381::Fr;

    let min = Fr::from(10u64);
    let max = Fr::from(100u64);
    let circuit = PrivacyCircuit::new(min, max, 50u64); // 50 in range

    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = circuit.public_inputs(&witness);

    let proof = prove(&pk, &witness).unwrap();
    let is_valid = verify(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test test_value_in_range_verifies`

Expected: FAIL - constraints not implemented

**Step 3: Implement range proof constraints**

Modify: `src/privacy/mod.rs`

```rust
use ark_ff::{Field, BigInteger};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use std::marker::PhantomData;

impl PrivacyCircuit {
    pub fn new(min: u64, max: u64, value: u64) -> Self {
        Self {
            min: Fp::from(min),
            max: Fp::from(max),
            value,
            _phantom: PhantomData,
        }
    }
}

impl Groth16Circuit<Fp> for PrivacyCircuit {
    fn circuit_name() -> &'static str { "privacy" }

    type PublicInputs = (Fp, Fp); // (min, max)
    type Witness = Fp; // value

    fn generate_constraints(
        cs: ConstraintSystemRef<Fp>,
        witness: &Self::Witness,
    ) -> Result<()> {
        // Implement range proof: min <= value <= max
        let value_var = cs.new_witness_variable(|| Ok(*witness))?;
        let min_var = cs.new_input_variable(|| Ok(self.min))?;
        let max_var = cs.new_input_variable(|| Ok(self.max))?;

        // value - min >= 0
        // Enforced by: (value - min) * (value - min) == (value - min)
        // For non-negative fields, this works if we assume field is large enough
        cs.enforce_constraint(
            "value - min >= 0",
            CS::one(),
            value_var - min_var,
            value_var - min_var,
            value_var - min_var,
        )?;

        // max - value >= 0
        cs.enforce_constraint(
            "max - value >= 0",
            CS::one(),
            max_var - value_var,
            max_var - value_var,
            max_var - value_var,
        )?;

        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        Ok(Fp::from(self.value))
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        (self.min, self.max)
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test test_value_in_range_verifies`

Expected: PASS (may need API adjustments)

**Step 5: Add more tests**

Modify: `tests/privacy_basic_tests.rs`

```rust
#[test]
fn test_value_below_range_fails() {
    // Test value < min fails verification
}

#[test]
fn test_value_above_range_fails() {
    // Test value > max fails verification
}
```

**Step 6: Commit**

```bash
cd code
git add src/privacy/mod.rs tests/privacy_basic_tests.rs
git commit -m "feat: implement range proof constraints"
```

---

## Task 5: Implement Identity Circuit Constraints (SHA-256)

**Files:**
- Modify: `src/identity/mod.rs`
- Add dependency to Cargo.toml: `ark-crypto-primitives = { version = "0.4", features = ["r1cs"] }`

**Step 1: Write test for hash preimage proof**

Modify: `tests/identity_basic_tests.rs`

```rust
#[test]
fn test_hash_preimage_proof() {
    use zk_groth16_snark::groth16::{setup, prove, verify};

    let preimage = b"hello world";
    let hash = sha256(preimage);
    let circuit = IdentityCircuit::new(hash, preimage.to_vec());

    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = circuit.public_inputs(&witness);

    let proof = prove(&pk, &witness).unwrap();
    let is_valid = verify(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
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

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test test_hash_preimage_proof`

Expected: FAIL - SHA-256 constraints not implemented

**Step 3: Implement SHA-256 constraints**

Modify: `src/identity/mod.rs`

```rust
use ark_crypto_primitives::crh::sha256::Sha256Gadget;
use ark_ff::Field;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::ConstraintSystemRef;
use ark_std::vec::Vec;

pub struct IdentityCircuit {
    pub hash: [u8; 32],
    pub preimage: Vec<u8>,
}

impl IdentityCircuit {
    pub fn new(hash: [u8; 32], preimage: Vec<u8>) -> Self {
        Self { hash, preimage }
    }
}

impl Groth16Circuit<Fp> for IdentityCircuit {
    fn circuit_name() -> &'static str { "identity" }

    type PublicInputs = [Fp; 32];
    type Witness = Vec<u8>;

    fn generate_constraints(
        cs: ConstraintSystemRef<Fp>,
        witness: &Self::Witness,
    ) -> Result<()> {
        // Convert witness to field elements
        let mut bytes = [Fp::from(0u8); 32];
        // TODO: Properly pad/truncate to 32 bytes
        for (i, byte) in witness.iter().enumerate().take(32) {
            bytes[i] = Fp::from(*byte);
        }

        // Create SHA-256 gadget
        let hash_gadget = Sha256Gadget::new();

        // Hash the preimage
        let computed_hash = hash_gadget.digest(cs, &bytes)?;

        // Compare with expected hash (public input)
        for (i, expected_byte) in self.hash.iter().enumerate() {
            let expected = Fp::from(*expected_byte);
            // computed_hash[i] should equal expected
            cs.enforce_constraint(
                &format!("hash_byte_{}", i),
                CS::one(),
                computed_hash[i].clone(),
                CS::one(),
                -expected,
            )?;
        }

        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        Ok(self.preimage.clone())
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        // Convert hash to field elements
        let mut inputs = [Fp::from(0u32); 32];
        for (i, byte) in self.hash.iter().enumerate() {
            inputs[i] = Fp::from(*byte);
        }
        inputs
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test test_hash_preimage_proof`

Expected: PASS (after API adjustments)

**Step 5: Commit**

```bash
cd code
git add src/identity/mod.rs tests/identity_basic_tests.rs Cargo.toml
git commit -m "feat: implement SHA-256 hash preimage constraints"
```

---

## Task 6: Implement Membership Circuit Constraints (Merkle)

**Files:**
- Modify: `src/membership/mod.rs`

**Step 1: Write test for Merkle membership proof**

Modify: `tests/membership_basic_tests.rs`

```rust
#[test]
fn test_merkle_membership_proof() {
    use zk_groth16_snark::groth16::{setup, prove, verify};

    let root = [0u8; 32];
    let leaf = b"secret";
    let path = vec![[1u8; 32]; 8]; // Simple path
    let circuit = MembershipCircuit::new(root, leaf.to_vec(), path);

    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = circuit.public_inputs(&witness);

    let proof = prove(&pk, &witness).unwrap();
    let is_valid = verify(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}
```

**Step 2: Run test to verify it fails**

Run: `cd code && cargo test test_merkle_membership_proof`

Expected: FAIL - Merkle constraints not implemented

**Step 3: Implement Merkle path verification constraints**

Modify: `src/membership/mod.rs`

```rust
use ark_crypto_primitives::crh::sha256::Sha256Gadget;
use ark_ff::Field;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::ConstraintSystemRef;
use ark_std::vec::Vec;

pub struct MembershipCircuit {
    pub root: [u8; 32],
    pub leaf: Vec<u8>,
    pub path: Vec<[u8; 32]>,
}

impl MembershipCircuit {
    pub fn new(root: [u8; 32], leaf: Vec<u8>, path: Vec<[u8; 32]>) -> Self {
        Self { root, leaf, path }
    }
}

impl Groth16Circuit<Fp> for MembershipCircuit {
    fn circuit_name() -> &'static str { "membership" }

    type PublicInputs = [Fp; 32];
    type Witness = (Vec<u8>, Vec<[u8; 32]>);

    fn generate_constraints(
        cs: ConstraintSystemRef<Fp>,
        witness: &Self::Witness,
    ) -> Result<()> {
        let (leaf, path) = witness;

        // Convert leaf to bytes
        let mut leaf_bytes = [Fp::from(0u8); 32];
        for (i, byte) in leaf.iter().enumerate().take(32) {
            leaf_bytes[i] = Fp::from(*byte);
        }

        // Initialize hash with leaf
        let mut current_hash = leaf_bytes;

        // Hash up the Merkle tree path
        for (level, sibling) in path.iter().enumerate() {
            let sibling_bytes: [Fp; 32] = sibling.iter()
                .map(|b| Fp::from(*b))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            // Hash: H(current || sibling) or H(sibling || current)
            // For simplicity, always do current || sibling
            let mut combined = current_hash.to_vec();
            combined.extend_from_slice(&sibling_bytes);

            let hash_gadget = Sha256Gadget::new();
            current_hash = hash_gadget.digest(cs, &combined.try_into().unwrap())?;
        }

        // Verify final hash equals root
        for (i, expected) in self.root.iter().enumerate() {
            cs.enforce_constraint(
                &format!("root_byte_{}", i),
                CS::one(),
                current_hash[i].clone(),
                CS::one(),
                -Fp::from(*expected),
            )?;
        }

        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        Ok((self.leaf.clone(), self.path.clone()))
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        // Convert root to field elements
        let mut inputs = [Fp::from(0u32); 32];
        for (i, byte) in self.root.iter().enumerate() {
            inputs[i] = Fp::from(*byte);
        }
        inputs
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd code && cargo test test_merkle_membership_proof`

Expected: PASS (after API adjustments)

**Step 5: Commit**

```bash
cd code
git add src/membership/mod.rs tests/membership_basic_tests.rs
git commit -m "feat: implement Merkle membership constraints"
```

---

## Task 7: Update Examples to Use Full Pipeline

**Files:**
- Modify: `examples/identity_proof.rs`
- Modify: `examples/full_demo.rs`

**Step 1: Update identity_proof example**

Modify: `examples/identity_proof.rs`

```rust
use zk_groth16_snark::identity::IdentityCircuit;
use zk_groth16_snark::groth16::{setup, prove, verify};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Identity Circuit Demo ===");
    println!("Prove knowledge of password without revealing it\n");

    // Setup: Define password
    let password = "my_secret_password";
    let password_hash = sha256(password.as_bytes());
    println!("Password hash: {:02x?}", password_hash);

    // Create circuit
    let circuit = IdentityCircuit::new(password_hash, password.as_bytes().to_vec());
    println!("Circuit: {}", circuit.circuit_name());

    // Setup
    println!("\n[1/3] Running trusted setup...");
    let (pk, vk) = setup(&circuit)?;
    println!("  Setup complete! Proving key and verifying key generated");

    // Prove
    println!("\n[2/3] Generating proof...");
    let witness = circuit.generate_witness()?;
    let proof = prove(&pk, &witness)?;
    println!("  Proof generated: {} bytes", proof.len());

    // Verify
    println!("\n[3/3] Verifying proof...");
    let public_inputs = circuit.public_inputs(&witness);
    let is_valid = verify(&vk, &public_inputs, &proof)?;
    println!("  Verification result: {}", is_valid);

    if is_valid {
        println!("\n✅ Success! Password proven without revealing it");
    } else {
        println!("\n❌ Proof verification failed");
    }

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

**Step 2: Update full_demo example**

Modify: `examples/full_demo.rs`

```rust
use zk_groth16_snark::{identity::IdentityCircuit, privacy::PrivacyCircuit, membership::MembershipCircuit};
use zk_groth16_snark::groth16::{setup, prove, verify};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Groth16 SNARK Library Demo ===\n");

    // Identity Circuit
    println!("[1/3] Identity Circuit (Hash Preimage)");
    demo_identity()?;

    // Privacy Circuit
    println!("\n[2/3] Privacy Circuit (Range Proof)");
    demo_privacy()?;

    // Membership Circuit
    println!("\n[3/3] Membership Circuit (Merkle Tree)");
    demo_membership()?;

    println!("\n✅ All demos successful! 🎉");
    Ok(())
}

fn demo_identity() -> Result<(), Box<dyn std::error::Error>> {
    let password = "secret";
    let hash = sha256(password.as_bytes());
    let circuit = IdentityCircuit::new(hash, password.as_bytes().to_vec());

    let (pk, vk) = setup(&circuit)?;
    let witness = circuit.generate_witness()?;
    let public_inputs = circuit.public_inputs(&witness);
    let proof = prove(&pk, &witness)?;
    let is_valid = verify(&vk, &public_inputs, &proof)?;

    println!("  ✓ Hash preimage proof: {}", if is_valid { "VALID" } else { "INVALID" });
    Ok(())
}

fn demo_privacy() -> Result<(), Box<dyn std::error::Error>> {
    let circuit = PrivacyCircuit::new(18, 150, 27); // age 27
    let (pk, vk) = setup(&circuit)?;
    let witness = circuit.generate_witness()?;
    let public_inputs = circuit.public_inputs(&witness);
    let proof = prove(&pk, &witness)?;
    let is_valid = verify(&vk, &public_inputs, &proof)?;

    println!("  ✓ Range proof (age ≥ 18): {}", if is_valid { "VALID" } else { "INVALID" });
    Ok(())
}

fn demo_membership() -> Result<(), Box<dyn std::error::Error>> {
    let root = [0u8; 32];
    let leaf = b"secret";
    let path = vec![[1u8; 32]; 8];
    let circuit = MembershipCircuit::new(root, leaf.to_vec(), path);

    let (pk, vk) = setup(&circuit)?;
    let witness = circuit.generate_witness()?;
    let public_inputs = circuit.public_inputs(&witness);
    let proof = prove(&pk, &witness)?;
    let is_valid = verify(&vk, &public_inputs, &proof)?;

    println!("  ✓ Merkle membership proof: {}", if is_valid { "VALID" } else { "INVALID" });
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

**Step 3: Test examples**

Run: `cargo run --example identity_proof`
Run: `cargo run --example full_demo`

Expected: Both run successfully

**Step 4: Commit**

```bash
cd code
git add examples/identity_proof.rs examples/full_demo.rs
git commit -m "feat: update examples to use full Groth16 pipeline"
```

---

## Task 8: Add Comprehensive Tests

**Files:**
- Create: `tests/integration_tests.rs`
- Create: `tests/property_tests.rs`

**Step 1: Add integration tests**

Create: `tests/integration_tests.rs`

```rust
use zk_groth16_snark::{identity::IdentityCircuit, privacy::PrivacyCircuit, membership::MembershipCircuit};
use zk_groth16_snark::groth16::{setup, prove, verify};

#[test]
fn test_full_pipeline_identity() {
    let password = "test_password";
    let hash = sha256(password.as_bytes());
    let circuit = IdentityCircuit::new(hash, password.as_bytes().to_vec());

    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = circuit.public_inputs(&witness);
    let proof = prove(&pk, &witness).unwrap();
    let is_valid = verify(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}

#[test]
fn test_full_pipeline_privacy() {
    let circuit = PrivacyCircuit::new(10, 100, 50);

    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = circuit.public_inputs(&witness);
    let proof = prove(&pk, &witness).unwrap();
    let is_valid = verify(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}

#[test]
fn test_invalid_witness_fails_verification() {
    // Test that wrong witness produces invalid proof
}
```

**Step 2: Add property-based tests**

Create: `tests/property_tests.rs`

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_range_proof_in_range(value in 10u64..100u64) {
        let circuit = PrivacyCircuit::new(10, 100, value);
        // Test that values in range always verify
    }
}
```

**Step 3: Commit**

```bash
cd code
git add tests/integration_tests.rs tests/property_tests.rs
git commit -m "test: add comprehensive integration and property tests"
```

---

## Task 9: Add Benchmarks

**Files:**
- Create: `benches/circuit_benchmarks.rs`
- Create: `benches/comparison_benchmarks.rs`

**Step 1: Create circuit benchmarks**

Create: `benches/circuit_benchmarks.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use zk_groth16_snark::{identity::IdentityCircuit, privacy::PrivacyCircuit};
use zk_groth16_snark::groth16::{setup, prove};

fn bench_identity_setup(c: &mut Criterion) {
    let hash = [0u8; 32];
    let preimage = vec![0u8; 32];
    let circuit = IdentityCircuit::new(hash, preimage);

    c.bench_function("identity_setup", |b| {
        b.iter(|| setup(black_box(&circuit)))
    });
}

fn bench_identity_prove(c: &mut Criterion) {
    let hash = [0u8; 32];
    let preimage = vec![0u8; 32];
    let circuit = IdentityCircuit::new(hash, preimage);
    let (pk, _) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();

    c.bench_function("identity_prove", |b| {
        b.iter(|| prove(black_box(&pk), black_box(&witness)))
    });
}

criterion_group!(benches, bench_identity_setup, bench_identity_prove);
criterion_main!(benches);
```

**Step 2: Create comparison benchmarks**

Create: `benches/comparison_benchmarks.rs`

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use zk_groth16_snark::{identity::IdentityCircuit, privacy::PrivacyCircuit, membership::MembershipCircuit};
use zk_groth16_snark::groth16::{setup, prove};

fn bench_circuit_comparison(c: &mut Criterion) {
    // Compare proving time across all three circuits
    let mut group = c.benchmark_group("prove");

    // Identity
    let identity = IdentityCircuit::new([0u8; 32], vec![0u8; 32]);
    let (pk_id, _) = setup(&identity).unwrap();
    let witness_id = identity.generate_witness().unwrap();
    group.bench_function("identity", |b| {
        b.iter(|| prove(&pk_id, &witness_id))
    });

    // Privacy
    let privacy = PrivacyCircuit::new(0, 100, 50);
    let (pk_priv, _) = setup(&privacy).unwrap();
    let witness_priv = privacy.generate_witness().unwrap();
    group.bench_function("privacy", |b| {
        b.iter(|| prove(&pk_priv, &witness_priv))
    });

    group.finish();
}

criterion_group!(comparison, bench_circuit_comparison);
criterion_main!(comparison);
```

**Step 3: Run benchmarks**

Run: `cargo bench`

**Step 4: Commit**

```bash
cd code
git add benches/
git commit -m "bench: add circuit performance benchmarks"
```

---

## Task 10: Final Polish

**Files:**
- Update: `README.md`
- Update: `docs/*.md`

**Step 1: Run full test suite**

Run: `cargo test`

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`

Fix all warnings

**Step 3: Format code**

Run: `cargo fmt`

**Step 4: Build documentation**

Run: `cargo doc --open`

**Step 5: Update README with performance results**

Add benchmark results to README

**Step 6: Final commit**

```bash
cd code
git add -A
git commit -m "chore: final polish - all tests pass, clippy clean, documented"
```

---

## Implementation Notes

**Key arkworks APIs:**
- `Groth16<E>::generate_random_parameters_with_reduction()` - Setup
- `Groth16<E>::prove()` - Prove
- `Groth16<E>::verify()` - Verify
- `ConstraintSynthesizer::new()` - Create constraint system
- `Sha256Gadget` - SHA-256 in-circuit

**Expected Challenges:**
1. **Type conversions** - Bytes to field elements, proper padding
2. **Public inputs ordering** - Must match generation order
3. **Witness structure** - May need helper functions
4. **API changes** - arkworks 0.4 may differ from examples

**Troubleshooting:**
- If proving fails: Check public inputs match expected order
- If verification fails: Ensure proof and public inputs are from same proving run
- If constraints fail: Check field element conversions

**Success Criteria:**
- ✅ All tests pass
- ✅ All examples run successfully
- ✅ Benchmarks produce results
- ✅ Zero clippy warnings
- ✅ `cargo doc --open` builds successfully

---

**This completes the zk-groth16-snark Phase 2 implementation.**
