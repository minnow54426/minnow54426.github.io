use ark_bls12_381::Fr;
use zk_groth16_snark::circuit::Groth16Circuit;

#[test]
fn test_circuit_name() {
    struct DummyCircuit;
    impl Groth16Circuit<Fr> for DummyCircuit {
        fn circuit_name() -> &'static str {
            "dummy"
        }

        // Use String instead of Vec<Fr> since Fr doesn't implement Serialize/Deserialize by default
        type PublicInputs = String;
        type Witness = String;

        fn generate_constraints(
            _cs: ark_relations::r1cs::ConstraintSystemRef<Fr>,
            _witness: &Self::Witness,
        ) -> zk_groth16_snark::Result<()> {
            Ok(())
        }

        fn generate_witness(&self) -> zk_groth16_snark::Result<Self::Witness> {
            Ok("dummy witness".to_string())
        }

        fn public_inputs(_witness: &Self::Witness) -> Self::PublicInputs {
            "dummy inputs".to_string()
        }
    }

    assert_eq!(DummyCircuit::circuit_name(), "dummy");
}

#[test]
fn test_circuit_methods() {
    struct TestCircuit;
    impl Groth16Circuit<Fr> for TestCircuit {
        fn circuit_name() -> &'static str {
            "test_circuit"
        }

        type PublicInputs = Vec<u8>;
        type Witness = Vec<u8>;

        fn generate_constraints(
            _cs: ark_relations::r1cs::ConstraintSystemRef<Fr>,
            _witness: &Self::Witness,
        ) -> zk_groth16_snark::Result<()> {
            Ok(())
        }

        fn generate_witness(&self) -> zk_groth16_snark::Result<Self::Witness> {
            Ok(vec![1, 2, 3, 4])
        }

        fn public_inputs(witness: &Self::Witness) -> Self::PublicInputs {
            witness.clone()
        }
    }

    let circuit = TestCircuit;
    let witness = circuit.generate_witness().unwrap();
    let public_inputs = TestCircuit::public_inputs(&witness);

    assert_eq!(witness, vec![1, 2, 3, 4]);
    assert_eq!(public_inputs, vec![1, 2, 3, 4]);
}
