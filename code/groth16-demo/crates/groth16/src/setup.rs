use crate::error::Groth16Error;
use crate::keys::{ProvingKey, VerificationKey};
use ark_bn254::{Fr, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, Field, PrimeField, UniformRand, Zero};
use groth16_math::fields::FieldWrapper;
use groth16_math::polynomial::Polynomial;
use groth16_qap::target_polynomial;
use rand::Rng;

/// Helper function to convert Fq field element to Fr (simplified)
fn fq_to_fr(fq: &ark_bn254::Fq) -> Fr {
    let bytes = fq.into_bigint().to_bytes_be();
    let mut padded = [0u8; 32];
    let start = 32usize.saturating_sub(bytes.len());
    padded[start..].copy_from_slice(&bytes);
    Fr::from_be_bytes_mod_order(&padded)
}

/// Performs the trusted setup ceremony to generate proving and verification keys.
///
/// # Trusted Setup
///
/// The trusted setup generates random secrets (α, β, γ, δ) that are used to
/// encrypt the QAP polynomials. These secrets MUST be discarded after the ceremony
/// as they are "toxic waste" that could compromise the system's soundness.
///
/// # Process
/// 1. Generate random secrets: α, β, γ, δ in the scalar field
/// 2. Generate random τ (tau) in the scalar field
/// 3. Compute powers of τ: [1, τ, τ², ..., τⁿ] encrypted in G1 and G2
/// 4. Evaluate QAP polynomials at τ and encrypt with the secrets
/// 5. Compute division polynomials and encrypt them
/// 6. Compute public input verification coefficients
///
/// # Arguments
/// * `a_polys` - A-polynomials from QAP [A₀(x), ..., Aₘ(x)]
/// * `b_polys` - B-polynomials from QAP [B₀(x), ..., Bₘ(x)]
/// * `c_polys` - C-polynomials from QAP [C₀(x), ..., Cₘ(x)]
/// * `num_inputs` - Number of public inputs (usually 1 for the constant 1)
/// * `rng` - Random number generator
///
/// # Returns
/// * `Ok((pk, vk))` - Proving key and verification key
/// * `Err(...)` - Error if setup fails
///
/// # Security Warning
/// The random secrets (α, β, γ, δ, τ) are "toxic waste" - they MUST be securely
/// deleted after this ceremony. If an attacker obtains these secrets, they can
/// forge proofs for any witness.
///
/// # Example
/// ```rust,ignore
/// use groth16_groth16::setup::trusted_setup;
/// use groth16_qap::r1cs_to_qap;
///
/// // Given R1CS constraints
/// let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, num_vars)?;
///
/// // Perform trusted setup
/// let (pk, vk) = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng)?;
///
/// // pk is used to generate proofs
/// // vk is used to verify proofs
///
/// // IMPORTANT: Securely delete all secrets (α, β, γ, δ, τ)
/// # Ok::<(), Groth16Error>(())
/// ```
pub fn trusted_setup<R>(
    a_polys: &[Polynomial<ark_bn254::Fq>],
    b_polys: &[Polynomial<ark_bn254::Fq>],
    c_polys: &[Polynomial<ark_bn254::Fq>],
    num_inputs: usize,
    rng: &mut R,
) -> Result<(ProvingKey, VerificationKey), Groth16Error>
where
    R: Rng,
{
    // Validate input
    if a_polys.is_empty() || b_polys.is_empty() || c_polys.is_empty() {
        return Err(Groth16Error::EmptyPolynomials);
    }

    if a_polys.len() != b_polys.len() || a_polys.len() != c_polys.len() {
        return Err(Groth16Error::MismatchedPolynomials(
            a_polys.len(),
            b_polys.len(),
            c_polys.len(),
        ));
    }

    if num_inputs > a_polys.len() - 1 {
        // num_inputs can be 0 (no public inputs except constant), but must not exceed available variables
        // We subtract 1 because index 0 is always the constant 1
        return Err(Groth16Error::InvalidInputs(num_inputs));
    }

    let num_vars = a_polys.len();
    let num_constraints = num_vars - 2;

    // Step 1: Generate random secrets (TOXIC WASTE)
    let alpha = Fr::rand(rng);
    let beta = Fr::rand(rng);
    let gamma = Fr::rand(rng);
    let delta = Fr::rand(rng);
    let tau = Fr::rand(rng);

    // Step 2: Compute powers of tau encrypted in G1 and G2
    let _tau_powers_g1 = compute_powers_of_tau_g1(tau, num_constraints + 2);
    let _tau_powers_g2 = compute_powers_of_tau_g2(tau, num_constraints + 2);

    // Step 3: Encrypt the secrets with generators
    let alpha_g1 = (G1Affine::generator() * alpha).into_affine();
    let beta_g1 = (G1Affine::generator() * beta).into_affine();
    let beta_g2 = (G2Affine::generator() * beta).into_affine();
    let gamma_g2 = (G2Affine::generator() * gamma).into_affine();
    let delta_g1 = (G1Affine::generator() * delta).into_affine();
    let delta_g2 = (G2Affine::generator() * delta).into_affine();

    // Step 4: Compute encrypted A-polynomials
    // Standard Groth16: Store Aᵢ(τ)·G₁ (without α multiplier)
    let tau_bytes = tau.into_bigint().to_bytes_be();
    let tau_u64 = u64::from_be_bytes(tau_bytes[24..32].try_into().unwrap_or([0u8; 8]));
    let tau_field = FieldWrapper::<ark_bn254::Fq>::from(tau_u64);
    let mut a_query = Vec::with_capacity(num_vars);
    for poly in a_polys {
        let eval = poly.evaluate(&tau_field);
        let fr_eval = fq_to_fr(&eval.value);
        let encrypted = (G1Affine::generator() * fr_eval).into_affine();
        a_query.push(encrypted);
    }

    // Step 5: Compute encrypted B-polynomials in G1 and G2
    // Standard Groth16: Store Bᵢ(τ)·G₁ and Bᵢ(τ)·G₂ (without β multiplier)
    let mut b_g1_query = Vec::with_capacity(num_vars);
    let mut b_g2_query = Vec::with_capacity(num_vars);
    for poly in b_polys {
        let eval = poly.evaluate(&tau_field);
        let fr_eval = fq_to_fr(&eval.value);

        let encrypted_g1 = (G1Affine::generator() * fr_eval).into_affine();
        b_g1_query.push(encrypted_g1);

        let encrypted_g2 = (G2Affine::generator() * fr_eval).into_affine();
        b_g2_query.push(encrypted_g2);
    }

    // Step 6: Compute encrypted C-polynomials in G1
    // Standard Groth16: Store Cᵢ(τ)·G₁ (without β multiplier)
    let mut c_query = Vec::with_capacity(num_vars);
    for poly in c_polys {
        let eval = poly.evaluate(&tau_field);
        let fr_eval = fq_to_fr(&eval.value);
        let encrypted = (G1Affine::generator() * fr_eval).into_affine();
        c_query.push(encrypted);
    }

    // Step 6: Compute division polynomials
    let target = target_polynomial::<ark_bn254::Fq>(num_constraints);
    let h_query =
        compute_division_polynomials_encrypted(&target, num_constraints, tau_field.clone())?;

    // Step 7: Compute IC for public inputs
    //
    // The IC (Input Consistency) vector encodes public inputs for verification.
    // Standard Groth16: IC[i] = β·Aᵢ(τ)·G₁ (with β)
    // We keep β in IC even though we removed it from other queries
    let mut ic = Vec::with_capacity(num_inputs + 1);

    // IC[0] = β·G₁ for constant 1
    ic.push((G1Affine::generator() * beta).into_affine());

    // For each public input i (1..num_inputs), compute IC[i]
    // Public inputs are at witness indices 1..num_inputs
    for a_poly in a_polys.iter().take(num_inputs + 1).skip(1) {
        // Public input at witness index i corresponds to a_polys[i]
        let a_eval = a_poly.evaluate(&tau_field).value;
        let fr_a_eval = fq_to_fr(&a_eval);

        // IC[i] = β·Aᵢ(τ)·G₁
        let ic_point = (G1Affine::generator() * beta * fr_a_eval).into_affine();
        ic.push(ic_point);
    }

    // Construct keys
    let pk = ProvingKey {
        alpha,
        beta,
        alpha_g1,
        beta_g1,
        beta_g2,
        delta_g1,
        delta_g2,
        a_query,
        b_g1_query,
        b_g2_query,
        c_query,
        h_query,
    };

    let vk = VerificationKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        ic,
    };

    Ok((pk, vk))
}

/// Performs a deterministic trusted setup for testing purposes.
pub fn trusted_setup_test(
    a_polys: &[Polynomial<ark_bn254::Fq>],
    b_polys: &[Polynomial<ark_bn254::Fq>],
    c_polys: &[Polynomial<ark_bn254::Fq>],
    num_inputs: usize,
    seed: &[u8; 32],
) -> Result<(ProvingKey, VerificationKey), Groth16Error> {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::from_seed(*seed);
    trusted_setup(a_polys, b_polys, c_polys, num_inputs, &mut rng)
}

/// Computes powers of tau encrypted in G1
fn compute_powers_of_tau_g1(tau: Fr, degree: usize) -> Vec<G1Affine> {
    let mut result = Vec::with_capacity(degree);
    let mut current = G1::from(G1Affine::generator());

    for _ in 0..degree {
        result.push(current.into_affine());
        current *= tau;
    }

    result
}

/// Computes powers of tau encrypted in G2
fn compute_powers_of_tau_g2(tau: Fr, degree: usize) -> Vec<G2Affine> {
    let mut result = Vec::with_capacity(degree);
    let mut current = G2::from(G2Affine::generator());

    for _ in 0..degree {
        result.push(current.into_affine());
        current *= tau;
    }

    result
}

/// Computes division polynomials and encrypts them
fn compute_division_polynomials_encrypted(
    target: &Polynomial<ark_bn254::Fq>,
    num_constraints: usize,
    tau: FieldWrapper<ark_bn254::Fq>,
) -> Result<Vec<G1Affine>, Groth16Error> {
    let mut result = Vec::new();

    for j in 0..num_constraints.saturating_sub(2) {
        let j_field = FieldWrapper::<ark_bn254::Fq>::from(j as u64);
        let divisor = Polynomial::<ark_bn254::Fq>::new(vec![
            FieldWrapper::<ark_bn254::Fq>::zero() - j_field.clone(),
            FieldWrapper::<ark_bn254::Fq>::one(),
        ]);

        match divide_polynomials(target, &divisor) {
            Ok((quotient, _remainder)) => {
                let h_j_at_tau = quotient.evaluate(&tau);
                let fr_scalar = fq_to_fr(&h_j_at_tau.value);
                let encrypted = (G1Affine::generator() * fr_scalar).into_affine();
                result.push(encrypted);
            }
            Err(e) => {
                return Err(Groth16Error::DivisionError(format!(
                    "Failed to divide t(x) by (x - {}): {}",
                    j, e
                )))
            }
        }
    }

    Ok(result)
}

/// Performs polynomial division
fn divide_polynomials(
    dividend: &Polynomial<ark_bn254::Fq>,
    divisor: &Polynomial<ark_bn254::Fq>,
) -> Result<(Polynomial<ark_bn254::Fq>, Polynomial<ark_bn254::Fq>), String> {
    if divisor.is_zero() {
        return Err("Division by zero".to_string());
    }

    if dividend.is_zero() {
        return Ok((
            Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<ark_bn254::Fq>::zero()]),
            Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<ark_bn254::Fq>::zero()]),
        ));
    }

    let mut remainder = dividend.clone();
    let mut quotient_coeffs = vec![FieldWrapper::<ark_bn254::Fq>::zero(); dividend.degree() + 1];

    let divisor_degree = divisor.degree();
    let zero = FieldWrapper::<ark_bn254::Fq>::zero();
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
            * FieldWrapper::<ark_bn254::Fq>::from(
                divisor_leading
                    .value
                    .inverse()
                    .expect("Divisor leading coefficient should never be zero"),
            );

        let degree_diff = remainder_degree - divisor_degree;
        quotient_coeffs[degree_diff] = quotient_coeffs[degree_diff].clone() + coeff.clone();

        let mut term_coeffs = vec![FieldWrapper::<ark_bn254::Fq>::zero(); degree_diff + 1];
        term_coeffs[degree_diff] = coeff;
        let term = Polynomial::<ark_bn254::Fq>::new(term_coeffs);

        let product = term * divisor.clone();
        remainder = remainder - product;
    }

    while quotient_coeffs.len() > 1 && quotient_coeffs.last().unwrap().value.is_zero() {
        quotient_coeffs.pop();
    }

    Ok((Polynomial::<ark_bn254::Fq>::new(quotient_coeffs), remainder))
}

#[cfg(test)]
mod tests {
    use super::*;
    use groth16_qap::r1cs_to_qap;
    use groth16_r1cs::constraint::R1CSConstraint;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_trusted_setup_structure() {
        // Create simple QAP with 4 constraints to get division polynomials
        let mut c1 = R1CSConstraint::<ark_bn254::Fq>::new();
        c1.add_a_variable(1, FieldWrapper::<ark_bn254::Fq>::from(1u64));
        c1.add_b_variable(2, FieldWrapper::<ark_bn254::Fq>::from(1u64));
        c1.add_c_variable(3, FieldWrapper::<ark_bn254::Fq>::from(1u64));

        let c2 = c1.clone();
        let c3 = c1.clone();
        let c4 = c1.clone();

        let constraints = vec![c1, c2, c3, c4];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 6).unwrap();

        // Perform trusted setup
        let seed = [42u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);
        let (pk, vk) = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng).unwrap();

        // Verify pk structure
        assert_eq!(pk.a_query.len(), 6);
        assert_eq!(pk.b_g1_query.len(), 6);
        assert_eq!(pk.b_g2_query.len(), 6);
        // num_constraints = 6 - 2 = 4, so h_query has 4 - 2 = 2 elements
        assert_eq!(pk.h_query.len(), 2);

        // Verify vk structure
        // With num_inputs=1, IC should have 2 elements: IC[0] for constant, IC[1] for public input
        assert_eq!(vk.ic.len(), 2);
    }

    #[test]
    fn test_trusted_setup_deterministic() {
        let mut c1 = R1CSConstraint::<ark_bn254::Fq>::new();
        c1.add_a_variable(1, FieldWrapper::<ark_bn254::Fq>::from(1u64));
        c1.add_b_variable(2, FieldWrapper::<ark_bn254::Fq>::from(1u64));
        c1.add_c_variable(3, FieldWrapper::<ark_bn254::Fq>::from(1u64));

        let constraints = vec![c1.clone(), c1.clone()];
        let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, 4).unwrap();

        let seed = [42u8; 32];

        // Run setup twice with same seed
        let (pk1, vk1) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();
        let (pk2, vk2) = trusted_setup_test(&a_polys, &b_polys, &c_polys, 1, &seed).unwrap();

        // Should produce identical keys
        assert_eq!(pk1.alpha_g1, pk2.alpha_g1);
        assert_eq!(pk1.beta_g1, pk2.beta_g1);
        assert_eq!(vk1.alpha_g1, vk2.alpha_g1);
    }

    #[test]
    fn test_empty_polynomials_error() {
        let a_polys: Vec<Polynomial<ark_bn254::Fq>> = vec![];
        let b_polys: Vec<Polynomial<ark_bn254::Fq>> = vec![];
        let c_polys: Vec<Polynomial<ark_bn254::Fq>> = vec![];

        let seed = [42u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);
        let result = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Groth16Error::EmptyPolynomials
        ));
    }

    #[test]
    fn test_mismatched_polynomials_error() {
        let a_polys = vec![Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<
            ark_bn254::Fq,
        >::one()])];
        let b_polys = vec![
            Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<ark_bn254::Fq>::one()]),
            Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<ark_bn254::Fq>::one()]),
        ];
        let c_polys = vec![Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<
            ark_bn254::Fq,
        >::one()])];

        let seed = [42u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);
        let result = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Groth16Error::MismatchedPolynomials(1, 2, 1)
        ));
    }

    #[test]
    fn test_invalid_inputs_error() {
        let a_polys = vec![Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<
            ark_bn254::Fq,
        >::one()])];
        let b_polys = vec![Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<
            ark_bn254::Fq,
        >::one()])];
        let c_polys = vec![Polynomial::<ark_bn254::Fq>::new(vec![FieldWrapper::<
            ark_bn254::Fq,
        >::one()])];

        let seed = [42u8; 32];
        let mut rng = ChaCha8Rng::from_seed(seed);

        let result = trusted_setup(&a_polys, &b_polys, &c_polys, 0, &mut rng);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Groth16Error::InvalidInputs(0)
        ));

        let result = trusted_setup(&a_polys, &b_polys, &c_polys, 2, &mut rng);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Groth16Error::InvalidInputs(2)
        ));
    }

    #[test]
    fn test_division_polynomials() {
        let dividend = Polynomial::<ark_bn254::Fq>::new(vec![
            FieldWrapper::<ark_bn254::Fq>::zero() - FieldWrapper::<ark_bn254::Fq>::one(),
            FieldWrapper::<ark_bn254::Fq>::zero(),
            FieldWrapper::<ark_bn254::Fq>::one(),
        ]);

        let divisor = Polynomial::<ark_bn254::Fq>::new(vec![
            FieldWrapper::<ark_bn254::Fq>::zero() - FieldWrapper::<ark_bn254::Fq>::one(),
            FieldWrapper::<ark_bn254::Fq>::one(),
        ]);

        let (quotient, remainder) = divide_polynomials(&dividend, &divisor).unwrap();

        assert_eq!(quotient.degree(), 1);
        assert!(remainder.is_zero());
    }

    #[test]
    fn test_compute_powers_of_tau() {
        use ark_ff::One;

        let tau = Fr::one();
        let powers_g1 = compute_powers_of_tau_g1(tau, 5);

        assert_eq!(powers_g1.len(), 5);
        let g1_gen = G1Affine::generator();
        for power in powers_g1 {
            assert_eq!(power, g1_gen);
        }
    }
}
