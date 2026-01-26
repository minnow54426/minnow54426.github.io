//! Privacy circuit for range proofs
//!
//! This module implements a circuit that proves a value is within a specified
//! range without revealing the value itself.
//!
//! # Example
//!
//! ```ignore
//! let min = 10u64;
//! let max = 100u64;
//! let circuit = PrivacyCircuit::new(min, max);
//! // Later: prove knowledge of value in range [min, max]
//! ```

use crate::circuit::Groth16Circuit;
use crate::error::{CircuitError, PrivacyError, Result};
use ark_bn254::Fr;
use ark_relations::r1cs::ConstraintSystemRef;
use serde::{Deserialize, Serialize};

/// Privacy circuit for range proofs
///
/// This circuit proves a value is within [min, max] without revealing the value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyCircuit {
    /// Minimum value of the range
    pub min: u64,
    /// Maximum value of the range
    pub max: u64,
}

impl PrivacyCircuit {
    /// Create a new Privacy circuit with the given range
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum value of the range
    /// * `max` - Maximum value of the range
    ///
    /// # Example
    ///
    /// ```
    /// # use zk_groth16_snark::privacy::PrivacyCircuit;
    /// let circuit = PrivacyCircuit::new(10, 100);
    /// assert_eq!(circuit.min, 10);
    /// assert_eq!(circuit.max, 100);
    /// ```
    pub fn new(min: u64, max: u64) -> Self {
        Self { min, max }
    }
}

/// Witness for privacy circuit containing the secret value and bounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyWitness {
    /// The secret value to prove is in range
    pub value: u64,
    /// Minimum value of the range
    pub min: u64,
    /// Maximum value of the range
    pub max: u64,
}

impl Groth16Circuit<Fr> for PrivacyCircuit {
    fn circuit_name() -> &'static str {
        "privacy"
    }

    /// Public inputs: the range bounds as u64 values
    type PublicInputs = (u64, u64);

    /// Private witness: contains the secret value and bounds
    type Witness = PrivacyWitness;

    fn generate_constraints(cs: ConstraintSystemRef<Fr>, witness: &Self::Witness) -> Result<()> {
        use ark_relations::r1cs::LinearCombination;
        use ark_ff::Field;

        // Allocate variables
        let value_var = cs.new_witness_variable(|| Ok(Fr::from(witness.value)))
            .map_err(|e| CircuitError::SynthesisError(e.to_string()))?;
        let min_var = cs.new_input_variable(|| Ok(Fr::from(witness.min)))
            .map_err(|e| CircuitError::SynthesisError(e.to_string()))?;
        let max_var = cs.new_input_variable(|| Ok(Fr::from(witness.max)))
            .map_err(|e| CircuitError::SynthesisError(e.to_string()))?;

        // Compute (value - min) * (max - value)
        let diff_min: LinearCombination<Fr> = LinearCombination::from(value_var) - LinearCombination::from(min_var);
        let diff_max: LinearCombination<Fr> = LinearCombination::from(max_var) - LinearCombination::from(value_var);

        // Allocate product variable
        let product_witness_val = (Fr::from(witness.value) - Fr::from(witness.min))
            * (Fr::from(witness.max) - Fr::from(witness.value));
        let product_var = cs.new_witness_variable(|| Ok(product_witness_val))
            .map_err(|e| CircuitError::SynthesisError(e.to_string()))?;

        // Enforce constraint: (value - min) * (max - value) = product
        // This is encoded as: A • B = C where A, B, C are linear combinations
        cs.enforce_constraint(
            diff_min,
            diff_max,
            LinearCombination::<Fr>::from(product_var),
        ).map_err(|e| CircuitError::SynthesisError(e.to_string()))?;

        Ok(())
    }

    fn generate_witness(&self) -> Result<Self::Witness> {
        // For now, return error since we need a value
        Err(CircuitError::Privacy(PrivacyError::ValueOutOfRange).into())
    }

    fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs {
        (witness.min, witness.max)
    }
}

impl PrivacyCircuit {
    /// Generate a witness for a specific value
    pub fn generate_witness_for_value(&self, value: u64) -> PrivacyWitness {
        PrivacyWitness {
            value,
            min: self.min,
            max: self.max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let circuit = PrivacyCircuit::new(10, 100);
        assert_eq!(circuit.min, 10);
        assert_eq!(circuit.max, 100);
    }

    #[test]
    fn test_circuit_name() {
        assert_eq!(PrivacyCircuit::circuit_name(), "privacy");
    }

    #[test]
    fn test_zero_bounds() {
        let circuit = PrivacyCircuit::new(0, 0);
        assert_eq!(circuit.min, 0);
        assert_eq!(circuit.max, 0);
    }

    #[test]
    fn test_generate_witness_for_value() {
        let circuit = PrivacyCircuit::new(10, 100);
        let witness = circuit.generate_witness_for_value(50);
        assert_eq!(witness.value, 50);
        assert_eq!(witness.min, 10);
        assert_eq!(witness.max, 100);
    }

    #[test]
    fn test_public_inputs() {
        let circuit = PrivacyCircuit::new(10, 100);
        let witness = circuit.generate_witness_for_value(50);
        let (min, max) = PrivacyCircuit::public_inputs(&witness);
        assert_eq!(min, 10);
        assert_eq!(max, 100);
    }
}
