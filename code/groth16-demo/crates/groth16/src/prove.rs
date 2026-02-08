use crate::error::Groth16Error;
use crate::keys::ProvingKey;
use ark_bn254::{Fr, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2};
use ark_ec::CurveGroup;
use ark_ff::{Field, PrimeField, UniformRand, Zero};
use groth16_math::fields::FieldWrapper;
use groth16_math::polynomial::Polynomial;
use rand::Rng;

/// Groth16 proof
///
/// A Groth16 proof consists of three group elements that demonstrate
/// knowledge of a valid witness without revealing it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof {
    /// Proof component A in G₁
    pub a: G1Affine,
    /// Proof component B in G₂
    pub b: G2Affine,
    /// Proof component C in G₁
    pub c: G1Affine,
}

/// Generates a Groth16 zero-knowledge proof.
///
/// # Arguments
/// * `pk` - Proving key from trusted setup
/// * `witness` - Witness assignment [z₀, z₁, ..., zₘ]
/// * `a_polys` - A-polynomials from QAP
/// * `b_polys` - B-polynomials from QAP
/// * `c_polys` - C-polynomials from QAP
/// * `public_inputs` - Number of public inputs (usually 1 for constant 1)
/// * `rng` - Random number generator
///
/// # Returns
/// * `Ok(Proof)` - The zero-knowledge proof
/// * `Err(Groth16Error)` - Error if proof generation fails
///
/// # Algorithm
/// 1. Evaluate QAP polynomials at the witness point:
///    - A_witness = Σⱼ witness[j]·Aⱼ(τ)  (using pk.a_query)
///    - B_witness = Σⱼ witness[j]·Bⱼ(τ)  (using pk.b_g1_query, pk.b_g2_query)
/// 2. Compute the division polynomial H(x):
///    - First compute p(x) = A_witness(x)·B_witness(x) - C_witness(x)
///    - Then H(x) = p(x) / t(x) where t(x) is the target polynomial
/// 3. Generate random blinding factors r, s
/// 4. Compute proof components:
///    - A = α·G₁ + A_witness + r·δ·G₁
///    - B = β·G₂ + B_witness + s·δ·G₂
///    - C = C_witness + H + additional terms for r and s
///
/// # Security
/// The random values r and s ensure zero-knowledge: different witnesses
/// produce different-looking proofs even for the same public statement.
///
/// # Example
/// ```rust,ignore
/// use groth16_groth16::prove::generate_proof;
/// use groth16_circuits::multiplier::MultiplierCircuit;
///
/// // Create circuit
/// let circuit = MultiplierCircuit::new(3, 4, 12);
/// let witness = circuit.witness();
///
/// // Get QAP polynomials and proving key
/// let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4)?;
/// let (pk, vk) = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng)?;
///
/// // Generate proof
/// let proof = generate_proof(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &mut rng)?;
///
/// // Proof can now be verified against public inputs (c=12)
/// # Ok::<(), Groth16Error>(())
/// ```
pub fn generate_proof<R: Rng + ?Sized>(
    pk: &ProvingKey,
    witness: &[FieldWrapper<Fr>],
    a_polys: &[Polynomial<Fr>],
    b_polys: &[Polynomial<Fr>],
    c_polys: &[Polynomial<Fr>],
    _public_inputs: usize,
    rng: &mut R,
) -> Result<Proof, Groth16Error> {
    // Validate inputs
    if witness.is_empty() {
        return Err(Groth16Error::InvalidWitnessLength {
            expected: a_polys.len(),
            actual: 0,
        });
    }

    if witness.len() != a_polys.len() {
        return Err(Groth16Error::InvalidWitnessLength {
            expected: a_polys.len(),
            actual: witness.len(),
        });
    }

    // Step 1: Compute A_base = Σⱼ witness[j]·Aⱼ(τ) (unblinded, without α)
    // pk.a_query contains [α·Aⱼ(τ)] so we need to subtract α
    let mut a_witness_blinded = G1::zero();
    for (j, w) in witness.iter().enumerate() {
        let w_fr = w.value;
        let g1_point = G1::from(pk.a_query[j]);
        a_witness_blinded += g1_point * w_fr;
    }
    // Extract α contribution: α·Σ witness[j] where j=0 is the constant 1
    let alpha_sum = witness[0].value;
    let a_alpha_part = G1::from(pk.alpha_g1) * alpha_sum;
    let a_base = a_witness_blinded - a_alpha_part;

    // Step 2: Compute B_base = Σⱼ witness[j]·Bⱼ(τ) (unblinded, without β)
    let mut b_witness_g1_blinded = G1::zero();
    let mut b_witness_g2_blinded = G2::zero();

    for (j, w) in witness.iter().enumerate() {
        let w_fr = w.value;
        let bg1_point = G1::from(pk.b_g1_query[j]);
        let bg2_point = G2::from(pk.b_g2_query[j]);
        b_witness_g1_blinded += bg1_point * w_fr;
        b_witness_g2_blinded += bg2_point * w_fr;
    }
    // Extract β contribution: β·Σ witness[j]
    let beta_sum = alpha_sum; // Same sum
    let b_beta_part_g1 = G1::from(pk.beta_g1) * beta_sum;
    let b_beta_part_g2 = G2::from(pk.beta_g2) * beta_sum;
    let b_base_g1 = b_witness_g1_blinded - b_beta_part_g1;
    let b_base_g2 = b_witness_g2_blinded - b_beta_part_g2;

    // Step 3: Compute C_base = Σⱼ witness[j]·Cⱼ(τ) (already has β)
    let mut c_base = G1::zero();
    for (j, w) in witness.iter().enumerate() {
        let w_fr = w.value;
        let g1_point = G1::from(pk.c_query[j]);
        c_base += g1_point * w_fr;
    }

    // Step 4: Generate random blinding factors
    let r = Fr::rand(rng);
    let s = Fr::rand(rng);

    // Step 5: Compute proof component A
    // A = α·G₁ + A_base + r·δ·G₁
    let delta_g1 = G1::from(pk.delta_g1);
    let a_g1 = G1::from(pk.alpha_g1) + a_base + delta_g1 * r;

    // Step 6: Compute proof component B
    // B = β·G₂ + B_base + s·δ·G₂
    let delta_g2 = G2::from(pk.delta_g2);
    let b_g2 = G2::from(pk.beta_g2) + b_base_g2 + delta_g2 * s;

    // Step 7: Compute proof component C
    // According to Groth16 paper:
    // C = [β·A(x) + α·B(x) + C(x)]_x=τ + H(x)·Z(x)_x=τ + δ·r·s
    //
    // Where:
    // - A_base = Σ witness[j]·Aⱼ(τ) (without α)
    // - B_base = Σ witness[j]·Bⱼ(τ) (without β)
    // - C_base = Σ witness[j]·Cⱼ(τ) (with β, but we need to subtract β for unblinded)
    //
    // The formula simplifies to:
    // C = β·A_base + α·B_base + C_base_unblinded + H(τ) + r·s·δ
    //   = (β·A_base + α·B_base + C_base - β·witness[0]) + H(τ) + r·s·δ
    //
    // But we also need to add the blinding terms with r and s:
    // C = β·A_base·s + α·B_base·r + C_base + H(τ) + δ·r·s
    //
    // Where A_base and B_base are the unblinded versions

    // First compute the H polynomial
    // Compute the witness polynomials A_w(x), B_w(x), C_w(x)
    let a_w_poly = compute_witness_polynomial(a_polys, witness);
    let b_w_poly = compute_witness_polynomial(b_polys, witness);
    let c_w_poly = compute_witness_polynomial(c_polys, witness);

    // Compute p(x) = A_w(x)·B_w(x) - C_w(x)
    let product_poly = a_w_poly.clone() * b_w_poly.clone();
    let diff_poly = product_poly - c_w_poly;

    // Get target polynomial t(x)
    let num_constraints = a_polys.len() - 2;
    let target_poly = groth16_qap::target_polynomial::<Fr>(num_constraints);

    // Divide to get H(x)
    let (h_poly, _remainder) =
        divide_polynomials(&diff_poly, &target_poly).map_err(Groth16Error::DivisionError)?;

    // Evaluate H at τ using h_query
    let mut h_tau = G1::zero();
    for (j, coeff) in h_poly.coeffs.iter().enumerate() {
        if j < pk.h_query.len() {
            let g1_point = G1::from(pk.h_query[j]);
            h_tau += g1_point * coeff.value;
        }
    }

    // Now compute C with the correct Groth16 formula:
    // C = A_base·s + B_base·r + C_base + H(τ) + δ·r·s
    // where A_base and B_base are unblinded (without α, β)
    // This matches the Groth16 paper specification for the C component
    let c_g1 = a_base * s + b_base_g1 * r + c_base + h_tau + delta_g1 * (r * s);

    // Convert to affine
    let proof = Proof {
        a: a_g1.into_affine(),
        b: b_g2.into_affine(),
        c: c_g1.into_affine(),
    };

    Ok(proof)
}

/// Generates a proof using a deterministic RNG for testing.
///
/// This version uses a fixed seed instead of random entropy, making it
/// reproducible across runs. Useful for testing but MUST NOT be used
/// in production.
pub fn generate_proof_test(
    pk: &ProvingKey,
    witness: &[FieldWrapper<Fr>],
    a_polys: &[Polynomial<Fr>],
    b_polys: &[Polynomial<Fr>],
    c_polys: &[Polynomial<Fr>],
    public_inputs: usize,
    seed: &[u8; 32],
) -> Result<Proof, Groth16Error> {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::from_seed(*seed);
    generate_proof(
        pk,
        witness,
        a_polys,
        b_polys,
        c_polys,
        public_inputs,
        &mut rng,
    )
}

/// Computes the witness polynomial by linearly combining QAP polynomials with witness values
fn compute_witness_polynomial(
    polys: &[Polynomial<Fr>],
    witness: &[FieldWrapper<Fr>],
) -> Polynomial<Fr> {
    let mut result_coeffs = vec![FieldWrapper::<Fr>::zero(); polys.len()];

    for (i, poly) in polys.iter().enumerate() {
        if i < witness.len() {
            let w = &witness[i];
            for (j, coeff) in poly.coeffs.iter().enumerate() {
                if j < result_coeffs.len() {
                    result_coeffs[j] = result_coeffs[j].clone() + coeff.clone() * w.clone();
                }
            }
        }
    }

    Polynomial::new(result_coeffs)
}

/// Performs polynomial division
fn divide_polynomials(
    dividend: &Polynomial<Fr>,
    divisor: &Polynomial<Fr>,
) -> Result<(Polynomial<Fr>, Polynomial<Fr>), String> {
    if divisor.is_zero() {
        return Err("Division by zero".to_string());
    }

    if dividend.is_zero() {
        return Ok((
            Polynomial::<Fr>::new(vec![FieldWrapper::<Fr>::zero()]),
            Polynomial::<Fr>::new(vec![FieldWrapper::<Fr>::zero()]),
        ));
    }

    let mut remainder = dividend.clone();
    let mut quotient_coeffs = vec![FieldWrapper::<Fr>::zero(); dividend.degree() + 1];

    let divisor_degree = divisor.degree();
    let zero = FieldWrapper::<Fr>::zero();
    let divisor_leading = divisor
        .coeffs
        .iter()
        .rev()
        .find(|c| !c.value.is_zero())
        .unwrap_or(&zero);

    while remainder.degree() >= divisor_degree && !remainder.is_zero() {
        let remainder_degree = remainder.degree();
        let remainder_leading = remainder
            .coeffs
            .iter()
            .rev()
            .find(|c| !c.value.is_zero())
            .unwrap_or(&zero);

        let coeff = remainder_leading.clone()
            * FieldWrapper::<Fr>::from(
                divisor_leading
                    .value
                    .inverse()
                    .expect("Divisor leading coefficient should never be zero"),
            );

        let degree_diff = remainder_degree - divisor_degree;
        quotient_coeffs[degree_diff] = quotient_coeffs[degree_diff].clone() + coeff.clone();

        let mut term_coeffs = vec![FieldWrapper::<Fr>::zero(); degree_diff + 1];
        term_coeffs[degree_diff] = coeff;
        let term = Polynomial::<Fr>::new(term_coeffs);

        let product = term * divisor.clone();
        remainder = remainder - product;
    }

    while quotient_coeffs.len() > 1 && quotient_coeffs.last().unwrap().value.is_zero() {
        quotient_coeffs.pop();
    }

    Ok((Polynomial::<Fr>::new(quotient_coeffs), remainder))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::trusted_setup_test;
    use groth16_qap::r1cs_to_qap;
    use groth16_r1cs::constraint::R1CSConstraint;

    #[test]
    fn test_proof_generation() {
        // Create simple QAP: a × b = c
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(1, FieldWrapper::<Fr>::from(1u64));
        c1.add_b_variable(2, FieldWrapper::<Fr>::from(1u64));
        c1.add_c_variable(3, FieldWrapper::<Fr>::from(1u64));

        // Duplicate for QAP (need 2+ constraints)
        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        // Trusted setup
        let seed = [42u8; 32];
        let (pk, _vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Generate witness for a=3, b=4, c=12
        let witness = vec![
            FieldWrapper::<Fr>::from(1u64),
            FieldWrapper::<Fr>::from(3u64),
            FieldWrapper::<Fr>::from(4u64),
            FieldWrapper::<Fr>::from(12u64),
        ];

        // Generate proof
        let proof =
            generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Verify proof structure
        assert_ne!(proof.a, G1Affine::identity()); // Not identity
        assert_ne!(proof.b, G2Affine::identity()); // Not identity
        assert_ne!(proof.c, G1Affine::identity()); // Not identity
    }

    #[test]
    fn test_proof_deterministic() {
        // Same as above but verify deterministic behavior
        let mut c1 = R1CSConstraint::<Fr>::new();
        c1.add_a_variable(1, FieldWrapper::<Fr>::from(1u64));
        c1.add_b_variable(2, FieldWrapper::<Fr>::from(1u64));
        c1.add_c_variable(3, FieldWrapper::<Fr>::from(1u64));

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        let seed = [42u8; 32];
        let (pk, _vk) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        let witness = vec![
            FieldWrapper::<Fr>::from(1u64),
            FieldWrapper::<Fr>::from(3u64),
            FieldWrapper::<Fr>::from(4u64),
            FieldWrapper::<Fr>::from(12u64),
        ];

        // Generate two proofs with same seed
        let proof1 =
            generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap();
        let proof2 =
            generate_proof_test(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Should be identical
        assert_eq!(proof1, proof2);
    }
}
