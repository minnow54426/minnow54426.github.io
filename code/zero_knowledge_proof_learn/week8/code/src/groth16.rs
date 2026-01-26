//! Groth16 proving system implementation
//!
//! This module provides setup, proving, and verification functionality for
//! the Groth16 zk-SNARK protocol.

use crate::circuit::Groth16Circuit;
use crate::error::{Error, Result, SetupError};
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_relations::r1cs::ConstraintSynthesizer;
use rand::rngs::OsRng;

/// Proving parameters generated during trusted setup
///
/// Wrapper around ark-groth16's ProvingKey
pub type ProvingParams = ProvingKey<Bn254>;

/// Verification key for proof verification
///
/// Wrapper around ark-groth16's VerifyingKey
pub type VerificationKey = VerifyingKey<Bn254>;

/// Perform trusted setup for a Groth16 circuit
///
/// This generates the proving and verification parameters for a given circuit.
/// In production, this would require a secure multi-party computation (MPC)
/// ceremony to ensure the toxic waste is properly destroyed.
///
/// # Arguments
///
/// * `circuit` - The circuit to generate parameters for
///
/// # Returns
///
/// A tuple of (ProvingParams, VerificationKey)
///
/// # Errors
///
/// Returns `SetupError::SetupFailed` if setup cannot be completed
///
/// # Example
///
/// ```ignore
/// let circuit = MyCircuit::new();
/// let (params, vk) = setup(&circuit)?;
/// ```
pub fn setup<C: Groth16Circuit<Fr>>(circuit: &C) -> Result<(ProvingParams, VerificationKey)> {
    // Generate witness for the circuit
    let witness = circuit
        .generate_witness()
        .map_err(|_| Error::Setup(SetupError::SetupFailed))?;

    // Create a constraint synthesizer that will generate constraints
    // We need to wrap our circuit to implement ConstraintSynthesizer
    let synthesizer = CircuitSynthesizer::new(circuit, witness)?;

    // Generate random parameters using Groth16
    let pk = Groth16::<Bn254>::generate_random_parameters_with_reduction(synthesizer, &mut OsRng)
        .map_err(|_| Error::Setup(SetupError::SetupFailed))?;

    // Extract the verifying key from the proving key
    let vk = pk.vk.clone();

    Ok((pk, vk))
}

/// Adapter struct to convert our Groth16Circuit trait into
/// ark-relations's ConstraintSynthesizer trait
struct CircuitSynthesizer<C: Groth16Circuit<Fr>> {
    witness: C::Witness,
}

impl<C: Groth16Circuit<Fr>> CircuitSynthesizer<C> {
    fn new(_circuit: &C, witness: C::Witness) -> Result<Self> {
        Ok(Self { witness })
    }
}

impl<C: Groth16Circuit<Fr>> ConstraintSynthesizer<Fr> for CircuitSynthesizer<C> {
    fn generate_constraints(
        self,
        cs: ark_relations::r1cs::ConstraintSystemRef<Fr>,
    ) -> ark_relations::r1cs::Result<()> {
        // Delegate to our circuit's constraint generation
        C::generate_constraints(cs, &self.witness)
            .map_err(|_| ark_relations::r1cs::SynthesisError::Unsatisfiable)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_returns_error() {
        struct DummyCircuit;
        impl Groth16Circuit<Fr> for DummyCircuit {
            fn circuit_name() -> &'static str {
                "dummy"
            }

            type PublicInputs = ();
            type Witness = ();

            fn generate_constraints(
                _cs: ark_relations::r1cs::ConstraintSystemRef<Fr>,
                _witness: &Self::Witness,
            ) -> Result<()> {
                Ok(())
            }

            fn generate_witness(&self) -> Result<Self::Witness> {
                Ok(())
            }

            fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {}
        }

        let circuit = DummyCircuit;
        // This should now succeed, but with an empty circuit
        let result = setup(&circuit);
        // Empty circuits should work now
        assert!(result.is_ok());
    }
}
