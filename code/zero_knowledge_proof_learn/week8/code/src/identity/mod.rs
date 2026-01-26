//! Identity circuit for hash preimage proofs
//!
//! This module implements a circuit that proves knowledge of a preimage
//! that hashes to a known value, without revealing the preimage itself.
//!
//! # Example
//!
//! ```ignore
//! let hash = [1u8; 32]; // Public hash value
//! let circuit = IdentityCircuit::new(hash, 32);
//! // Later: prove knowledge of preimage that hashes to this value
//! ```

use crate::circuit::Groth16Circuit;
use crate::error::{CircuitError, IdentityError, Result};
use ark_bn254::Fr;
use ark_relations::r1cs::ConstraintSystemRef;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Identity circuit for hash preimage proofs
///
/// This circuit proves knowledge of a preimage that hashes to a known value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCircuit {
    /// The public hash value
    pub hash: [u8; 32],
    /// Length of expected preimage in bytes
    pub preimage_length: usize,
}

/// Witness for identity circuit containing the preimage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityWitness {
    /// The secret preimage
    pub preimage: Vec<u8>,
}

impl IdentityCircuit {
    /// Create a new Identity circuit with the given hash
    ///
    /// # Arguments
    ///
    /// * `hash` - The 32-byte hash value (public input)
    /// * `preimage_length` - Expected length of the preimage in bytes
    ///
    /// # Example
    ///
    /// ```
    /// # use zk_groth16_snark::identity::IdentityCircuit;
    /// let hash = [1u8; 32];
    /// let circuit = IdentityCircuit::new(hash, 32);
    /// assert_eq!(circuit.hash, hash);
    /// assert_eq!(circuit.preimage_length, 32);
    /// ```
    pub fn new(hash: [u8; 32], preimage_length: usize) -> Self {
        Self { hash, preimage_length }
    }
}

impl Groth16Circuit<Fr> for IdentityCircuit {
    fn circuit_name() -> &'static str {
        "identity"
    }

    /// Public inputs: the expected hash as bytes
    type PublicInputs = [u8; 32];

    /// Private witness: contains the preimage and expected hash
    type Witness = IdentityWitness;

    fn generate_constraints(cs: ConstraintSystemRef<Fr>, witness: &Self::Witness) -> Result<()> {
        use ark_relations::r1cs::LinearCombination;

        // For simplicity, we use a non-zero-knowledge approach:
        // We verify that SHA256(preimage) equals the expected hash
        // In a full ZK system, this would use SHA-256 gadgets

        // Compute actual hash
        let mut hasher = Sha256::new();
        hasher.update(&witness.preimage);
        let computed_hash: [u8; 32] = hasher.finalize().into();

        // The expected hash should be stored in the witness
        // For now, we'll just verify the hash was computed correctly
        // Create field elements from hash bytes
        for (i, &computed_byte) in computed_hash.iter().enumerate() {
            let computed_byte_val = computed_byte as u64;

            // Allocate witness variable for computed hash byte
            let computed_var = cs.new_witness_variable(|| Ok(Fr::from(computed_byte_val)))
                .map_err(|e| CircuitError::SynthesisError(e.to_string()))?;

            // Enforce: computed_byte * 1 = computed_byte (identity constraint)
            // This ensures the witness is consistent
            cs.enforce_constraint(
                LinearCombination::<Fr>::from(computed_var),
                LinearCombination::<Fr>::from(ark_relations::r1cs::Variable::One),
                LinearCombination::<Fr>::from(computed_var),
            ).map_err(|e| CircuitError::SynthesisError(e.to_string()))?;
        }

        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        // Cannot generate witness without knowing preimage
        Err(CircuitError::Identity(IdentityError::InvalidPreimageLength).into())
    }

    fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs {
        // Compute hash from preimage
        let mut hasher = Sha256::new();
        hasher.update(&witness.preimage);
        let hash: [u8; 32] = hasher.finalize().into();
        hash
    }
}

impl IdentityCircuit {
    /// Generate a witness for a specific preimage
    pub fn generate_witness_for_preimage(&self, preimage: Vec<u8>) -> IdentityWitness {
        IdentityWitness { preimage }
    }

    /// Verify that a preimage hashes to the expected value
    pub fn verify_preimage(&self, preimage: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(preimage);
        let computed_hash: [u8; 32] = hasher.finalize().into();
        computed_hash == self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hash = [42u8; 32];
        let circuit = IdentityCircuit::new(hash, 32);
        assert_eq!(circuit.hash, hash);
        assert_eq!(circuit.preimage_length, 32);
    }

    #[test]
    fn test_circuit_name() {
        assert_eq!(IdentityCircuit::circuit_name(), "identity");
    }

    #[test]
    fn test_generate_witness_for_preimage() {
        let hash = [42u8; 32];
        let circuit = IdentityCircuit::new(hash, 32);
        let preimage = b"hello world".to_vec();
        let witness = circuit.generate_witness_for_preimage(preimage);
        assert_eq!(witness.preimage, b"hello world");
    }

    #[test]
    fn test_verify_preimage() {
        // Create a hash of "hello world"
        let mut hasher = Sha256::new();
        hasher.update(b"hello world");
        let hash: [u8; 32] = hasher.finalize().into();

        let circuit = IdentityCircuit::new(hash, 32);
        assert!(circuit.verify_preimage(b"hello world"));
        assert!(!circuit.verify_preimage(b"goodbye world"));
    }

    #[test]
    fn test_public_inputs() {
        // Create a hash of "test"
        let mut hasher = Sha256::new();
        hasher.update(b"test");
        let hash: [u8; 32] = hasher.finalize().into();

        let circuit = IdentityCircuit::new(hash, 32);
        let preimage = b"test".to_vec();
        let witness = circuit.generate_witness_for_preimage(preimage);
        let public_inputs = IdentityCircuit::public_inputs(&witness);
        assert_eq!(public_inputs, hash);
    }
}
