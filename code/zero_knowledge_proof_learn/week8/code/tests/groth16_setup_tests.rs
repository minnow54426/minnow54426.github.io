//! Tests for Groth16 setup functionality

use ark_bn254::Fr;
use zk_groth16_snark::circuit::Groth16Circuit;
use zk_groth16_snark::groth16::setup;

/// Simple test circuit for setup testing
struct TestCircuit;

impl Groth16Circuit<Fr> for TestCircuit {
    fn circuit_name() -> &'static str {
        "test_circuit"
    }

    type PublicInputs = ();
    type Witness = ();

    fn generate_constraints(
        _cs: ark_relations::r1cs::ConstraintSystemRef<Fr>,
        _witness: &Self::Witness,
    ) -> zk_groth16_snark::Result<()> {
        Ok(())
    }

    fn generate_witness(&self) -> zk_groth16_snark::Result<Self::Witness> {
        Ok(())
    }

    fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {}
}

#[test]
fn test_setup() {
    let circuit = TestCircuit;
    let (pk, vk) = setup(&circuit).unwrap();

    // Verify keys are generated
    assert!(pk.vk.gamma_abc_g1.len() > 0); // Has gamma G1 elements
    // Check that alpha_g1 is not the identity (zero) point
    assert!(vk.alpha_g1 != ark_ec::short_weierstrass::Affine::identity()); // VK initialized with alpha
}
