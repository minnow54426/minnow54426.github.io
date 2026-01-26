//! Membership circuit for Merkle tree proofs
//!
//! This module implements a circuit that proves membership in a Merkle tree
//! without revealing which leaf is being proven.
//!
//! # Example
//!
//! ```ignore
//! let root = [1u8; 32]; // Merkle root
//! let circuit = MembershipCircuit::new(root);
//! // Later: prove knowledge of leaf and path that hashes to root
//! ```

use ark_ff::Field;
use ark_relations::r1cs::ConstraintSystemRef;
use serde::{Deserialize, Serialize};
use crate::circuit::Groth16Circuit;
use crate::error::{CircuitError, MembershipError, Result};
use ark_bn254::Fr;

/// Membership circuit for Merkle tree inclusion proofs
///
/// This circuit proves a leaf is in a Merkle tree with given root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipCircuit {
    /// The Merkle tree root
    pub root: [u8; 32],
}

impl MembershipCircuit {
    /// Create a new Membership circuit with the given root
    ///
    /// # Arguments
    ///
    /// * `root` - The 32-byte Merkle root (public input)
    ///
    /// # Example
    ///
    /// ```
    /// # use zk_groth16_snark::membership::MembershipCircuit;
    /// let root = [1u8; 32];
    /// let circuit = MembershipCircuit::new(root);
    /// assert_eq!(circuit.root, root);
    /// ```
    pub fn new(root: [u8; 32]) -> Self {
        Self { root }
    }
}

impl Groth16Circuit<Fr> for MembershipCircuit {
    fn circuit_name() -> &'static str {
        "membership"
    }

    /// Public inputs: the Merkle root
    type PublicInputs = [u8; 32];

    /// Private witness: leaf and Merkle path
    type Witness = (Vec<u8>, Vec<[u8; 32]>);

    fn generate_constraints(
        _cs: ConstraintSystemRef<Fr>,
        _witness: &Self::Witness,
    ) -> Result<()> {
        // Stub implementation - TODO: Implement actual Merkle path constraints
        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        // Stub implementation - TODO: Generate actual leaf and path witness
        Err(CircuitError::Membership(MembershipError::InvalidPathLength).into())
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
        // Stub implementation - TODO: Extract actual public inputs
        [0u8; 32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let root = [42u8; 32];
        let circuit = MembershipCircuit::new(root);
        assert_eq!(circuit.root, root);
    }

    #[test]
    fn test_circuit_name() {
        assert_eq!(MembershipCircuit::circuit_name(), "membership");
    }

    #[test]
    fn test_empty_root() {
        let root = [0u8; 32];
        let circuit = MembershipCircuit::new(root);
        assert_eq!(circuit.root, root);
    }
}
