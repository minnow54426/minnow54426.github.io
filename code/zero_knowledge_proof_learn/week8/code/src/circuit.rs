use ark_ff::Field;
use ark_relations::r1cs::ConstraintSystemRef;
use serde::{Deserialize, Serialize};
use crate::Result;

/// Core trait that all Groth16 circuits must implement
pub trait Groth16Circuit<F: Field> {
    /// Circuit identifier for debugging/serialization
    fn circuit_name() -> &'static str;

    /// Public inputs for verification
    type PublicInputs: Clone + Serialize + for<'de> Deserialize<'de>;

    /// Private witness (known only to prover)
    type Witness: Clone + Serialize + for<'de> Deserialize<'de>;

    /// Generate constraint system
    fn generate_constraints(
        cs: ConstraintSystemRef<F>,
        witness: &Self::Witness,
    ) -> Result<()>;

    /// Create witness from private inputs
    fn generate_witness(&self) -> Result<Self::Witness>;

    /// Extract public inputs from witness
    fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs;
}
