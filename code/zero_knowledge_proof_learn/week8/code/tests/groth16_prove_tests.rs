use zk_groth16_snark::circuit::Groth16Circuit;
use zk_groth16_snark::groth16::{setup, prove, verify};
use zk_groth16_snark::Result;
use ark_bn254::Fr;
use ark_relations::r1cs::ConstraintSystemRef;

struct SimpleCircuit {
    value: u64,
}

impl Groth16Circuit<Fr> for SimpleCircuit {
    fn circuit_name() -> &'static str {
        "simple"
    }

    type PublicInputs = u64;
    type Witness = u64;

    fn generate_constraints(
        _cs: ConstraintSystemRef<Fr>,
        _witness: &Self::Witness,
    ) -> Result<()> {
        // Trivial constraint - empty circuit for testing
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

    let proof = prove::<SimpleCircuit>(&pk, &witness).unwrap();

    // Groth16 compressed proofs on BN254 are 128 bytes
    // (48 + 48 + 32 for the three group elements)
    assert_eq!(proof.len(), 128);
}

#[test]
fn test_prove_and_verify() {
    let circuit = SimpleCircuit { value: 42 };
    let (pk, vk) = setup(&circuit).unwrap();
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = SimpleCircuit::public_inputs(&witness);

    let proof = prove::<SimpleCircuit>(&pk, &witness).unwrap();
    let is_valid = verify::<SimpleCircuit>(&vk, &public_inputs, &proof).unwrap();

    assert!(is_valid);
}
