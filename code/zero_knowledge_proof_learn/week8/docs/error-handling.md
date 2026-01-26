# Error Handling Guide

This guide provides a comprehensive overview of all error types in the `zk-groth16-snark` library, including their meanings, common causes, and how to fix them.

## Error Hierarchy

```
Error
├── Circuit(CircuitError)
│   ├── Identity(IdentityError)
│   ├── Membership(MembershipError)
│   └── Privacy(PrivacyError)
├── Setup(SetupError)
├── Prove(ProveError)
├── Verify(VerifyError)
├── Serialization(SerializationError)
└── Io(io::Error)
```

## Setup Errors

### `SetupError::ParametersAlreadyExist`

**Meaning**: Setup parameters (proving key and verifying key) already exist for this circuit.

**Common Cause**: Calling `setup()` twice without clearing the `params/` directory, or attempting to run setup when parameters have already been generated.

**Fix**:
```rust
// Option 1: Delete existing parameters
fs::remove_dir_all("params/identity")?;

// Option 2: Load existing parameters instead
let (pk, vk) = load_params("identity")?;

// Option 3: Check for existence first
if params_exist("identity") {
    let (pk, vk) = load_params("identity")?;
} else {
    let (pk, vk) = setup(&circuit)?;
}
```

**Example**:
```rust
use zk_groth16_snark::{setup, load_params, SetupError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circuit = IdentityCircuit::new(hash);

    match setup(&circuit) {
        Ok((pk, vk)) => {
            println!("Setup complete!");
        }
        Err(e) if e.kind() == ErrorKind::ParametersAlreadyExist => {
            println!("Loading existing parameters...");
            let (pk, vk) = load_params("identity")?;
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(())
}
```

### `SetupError::InsufficientEntropy`

**Meaning**: The random number generator failed to produce sufficient entropy for parameter generation.

**Common Cause**: Using a deterministic RNG or running on a system with limited entropy sources.

**Fix**:
```rust
// Use a cryptographically secure RNG
use rand::rngs::OsRng;

let mut rng = OsRng;
let (pk, vk) = Groth16::generate_random_parameters_with_reduction(&circuit, &mut rng)?;
```

### `SetupError::SetupFailed`

**Meaning**: Parameter generation failed during the Groth16 setup process.

**Common Cause**: Circuit constraints are malformed or the field type is incompatible.

**Fix**:
1. Verify circuit constraints are correctly defined
2. Check that field types match (e.g., using `Bls12_381` consistently)
3. Ensure all public inputs and witnesses are correctly sized

**Example**:
```rust
// Check constraint system before setup
let cs = ConstraintMetavar::new();
circuit.generate_constraints(cs.clone())?;
if cs.num_constraints() == 0 {
    return Err(SetupError::SetupFailed.into());
}
```

---

## Prove Errors

### `ProveError::WitnessGenerationFailed`

**Meaning**: Failed to generate the witness (private inputs) from the provided inputs.

**Common Cause**: Invalid input data types, incorrect input sizes, or malformed witness data.

**Fix**:
```rust
// Validate inputs before generating witness
fn validate_preimage(preimage: &str) -> Result<(), Error> {
    if preimage.len() != 32 {
        return Err(Error::Prove(ProveError::WitnessGenerationFailed));
    }
    Ok(())
}

// Use in your code
validate_preimage(preimage)?;
let witness = circuit.generate_witness(preimage)?;
```

**Debugging Tips**:
- Check that all private inputs match expected sizes
- Verify string/byte conversions are correct
- Ensure witness struct is properly initialized

### `ProveError::ProofCreationFailed`

**Meaning**: The Groth16 proof generation algorithm failed.

**Common Cause**: Malformed proving key, invalid witness, or arithmetic errors in constraints.

**Fix**:
1. Verify the proving key was loaded correctly
2. Check that witness satisfies all constraints
3. Ensure field arithmetic is consistent

**Example**:
```rust
match prove(&pk, &witness) {
    Ok(proof) => proof,
    Err(ProveError::ProofCreationFailed) => {
        eprintln!("Proof creation failed. Check that:");
        eprintln!("  - Proving key is valid");
        eprintln!("  - Witness satisfies constraints");
        eprintln!("  - Field arithmetic is consistent");
        return Err(e.into());
    }
    Err(e) => return Err(e.into()),
}
```

---

## Verify Errors

### `VerifyError::InvalidProof`

**Meaning**: The proof verification failed because the proof is cryptographically invalid.

**Common Cause**: The proof was tampered with, corrupted, or generated for different public inputs.

**Fix**:
- Ensure the proof hasn't been modified
- Verify public inputs match what the proof was created for
- Check that the proof and verifying key are from the same setup

**Example**:
```rust
match verify(&vk, &public_inputs, &proof) {
    Ok(false) | Err(VerifyError::InvalidProof) => {
        eprintln!("Proof verification failed. Possible causes:");
        eprintln!("  - Proof was tampered with");
        eprintln!("  - Public inputs don't match proof");
        eprintln!("  - Wrong verifying key for this proof");
        return Err(e.into());
    }
    Ok(true) => println!("Proof verified!"),
    Err(e) => return Err(e.into()),
}
```

### `VerifyError::ProofVerificationFailed`

**Meaning**: The verifier rejected the proof as invalid.

**Common Cause**: Same as `InvalidProof`, but this error is more general and can include verifier-side issues.

**Fix**:
1. Verify the verifying key is correct
2. Check public inputs encoding
3. Ensure the proof format is compatible

### `VerifyError::PublicInputsIncorrect`

**Meaning**: Public inputs don't match expected format or values.

**Common Cause**: Wrong number of public inputs, incorrect encoding, or field element conversion errors.

**Fix**:
```rust
// Validate public inputs before verification
fn validate_public_inputs(inputs: &[F]) -> Result<(), Error> {
    if inputs.len() != expected_input_count() {
        return Err(Error::Verify(VerifyError::PublicInputsIncorrect));
    }
    Ok(())
}

validate_public_inputs(&public_inputs)?;
let is_valid = verify(&vk, &public_inputs, &proof)?;
```

---

## Circuit Errors

### Identity Circuit Errors

#### `IdentityError::InvalidPreimageLength`

**Meaning**: Preimage (secret value) has incorrect length.

**Common Cause**: Preimage is not exactly 32 bytes (256 bits).

**Fix**:
```rust
// Pad or truncate to exactly 32 bytes
let preimage = if input.len() < 32 {
    let mut padded = vec![0u8; 32];
    padded[..input.len()].copy_from_slice(input.as_bytes());
    padded
} else {
    input.as_bytes()[..32].to_vec()
};
```

#### `IdentityError::HashMismatch`

**Meaning**: The hash of the preimage doesn't match the public hash output.

**Common Cause**: Wrong preimage, incorrect hash computation, or bit/byte ordering issues.

**Fix**:
- Verify preimage is correct
- Check hash function is SHA-256
- Ensure consistent byte order (big-endian vs little-endian)

### Membership Circuit Errors

#### `MembershipError::InvalidPathLength`

**Meaning**: Authentication path has incorrect length for the tree depth.

**Common Cause**: Path doesn't match the configured tree depth (e.g., depth 8 expects 8 siblings).

**Fix**:
```rust
// Validate path length
if path.len() != tree_depth {
    return Err(Error::Circuit(
        CircuitError::Membership(MembershipError::InvalidPathLength)
    ));
}
```

#### `MembershipError::RootMismatch`

**Meaning**: Computed root from path doesn't match expected root.

**Common Cause**: Wrong path, incorrect leaf, or tampered path data.

**Fix**:
- Verify path is correct (matches tree structure)
- Check leaf value is correct
- Ensure path siblings are in correct order

#### `MembershipError::LeafNotFound`

**Meaning**: Leaf doesn't exist in the tree.

**Common Cause**: Attempting to prove membership for a non-existent leaf.

**Fix**:
```rust
// Check leaf exists before generating proof
if !tree.contains_leaf(&leaf) {
    return Err(Error::Circuit(
        CircuitError::Membership(MembershipError::LeafNotFound)
    ));
}
```

### Privacy Circuit Errors

#### `PrivacyError::ValueOutOfRange`

**Meaning**: Secret value is outside the specified range.

**Common Cause**: Value is less than min or greater than max.

**Fix**:
```rust
// Validate value is in range before proving
if value < min || value > max {
    return Err(Error::Circuit(
        CircuitError::Privacy(PrivacyError::ValueOutOfRange)
    ));
}
```

#### `PrivacyError::InvalidBitWidth`

**Meaning**: Value requires more bits than the circuit supports.

**Common Cause**: Using a 32-bit circuit for a 64-bit value, for example.

**Fix**:
```rust
// Use appropriate bit width
let bit_width = 64; // for values up to 2^64 - 1
let circuit = PrivacyCircuit::with_bit_width(min, max, bit_width)?;
```

---

## Serialization Errors

### `SerializationError::DeserializationFailed`

**Meaning**: Failed to deserialize parameters or proofs from bytes.

**Common Cause**: Corrupted data, version mismatch, or incompatible format.

**Fix**:
```rust
// Use proper error handling
match deserialize_proof(&proof_bytes) {
    Ok(proof) => proof,
    Err(SerializationError::DeserializationFailed) => {
        eprintln!("Failed to deserialize proof. Check that:");
        eprintln!("  - Data is not corrupted");
        eprintln!("  - Version matches library version");
        eprintln!("  - Format is compatible");
        return Err(e.into());
    }
    Err(e) => return Err(e.into()),
}
```

### `SerializationError::VersionMismatch`

**Meaning**: Serialized data format version doesn't match library version.

**Common Cause**: Using data from an older or newer library version.

**Fix**:
- Regenerate parameters with current library version
- Check library version compatibility
- Use version check before deserialization

```rust
const LIBRARY_VERSION: u32 = 1;

fn check_version(data_version: u32) -> Result<(), Error> {
    if data_version != LIBRARY_VERSION {
        return Err(Error::Serialization(
            SerializationError::VersionMismatch
        ));
    }
    Ok(())
}
```

---

## Debugging Scenarios

### "Why does my proof fail to verify?"

**Common causes and solutions**:

1. **Wrong public inputs**:
   ```rust
   // Ensure public inputs match exactly
   assert_eq!(original_public_inputs, verification_public_inputs);
   ```

2. **Proof from different setup**:
   ```rust
   // Verify VK and proof are from same setup
   assert_eq!(proof_setup_id, vk_setup_id);
   ```

3. **Corrupted proof data**:
   ```rust
   // Check proof size
   if proof_bytes.len() != EXPECTED_PROOF_SIZE {
       return Err("Proof size incorrect".into());
   }
   ```

4. **Field element encoding issues**:
   ```rust
   // Ensure consistent encoding
   let public_inputs = serialize_to_field_elements(&inputs)?;
   ```

### "Why does setup take so long?"

**Explanation**: Groth16 setup is computationally expensive but one-time cost.

**Solutions**:
- Reuse existing parameters (load from disk)
- Use smaller circuits if possible
- Consider faster proving systems (e.g., Sonic, Marlin) if setup time is critical

### "Why is my proof so large?"

**Expected sizes**:
- Groth16 proof: ~288 bytes (fixed)
- Proving key: ~10-15 KB (depends on circuit)
- Verifying key: ~1-2 KB (depends on circuit)

**If larger than expected**:
- Check for unnecessary data in proof
- Verify compression is enabled
- Compare with benchmarks

---

## Best Practices

### 1. Always Handle Errors Explicitly

```rust
// Good
match prove(&pk, &witness) {
    Ok(proof) => { /* use proof */ }
    Err(e) => {
        eprintln!("Prove failed: {}", e);
        return Err(e.into());
    }
}

// Avoid
let proof = prove(&pk, &witness).unwrap(); // Can panic!
```

### 2. Validate Inputs Early

```rust
// Validate before expensive operations
validate_inputs(&public_inputs, &witness)?;
let proof = prove(&pk, &witness)?;
```

### 3. Provide Context with Errors

```rust
// Add context for better debugging
let proof = prove(&pk, &witness)
    .map_err(|e| Error::Prove(ProveError::ProofCreationFailed))
    .context("Failed to generate proof for identity circuit")?;
```

### 4. Log Errors for Debugging

```rust
// Enable debug logging
env_logger::init();

// Log errors with details
error!("Proof verification failed: {:?}", e);
error!("Public inputs: {:?}", public_inputs);
error!("Proof size: {} bytes", proof_bytes.len());
```

### 5. Test Error Cases

```rust
#[test]
fn test_invalid_witness_fails() {
    let result = prove_with_invalid_witness();
    assert!(matches!(result, Err(ProveError::WitnessGenerationFailed)));
}
```

---

## Quick Reference

| Error | Most Common Cause | Quick Fix |
|-------|-------------------|-----------|
| `ParametersAlreadyExist` | Running setup twice | Load existing params or delete old ones |
| `WitnessGenerationFailed` | Invalid input data | Validate input sizes and types |
| `ProofCreationFailed` | Malformed circuit/keys | Verify circuit constraints |
| `InvalidProof` | Tampered or wrong proof | Regenerate proof with correct inputs |
| `PublicInputsIncorrect` | Wrong number/format of inputs | Validate inputs before verify |
| `InvalidPreimageLength` | Preimage not 32 bytes | Pad or truncate to 32 bytes |
| `InvalidPathLength` | Path length ≠ tree depth | Check tree depth configuration |
| `ValueOutOfRange` | Value outside [min, max] | Validate value before proving |
| `DeserializationFailed` | Corrupted data | Check data integrity and version |

---

## Getting Help

If you encounter errors not covered here:

1. **Check the examples**: All example code demonstrates proper error handling
2. **Run tests**: `cargo test` shows expected error handling patterns
3. **Enable debug logging**: `RUST_LOG=debug cargo run`
4. **Open an issue**: Include error message, code snippet, and backtrace

---

## References

- [Rust Error Handling Best Practices](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror Crate Documentation](https://docs.rs/thiserror/)
- [arkworks Error Handling Patterns](https://github.com/arkworks-rs)
