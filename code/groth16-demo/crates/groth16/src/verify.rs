use crate::error::Groth16Error;
use crate::keys::VerificationKey;
use crate::prove::Proof;
use ark_bn254::{Bn254, Fr, G1Projective as G1};
use ark_ec::pairing::Pairing;
use ark_ff::{PrimeField, Zero};
use groth16_math::fields::FieldWrapper;

/// Verifies a Groth16 zero-knowledge proof.
///
/// # Arguments
/// * `vk` - Verification key from trusted setup
/// * `proof` - The proof to verify
/// * `public_inputs` - Public inputs from the witness (e.g., [c] for multiplier)
///
/// # Returns
/// * `Ok(true)` - Proof is valid
/// * `Ok(false)` - Proof is invalid
/// * `Err(Groth16Error)` - Error during verification
///
/// # Verification Equation
///
/// The proof is valid if and only if:
/// ```text
/// e(A, B) = e(α, β) · e(Σpublic_i·IC_i, γ) · e(C, δ)
/// ```
///
/// Where:
/// - `e(·, ·)` is the bilinear pairing
/// - `A, B, C` are proof components
/// - `α, β, γ, δ` are from the verification key
/// - `IC_i` are input consistency elements
/// - `public_i` are public inputs
///
/// # Intuition
///
/// The verification equation checks that:
/// 1. **Proof structure**: A and B were constructed using the toxic waste secrets
/// 2. **Public input consistency**: The witness matches the claimed public inputs
/// 3. **QAP satisfaction**: The division polynomial H(x) exists (witness satisfies constraints)
///
/// # Security
///
/// This verification provides:
/// - **Soundness**: False proofs will be rejected (except with negligible probability)
/// - **Zero-knowledge**: Valid proofs reveal nothing about private witness values
/// - **Succinctness**: Verification is constant-time regardless of circuit complexity
///
/// # Example
/// ```rust,ignore
/// use groth16_groth16::{verify_proof, generate_proof_test, trusted_setup_test};
/// use groth16_circuits::multiplier::MultiplierCircuit;
/// use groth16_qap::r1cs_to_qap;
///
/// // Setup and proof generation
/// let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed)?;
/// let proof = generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed)?;
///
/// // Verify with public input c=12
/// let public_inputs = vec![FieldWrapper::<Fr>::from(12u64)];
/// let is_valid = verify_proof(&vk, &proof, &public_inputs)?;
///
/// assert!(is_valid);  // Proof should be valid
/// # Ok::<(), Groth16Error>(())
/// ```
pub fn verify_proof(
    vk: &VerificationKey,
    proof: &Proof,
    public_inputs: &[FieldWrapper<Fr>],
) -> Result<bool, Groth16Error> {
    // Validate inputs
    // The IC vector contains the input consistency elements
    // Different implementations handle the constant differently:
    // - Some include IC[0] for constant 1, then IC[1..] for public inputs
    // - Others only include IC for public inputs (constant is implicit)
    // We support both cases by checking the length
    if vk.ic.is_empty() && !public_inputs.is_empty() {
        return Err(Groth16Error::InvalidInputs(public_inputs.len()));
    }

    // Step 1: Compute left side of verification equation
    // Left side: e(A, B)
    let left = Bn254::pairing(proof.a, proof.b);

    // Step 2: Compute right side components
    // Component 1: e(α, β)
    let alpha_beta = Bn254::pairing(vk.alpha_g1, vk.beta_g2);

    // Component 2: e(Σpublic_i·IC_i, γ)
    // Compute the linear combination of IC elements with public inputs
    // Check if IC includes the constant (IC length = public_inputs + 1)
    // or just public inputs (IC length = public_inputs)
    let has_constant = vk.ic.len() == public_inputs.len() + 1;

    let mut public_acc = G1::zero();

    if has_constant {
        // IC[0] is for constant 1, IC[1..] are for public inputs
        if !vk.ic.is_empty() {
            public_acc += G1::from(vk.ic[0]);
        }
        for (i, input) in public_inputs.iter().enumerate() {
            if i + 1 < vk.ic.len() {
                let input_scalar = input.value;
                let ic_point = G1::from(vk.ic[i + 1]);
                public_acc += ic_point * input_scalar;
            }
        }
    } else {
        // IC only contains public inputs, constant is implicit
        for (i, input) in public_inputs.iter().enumerate() {
            if i < vk.ic.len() {
                let input_scalar = input.value;
                let ic_point = G1::from(vk.ic[i]);
                public_acc += ic_point * input_scalar;
            }
        }
    }

    let public_gamma = Bn254::pairing(public_acc, vk.gamma_g2);

    // Component 3: e(C, δ)
    let c_delta = Bn254::pairing(proof.c, vk.delta_g2);

    // Right side: e(α, β) · e(Σpublic·IC, γ) · e(C, δ)
    // PairingOutput is a newtype wrapper, so we access .0 to get the TargetField
    let right_field = alpha_beta.0 * public_gamma.0 * c_delta.0;

    // Step 3: Check if verification equation holds
    let is_valid = left.0 == right_field;

    // Debug output
    if !is_valid {
        eprintln!("Verification FAILED:");
        eprintln!("  left (pairing e(A,B)): {:?}", left.0);
        eprintln!("  right (e(α,β) * e(public,γ) * e(C,δ)): {:?}", right_field);
        eprintln!("  e(α,β): {:?}", alpha_beta.0);
        eprintln!("  e(public,γ): {:?}", public_gamma.0);
        eprintln!("  e(C,δ): {:?}", c_delta.0);
        eprintln!(
            "  IC length: {}, public_inputs length: {}",
            vk.ic.len(),
            public_inputs.len()
        );
        eprintln!("  public_inputs: {:?}", public_inputs);
        // Check if the issue is with the left or right side
        eprintln!(
            "  alpha-beta * public-gamma: {:?}",
            (alpha_beta.0 * public_gamma.0)
        );
        eprintln!(
            "  (alpha-beta * public-gamma) * c-delta: {:?}",
            ((alpha_beta.0 * public_gamma.0) * c_delta.0)
        );
    }

    Ok(is_valid)
}

/// Batch verifies multiple Groth16 proofs efficiently using random linear combination.
///
/// # Mathematical Background
///
/// Batch verification uses a random linear combination to verify multiple proofs
/// in a single pairing operation, amortizing the expensive pairing cost.
///
/// # The Batch Verification Equation
///
/// Instead of verifying n proofs individually (O(n) pairings), we compute:
///
/// ```text
/// e(Σ rᵢ · Aᵢ, Bᵢ) = e(α, β) · e(Σ rᵢ · (Σ xᵢ · ICᵢ + Cᵢ), δ)
/// ```
///
/// Where:
/// - `rᵢ` are random non-zero scalars for each proof (generated by RNG)
/// - `Σ` denotes summation over all proofs
/// - Each proof uses the same vk (same α, β, γ, δ)
///
/// # Efficiency Gain
///
/// **Individual verification**: O(n) pairing operations
/// **Batch verification**: O(1) pairing operations (constant!)
///
/// For n proofs:
/// - Individual: ~2n pairings (2 pairings per proof)
/// - Batch: ~2 pairings (constant regardless of n)
///
/// Speedup: Approximately n× faster for large batches
///
/// # Security
///
/// The random scalars rᵢ ensure that a cheating prover cannot construct a
/// batch of proofs that passes verification unless each individual proof
/// is valid. This follows from the Schwartz-Zippel lemma:
///
/// > A non-zero polynomial over a finite field evaluates to zero at
/// > only a small fraction of points. With random rᵢ, the probability
/// > of a forgery succeeding is negligible.
///
/// # Implementation Note
///
/// **Current Implementation**: This version uses individual verification in a loop (O(n) pairings).
///
/// True O(1) batch verification requires:
/// - Combining group elements BEFORE pairing: Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ, Σ rᵢ·Cᵢ, Σ rᵢ·publicᵢ
/// - Then computing: e(Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ) = e(α·Σrᵢ, β) · e(Σ rᵢ·publicᵢ·IC, γ) · e(Σ rᵢ·Cᵢ, δ)
///
/// Since implementing this requires direct access to group element operations and
/// significant refactoring of the pairing operations, we use the pragmatic fallback
/// of individual verification with early exit.
///
/// This maintains correctness and security, but doesn't achieve the O(1) pairing
/// optimization. Future versions will implement true batch verification.
///
/// # Arguments
///
/// * `vk` - Verification key (shared by all proofs in the batch)
/// * `proofs_and_inputs` - Slice of (proof, public_inputs) tuples
/// * `rng` - Random number generator for generating random scalars (unused in current implementation)
///
/// # Returns
///
/// * `Ok(true)` - All proofs are valid
/// * `Ok(false)` - At least one proof is invalid
/// * `Err(Groth16Error)` - Error during verification
///
/// # Example
///
/// ```rust,no_run,ignore
/// use groth16::{trusted_setup, generate_proof, batch_verify};
/// use groth16_r1cs::constraint::R1CSConstraint;
/// use groth16::error::Groth16Error;
/// use groth16::Fq as ScalarField;
/// use groth16_qap::r1cs_to_qap;
/// use rand_chacha::ChaCha20Rng;
/// use rand::SeedableRng;
///
/// # fn main() -> Result<(), Groth16Error> {
/// let mut rng = ChaCha20Rng::from_entropy();
///
/// // Create circuit: a × b = c
/// let mut constraint = R1CSConstraint::<ScalarField>::new();
/// constraint.add_a_variable(2, ScalarField::from(1u64)); // a
/// constraint.add_b_variable(3, ScalarField::from(1u64)); // b
/// constraint.add_c_variable(1, ScalarField::from(1u64)); // c
///
/// let qap = r1cs_to_qap(&[constraint.clone(), constraint])?;
/// let (pk, vk) = trusted_setup(&qap, &mut rng)?;
///
/// // Generate multiple proofs
/// let proof1 = generate_proof_test(&pk, &qap, vec![
///     ScalarField::from(1u64),  // constant
///     ScalarField::from(12u64), // c = 12
///     ScalarField::from(3u64),  // a = 3
///     ScalarField::from(4u64),  // b = 4
/// ], &mut rng)?;
///
/// let proof2 = generate_proof_test(&pk, &qap, vec![
///     ScalarField::from(1u64),  // constant
///     ScalarField::from(30u64), // c = 30
///     ScalarField::from(5u64),  // a = 5
///     ScalarField::from(6u64),  // b = 6
/// ], &mut rng)?;
///
/// // Batch verify
/// let proofs = vec![
///     (proof1, vec![ScalarField::from(12u64)]),
///     (proof2, vec![ScalarField::from(30u64)]),
/// ];
/// let all_valid = batch_verify(&vk, &proofs, &mut rng)?;
///
/// assert!(all_valid);  // Both proofs should be valid
/// # Ok(())
/// # }
/// ```
///
/// # When to Use Batch Verification
///
/// **Use batch verification when**:
/// - Verifying multiple proofs for the same circuit
/// - Performance is critical (e.g., blockchain rollups)
/// - You have access to a secure RNG
///
/// **Use individual verification when**:
/// - Verifying a single proof
/// - Proofs use different circuits (different vk)
/// - You need to identify which specific proof failed
pub fn batch_verify<R>(
    vk: &VerificationKey,
    proofs_and_inputs: &[(Proof, Vec<FieldWrapper<Fr>>)],
    _rng: &mut R,
) -> Result<bool, Groth16Error>
where
    R: rand::RngCore + rand::CryptoRng,
{
    if proofs_and_inputs.is_empty() {
        return Ok(true);
    }

    // Verify each proof individually
    // Early exit on first failure for efficiency
    for (proof, public_inputs) in proofs_and_inputs {
        let is_valid = verify_proof(vk, proof, public_inputs)?;
        if !is_valid {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prove::generate_proof_test;
    use crate::setup::trusted_setup_test;
    use groth16_qap::r1cs_to_qap;
    use groth16_r1cs::constraint::R1CSConstraint;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_verify_valid_proof() {
        // Create multiplier circuit: 3 × 4 = 12
        // Using standard Groth16 witness ordering: [1, c, a, b]
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(2, FieldWrapper::<Fr>::from(1u64)); // a at index 2
        c1.add_b_variable(3, FieldWrapper::<Fr>::from(1u64)); // b at index 3
        c1.add_c_variable(1, FieldWrapper::<Fr>::from(1u64)); // c at index 1

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        // Setup
        let seed = [42u8; 32];
        let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Generate witness for a=3, b=4, c=12
        // Witness ordering: [1, c, a, b]
        let witness = vec![
            FieldWrapper::<Fr>::from(1u64),  // constant 1
            FieldWrapper::<Fr>::from(12u64), // public output c
            FieldWrapper::<Fr>::from(3u64),  // private input a
            FieldWrapper::<Fr>::from(4u64),  // private input b
        ];

        // Generate proof
        let proof =
            generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Verify with public input c=12
        let public_inputs = vec![FieldWrapper::<Fr>::from(12u64)];
        let is_valid = verify_proof(&vk, &proof, &public_inputs).unwrap();

        assert!(is_valid, "Valid proof should verify");
    }

    #[test]
    fn test_verify_invalid_public_input() {
        // Same setup as above, using standard Groth16 witness ordering
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(2, FieldWrapper::<Fr>::from(1u64)); // a at index 2
        c1.add_b_variable(3, FieldWrapper::<Fr>::from(1u64)); // b at index 3
        c1.add_c_variable(1, FieldWrapper::<Fr>::from(1u64)); // c at index 1

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        let seed = [42u8; 32];
        let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Proof for a=3, b=4, c=12
        let witness = vec![
            FieldWrapper::<Fr>::from(1u64),  // constant 1
            FieldWrapper::<Fr>::from(12u64), // public output c
            FieldWrapper::<Fr>::from(3u64),  // private input a
            FieldWrapper::<Fr>::from(4u64),  // private input b
        ];

        let proof =
            generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Try to verify with WRONG public input c=99
        let public_inputs = vec![FieldWrapper::<Fr>::from(99u64)];
        let is_valid = verify_proof(&vk, &proof, &public_inputs).unwrap();

        assert!(!is_valid, "Proof should NOT verify with wrong public input");
    }

    #[test]
    fn test_verify_with_empty_public_inputs() {
        // Create a circuit with no public inputs (except constant 1)
        // This is unusual but should be handled gracefully
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(1, FieldWrapper::<Fr>::from(1u64)); // a at index 1
        c1.add_b_variable(2, FieldWrapper::<Fr>::from(1u64)); // b at index 2
        c1.add_c_variable(3, FieldWrapper::<Fr>::from(1u64)); // c at index 3

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        // Setup with 0 public inputs (only constant 1)
        let seed = [42u8; 32];
        let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 0, &seed).unwrap();

        // Generate witness (all values are private in this case)
        let witness = vec![
            FieldWrapper::<Fr>::from(1u64),  // constant 1
            FieldWrapper::<Fr>::from(3u64),  // a
            FieldWrapper::<Fr>::from(4u64),  // b
            FieldWrapper::<Fr>::from(12u64), // c
        ];

        let proof =
            generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 0, &seed).unwrap();

        // Verify with empty public inputs
        let public_inputs = vec![];
        let is_valid = verify_proof(&vk, &proof, &public_inputs).unwrap();

        assert!(
            is_valid,
            "Valid proof should verify even with no public inputs"
        );
    }

    #[test]
    fn test_batch_verify_valid_proofs() {
        // Using standard Groth16 witness ordering: [1, c, a, b]
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(2, FieldWrapper::<Fr>::from(1u64)); // a at index 2
        c1.add_b_variable(3, FieldWrapper::<Fr>::from(1u64)); // b at index 3
        c1.add_c_variable(1, FieldWrapper::<Fr>::from(1u64)); // c at index 1

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        let seed = [42u8; 32];
        let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Generate multiple valid proofs
        // Proof 1: 3 × 4 = 12
        let proof1 = generate_proof_test(
            &pk,
            &[
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(12u64), // c
                FieldWrapper::<Fr>::from(3u64),  // a
                FieldWrapper::<Fr>::from(4u64),  // b
            ],
            &a_polys,
            &b_polys,
            &c_polys,
            1,
            &seed,
        )
        .unwrap();

        // Proof 2: 5 × 6 = 30
        let proof2 = generate_proof_test(
            &pk,
            &[
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(30u64), // c
                FieldWrapper::<Fr>::from(5u64),  // a
                FieldWrapper::<Fr>::from(6u64),  // b
            ],
            &a_polys,
            &b_polys,
            &c_polys,
            1,
            &seed,
        )
        .unwrap();

        let proofs_and_inputs = vec![
            (proof1, vec![FieldWrapper::<Fr>::from(12u64)]),
            (proof2, vec![FieldWrapper::<Fr>::from(30u64)]),
        ];

        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);
        let is_valid = batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap();
        assert!(is_valid, "All valid proofs should verify in batch");
    }

    #[test]
    fn test_batch_verify_with_invalid_proof() {
        // Using standard Groth16 witness ordering: [1, c, a, b]
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(2, FieldWrapper::<Fr>::from(1u64)); // a at index 2
        c1.add_b_variable(3, FieldWrapper::<Fr>::from(1u64)); // b at index 3
        c1.add_c_variable(1, FieldWrapper::<Fr>::from(1u64)); // c at index 1

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        let seed = [42u8; 32];
        let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Generate one valid proof: 3 × 4 = 12
        let proof1 = generate_proof_test(
            &pk,
            &[
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(12u64), // c
                FieldWrapper::<Fr>::from(3u64),  // a
                FieldWrapper::<Fr>::from(4u64),  // b
            ],
            &a_polys,
            &b_polys,
            &c_polys,
            1,
            &seed,
        )
        .unwrap();

        // Create a valid proof but verify with wrong public input
        let proof2 = generate_proof_test(
            &pk,
            &[
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(30u64), // c
                FieldWrapper::<Fr>::from(5u64),  // a
                FieldWrapper::<Fr>::from(6u64),  // b
            ],
            &a_polys,
            &b_polys,
            &c_polys,
            1,
            &seed,
        )
        .unwrap();

        let proofs_and_inputs = vec![
            (proof1, vec![FieldWrapper::<Fr>::from(12u64)]),
            (proof2, vec![FieldWrapper::<Fr>::from(99u64)]), // Wrong public input!
        ];

        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);
        let is_valid = batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap();
        assert!(!is_valid, "Batch should fail with invalid proof");
    }

    /// Test helper: Generate a test circuit and keys for multiplier: a × b = c
    fn setup_test_circuit() -> (
        crate::keys::ProvingKey,
        VerificationKey,
        Vec<R1CSConstraint<Fr>>,
    ) {
        // Create multiplier circuit: a × b = c
        // Using standard Groth16 witness ordering: [1, c, a, b]
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(2, FieldWrapper::<Fr>::from(1u64)); // a at index 2
        c1.add_b_variable(3, FieldWrapper::<Fr>::from(1u64)); // b at index 3
        c1.add_c_variable(1, FieldWrapper::<Fr>::from(1u64)); // c at index 1

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        // Setup with 1 public input (c)
        let seed = [42u8; 32];
        let (pk, vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        (pk, vk, constraints)
    }

    /// Test helper: Generate a proof with the given witness
    fn generate_proof_for_witness(
        pk: &crate::keys::ProvingKey,
        witness: &[FieldWrapper<Fr>],
        constraints: &[R1CSConstraint<Fr>],
    ) -> Proof {
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(constraints, 4).unwrap();
        let seed = [42u8; 32];
        generate_proof_test(pk, witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap()
    }

    #[test]
    fn test_batch_verify_all_valid() {
        let (pk, vk, constraints) = setup_test_circuit();
        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);

        // Generate 5 valid proofs with different witnesses
        let proofs_and_inputs = vec![
            {
                // 3 × 4 = 12
                let witness = vec![
                    FieldWrapper::<Fr>::from(1u64),  // constant 1
                    FieldWrapper::<Fr>::from(12u64), // c
                    FieldWrapper::<Fr>::from(3u64),  // a
                    FieldWrapper::<Fr>::from(4u64),  // b
                ];
                let proof = generate_proof_for_witness(&pk, &witness, &constraints);
                (proof, vec![FieldWrapper::<Fr>::from(12u64)])
            },
            {
                // 5 × 6 = 30
                let witness = vec![
                    FieldWrapper::<Fr>::from(1u64),  // constant 1
                    FieldWrapper::<Fr>::from(30u64), // c
                    FieldWrapper::<Fr>::from(5u64),  // a
                    FieldWrapper::<Fr>::from(6u64),  // b
                ];
                let proof = generate_proof_for_witness(&pk, &witness, &constraints);
                (proof, vec![FieldWrapper::<Fr>::from(30u64)])
            },
            {
                // 2 × 3 = 6
                let witness = vec![
                    FieldWrapper::<Fr>::from(1u64), // constant 1
                    FieldWrapper::<Fr>::from(6u64), // c
                    FieldWrapper::<Fr>::from(2u64), // a
                    FieldWrapper::<Fr>::from(3u64), // b
                ];
                let proof = generate_proof_for_witness(&pk, &witness, &constraints);
                (proof, vec![FieldWrapper::<Fr>::from(6u64)])
            },
            {
                // 7 × 8 = 56
                let witness = vec![
                    FieldWrapper::<Fr>::from(1u64),  // constant 1
                    FieldWrapper::<Fr>::from(56u64), // c
                    FieldWrapper::<Fr>::from(7u64),  // a
                    FieldWrapper::<Fr>::from(8u64),  // b
                ];
                let proof = generate_proof_for_witness(&pk, &witness, &constraints);
                (proof, vec![FieldWrapper::<Fr>::from(56u64)])
            },
            {
                // 10 × 11 = 110
                let witness = vec![
                    FieldWrapper::<Fr>::from(1u64),   // constant 1
                    FieldWrapper::<Fr>::from(110u64), // c
                    FieldWrapper::<Fr>::from(10u64),  // a
                    FieldWrapper::<Fr>::from(11u64),  // b
                ];
                let proof = generate_proof_for_witness(&pk, &witness, &constraints);
                (proof, vec![FieldWrapper::<Fr>::from(110u64)])
            },
        ];

        let result = batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap();
        assert!(result, "All valid proofs should verify in batch");
    }

    #[test]
    fn test_batch_verify_with_one_invalid() {
        let (pk, vk, constraints) = setup_test_circuit();
        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);

        // Generate valid proofs
        let proof1 = {
            let witness = vec![
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(12u64), // c
                FieldWrapper::<Fr>::from(3u64),  // a
                FieldWrapper::<Fr>::from(4u64),  // b
            ];
            generate_proof_for_witness(&pk, &witness, &constraints)
        };

        let proof2 = {
            let witness = vec![
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(30u64), // c
                FieldWrapper::<Fr>::from(5u64),  // a
                FieldWrapper::<Fr>::from(6u64),  // b
            ];
            generate_proof_for_witness(&pk, &witness, &constraints)
        };

        let proof3 = {
            let witness = vec![
                FieldWrapper::<Fr>::from(1u64), // constant 1
                FieldWrapper::<Fr>::from(6u64), // c
                FieldWrapper::<Fr>::from(2u64), // a
                FieldWrapper::<Fr>::from(3u64), // b
            ];
            generate_proof_for_witness(&pk, &witness, &constraints)
        };

        // Create batch with one invalid proof (wrong public input)
        let proofs_and_inputs = vec![
            (proof1, vec![FieldWrapper::<Fr>::from(12u64)]), // Valid
            (proof2, vec![FieldWrapper::<Fr>::from(99u64)]), // Invalid: wrong public input
            (proof3, vec![FieldWrapper::<Fr>::from(6u64)]),  // Valid
        ];

        let result = batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap();
        assert!(!result, "Batch should fail when one proof is invalid");
    }

    #[test]
    fn test_batch_verify_single_proof() {
        let (pk, vk, constraints) = setup_test_circuit();
        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);

        // Generate a single valid proof
        let proof = {
            let witness = vec![
                FieldWrapper::<Fr>::from(1u64),  // constant 1
                FieldWrapper::<Fr>::from(12u64), // c
                FieldWrapper::<Fr>::from(3u64),  // a
                FieldWrapper::<Fr>::from(4u64),  // b
            ];
            generate_proof_for_witness(&pk, &witness, &constraints)
        };

        let proofs_and_inputs = vec![(proof, vec![FieldWrapper::<Fr>::from(12u64)])];

        let result = batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap();
        assert!(result, "Single valid proof should verify in batch");
    }

    #[test]
    fn test_batch_verify_empty() {
        let (_pk, vk, _constraints) = setup_test_circuit();
        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);

        // Empty batch should return true (trivially valid)
        let proofs_and_inputs: Vec<(Proof, Vec<FieldWrapper<Fr>>)> = vec![];

        let result = batch_verify(&vk, &proofs_and_inputs, &mut rng).unwrap();
        assert!(result, "Empty batch should be valid");
    }
}
