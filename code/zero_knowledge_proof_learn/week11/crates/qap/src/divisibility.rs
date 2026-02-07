use crate::error::QapError;
use ark_ff::PrimeField;
use groth16_math::{fields::FieldWrapper, polynomial::Polynomial};

/// Checks if a witness polynomial is divisible by the target polynomial.
///
/// Given witness values, QAP polynomials (A, B, C), and the target polynomial t(x),
/// this function verifies that the witness satisfies all R1CS constraints by checking
/// if p(x) / t(x) has no remainder.
///
/// # Arguments
/// * `witness` - The witness assignment (z₀, z₁, ..., zₘ)
/// * `a_polynomials` - Vector of A polynomials [A₀(x), A₁(x), ..., Aₘ(x)]
/// * `b_polynomials` - Vector of B polynomials [B₀(x), B₁(x), ..., Bₘ(x)]
/// * `c_polynomials` - Vector of C polynomials [C₀(x), C₁(x), ..., Cₘ(x)]
/// * `target` - The target polynomial t(x) = ∏ᵢ₌₁ⁿ (x - i)
///
/// # Returns
/// * `Ok(true)` - If p(x) is divisible by t(x) (witness is valid)
/// * `Ok(false)` - If p(x) is not divisible by t(x) (witness is invalid)
/// * `Err(QapError::MismatchedLengths)` - If witness length doesn't match polynomial vectors
///
/// # Algorithm
/// 1. Compute a(x) = Σⱼ witness[j] · Aⱼ(x)
/// 2. Compute b(x) = Σⱼ witness[j] · Bⱼ(x)
/// 3. Compute c(x) = Σⱼ witness[j] · Cⱼ(x)
/// 4. Compute p(x) = a(x) · b(x) - c(x)
/// 5. Check if p(x) is divisible by t(x) using polynomial long division
///
/// # Example
/// ```rust,ignore
/// use groth16_qap::check_divisibility;
/// use groth16_math::{fields::FieldWrapper, polynomial::Polynomial};
/// use ark_bn254::Fq;
///
/// // Given QAP polynomials and witness from a valid R1CS instance
/// let witness = vec![FieldWrapper::<Fq>::from(1u64), /* ... */];
/// let a_polys = vec![/* ... */];
/// let b_polys = vec![/* ... */];
/// let c_polys = vec![/* ... */];
/// let target = Polynomial::<Fq> { /* ... */ };
///
/// let is_valid = check_divisibility(&witness, &a_polys, &b_polys, &c_polys, &target);
/// assert!(is_valid.unwrap());
/// ```
pub fn check_divisibility<F>(
    witness: &[FieldWrapper<F>],
    a_polynomials: &[Polynomial<F>],
    b_polynomials: &[Polynomial<F>],
    c_polynomials: &[Polynomial<F>],
    target: &Polynomial<F>,
) -> Result<bool, QapError>
where
    F: PrimeField,
{
    // Check that witness length matches polynomial vectors
    let witness_len = witness.len();
    let a_len = a_polynomials.len();
    let b_len = b_polynomials.len();
    let c_len = c_polynomials.len();

    if witness_len != a_len || witness_len != b_len || witness_len != c_len {
        return Err(QapError::MismatchedLengths(
            witness_len,
            a_len.max(b_len).max(c_len),
        ));
    }

    // Compute a(x) = Σⱼ witness[j] · Aⱼ(x)
    let mut a = Polynomial::<F>::new(vec![FieldWrapper::zero()]);
    for (j, poly) in a_polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        a = a + scaled;
    }

    // Compute b(x) = Σⱼ witness[j] · Bⱼ(x)
    let mut b = Polynomial::<F>::new(vec![FieldWrapper::zero()]);
    for (j, poly) in b_polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        b = b + scaled;
    }

    // Compute c(x) = Σⱼ witness[j] · Cⱼ(x)
    let mut c = Polynomial::<F>::new(vec![FieldWrapper::zero()]);
    for (j, poly) in c_polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        c = c + scaled;
    }

    // Compute p(x) = a(x) · b(x) - c(x)
    let ab = a.clone() * b.clone();
    let p = ab - c;

    // Check if p(x) is divisible by t(x)
    let (_quotient, remainder) = polynomial_long_division(&p, target)?;

    // Witness is valid iff remainder is zero
    Ok(remainder.is_zero())
}

/// Performs polynomial long division to divide one polynomial by another.
///
/// Returns the quotient and remainder such that:
/// dividend = divisor * quotient + remainder
/// where degree(remainder) < degree(divisor)
///
/// # Arguments
/// * `dividend` - The polynomial to divide
/// * `divisor` - The polynomial to divide by (must be non-zero)
///
/// # Returns
/// * `Ok((quotient, remainder))` - The quotient and remainder polynomials
/// * `Err(QapError::DivisionByZero)` - If divisor is the zero polynomial
///
/// # Algorithm
/// Standard polynomial long division:
/// 1. While degree(dividend) >= degree(divisor):
///    a. Compute term = (leading_coefficient(dividend) / leading_coefficient(divisor)) · x^(degree_diff)
///    b. Add term to quotient
///    c. Subtract term * divisor from dividend
/// 2. The final dividend is the remainder
fn polynomial_long_division<F>(
    dividend: &Polynomial<F>,
    divisor: &Polynomial<F>,
) -> Result<(Polynomial<F>, Polynomial<F>), QapError>
where
    F: PrimeField,
{
    // Check for division by zero
    if divisor.is_zero() {
        return Err(QapError::DivisionByZero);
    }

    // If dividend is zero, return zero quotient and remainder
    if dividend.is_zero() {
        return Ok((Polynomial::<F>::new(vec![]), Polynomial::<F>::new(vec![])));
    }

    let mut remainder = dividend.clone();
    let mut quotient_coeffs = vec![FieldWrapper::<F>::zero(); dividend.degree() + 1];

    let divisor_degree = divisor.degree();

    // Get leading coefficient of divisor (last non-zero coefficient)
    let zero = FieldWrapper::<F>::zero();
    let divisor_leading = divisor
        .coeffs
        .iter()
        .rev()
        .find(|c| !c.value.is_zero())
        .unwrap_or(&zero);

    while remainder.degree() >= divisor_degree && !remainder.is_zero() {
        let remainder_degree = remainder.degree();

        // Get leading coefficient of remainder
        let remainder_leading = remainder
            .coeffs
            .iter()
            .rev()
            .find(|c| !c.value.is_zero())
            .unwrap_or(&zero);

        // Compute coefficient for this term
        let coeff = remainder_leading.clone()
            * FieldWrapper::<F>::from(
                divisor_leading
                    .value
                    .inverse()
                    .expect("Divisor leading coefficient should never be zero"),
            );

        // Degree difference
        let degree_diff = remainder_degree - divisor_degree;

        // Add term to quotient
        quotient_coeffs[degree_diff] = quotient_coeffs[degree_diff].clone() + coeff.clone();

        // Construct the term polynomial: coeff * x^degree_diff
        let mut term_coeffs = vec![FieldWrapper::<F>::zero(); degree_diff + 1];
        term_coeffs[degree_diff] = coeff;
        let term = Polynomial::<F>::new(term_coeffs);

        // Subtract term * divisor from remainder
        let product = term * divisor.clone();
        remainder = remainder - product;
    }

    // Remove trailing zero coefficients from quotient
    while quotient_coeffs.len() > 1 && quotient_coeffs.last().unwrap().value.is_zero() {
        quotient_coeffs.pop();
    }

    Ok((Polynomial::<F>::new(quotient_coeffs), remainder))
}

/// Constructs the target polynomial t(x) = ∏ᵢ₌₁ⁿ (x - i).
///
/// The target polynomial is the product of (x - i) for i = 1, 2, ..., n,
/// where n is the number of constraints. This polynomial has roots at
/// x = 1, 2, ..., n, which are the constraint indices.
///
/// # Arguments
/// * `num_constraints` - The number of constraints (n)
///
/// # Returns
/// * The target polynomial t(x) of degree n
///
/// # Example
/// ```rust,ignore
/// use groth16_qap::target_polynomial;
/// use ark_bn254::Fq;
///
/// // For 2 constraints: t(x) = (x - 1)(x - 2) = x² - 3x + 2
/// let t = target_polynomial::<Fq>(2);
/// assert_eq!(t.degree(), 2);
/// ```
pub fn target_polynomial<F>(num_constraints: usize) -> Polynomial<F>
where
    F: PrimeField,
{
    if num_constraints == 0 {
        return Polynomial::<F>::new(vec![]);
    }

    // Start with (x - 1)
    let mut result = Polynomial::<F>::new(vec![
        FieldWrapper::<F>::zero() - FieldWrapper::<F>::one(), // constant term: -1
        FieldWrapper::<F>::one(),                             // x term: 1
    ]);

    // Multiply by (x - i) for i = 2..num_constraints
    for i in 2..=num_constraints {
        let i_field = FieldWrapper::<F>::from(i as u64);
        let factor = Polynomial::<F>::new(vec![
            FieldWrapper::<F>::zero() - i_field, // constant term: -i
            FieldWrapper::<F>::one(),            // x term: 1
        ]);
        result = result * factor;
    }

    result
}

/// Scales a polynomial by a scalar value.
fn scale_polynomial<F>(poly: &Polynomial<F>, scalar: &FieldWrapper<F>) -> Polynomial<F>
where
    F: PrimeField,
{
    let scaled_coeffs: Vec<FieldWrapper<F>> = poly
        .coeffs
        .iter()
        .map(|c| c.clone() * scalar.clone())
        .collect();

    Polynomial::<F>::new(scaled_coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use ark_ff::Zero;

    #[test]
    fn test_target_polynomial_degree_1() {
        let t = target_polynomial::<Fq>(1);
        // t(x) = (x - 1) = -1 + x
        assert_eq!(t.degree(), 1);

        // t(1) should be 0
        let x1 = FieldWrapper::<Fq>::from(1u64);
        assert_eq!(t.evaluate(&x1).value, Fq::from(0u64));
    }

    #[test]
    fn test_target_polynomial_degree_2() {
        let t = target_polynomial::<Fq>(2);
        // t(x) = (x - 1)(x - 2) = 2 - 3x + x²
        assert_eq!(t.degree(), 2);

        // t(1) should be 0, t(2) should be 0
        let x1 = FieldWrapper::<Fq>::from(1u64);
        let x2 = FieldWrapper::<Fq>::from(2u64);
        assert_eq!(t.evaluate(&x1).value, Fq::from(0u64));
        assert_eq!(t.evaluate(&x2).value, Fq::from(0u64));
    }

    #[test]
    fn test_polynomial_division_exact() {
        // Divide x² - 1 by x - 1, should get x + 1 with remainder 0
        let dividend = Polynomial::<Fq> {
            coeffs: vec![
                FieldWrapper::<Fq>::zero() - FieldWrapper::<Fq>::one(), // constant term: -1
                FieldWrapper::<Fq>::zero(),                             // x term: 0
                FieldWrapper::<Fq>::one(),                              // x² term: 1
            ],
        };

        let divisor = Polynomial::<Fq> {
            coeffs: vec![
                FieldWrapper::<Fq>::zero() - FieldWrapper::<Fq>::one(), // (x - 1)
                FieldWrapper::<Fq>::one(),
            ],
        };

        let (quotient, remainder) = polynomial_long_division(&dividend, &divisor).unwrap();

        // Quotient should be x + 1
        assert_eq!(quotient.degree(), 1);
        // Remainder should be zero
        assert!(remainder.coeffs.is_empty() || remainder.coeffs.iter().all(|c| c.value.is_zero()));
    }

    #[test]
    fn test_polynomial_division_with_remainder() {
        // Divide x² + 1 by x - 1, should get x + 1 with remainder 2
        let dividend = Polynomial::<Fq> {
            coeffs: vec![
                FieldWrapper::<Fq>::one(),  // constant term: 1
                FieldWrapper::<Fq>::zero(), // x term: 0
                FieldWrapper::<Fq>::one(),  // x² term: 1
            ],
        };

        let divisor = Polynomial::<Fq> {
            coeffs: vec![
                FieldWrapper::<Fq>::zero() - FieldWrapper::<Fq>::one(), // (x - 1)
                FieldWrapper::<Fq>::one(),
            ],
        };

        let (quotient, remainder) = polynomial_long_division(&dividend, &divisor).unwrap();

        // Quotient should be x + 1
        assert_eq!(quotient.degree(), 1);
        // Remainder should be non-zero (2)
        assert!(!remainder.coeffs.is_empty());
    }

    #[test]
    fn test_division_by_zero() {
        let dividend = Polynomial::<Fq> {
            coeffs: vec![FieldWrapper::<Fq>::from(1u64)],
        };

        let divisor = Polynomial::<Fq> { coeffs: vec![] };

        let result = polynomial_long_division(&dividend, &divisor);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QapError::DivisionByZero));
    }

    #[test]
    fn test_check_divisibility_valid_witness() {
        // This test requires constructing a full QAP from an R1CS
        // For now, we test with simple polynomials
        // If a(x) = x, b(x) = x, c(x) = x², then p(x) = x² - x² = 0
        // 0 is divisible by any polynomial

        let a = Polynomial::<Fq> {
            coeffs: vec![
                FieldWrapper::<Fq>::from(0u64),
                FieldWrapper::<Fq>::from(1u64),
            ],
        };

        let b = a.clone();
        let c = Polynomial::<Fq> {
            coeffs: vec![
                FieldWrapper::<Fq>::from(0u64),
                FieldWrapper::<Fq>::from(0u64),
                FieldWrapper::<Fq>::from(1u64),
            ],
        };

        let witness = vec![FieldWrapper::<Fq>::from(1u64)];
        let a_polys = vec![a];
        let b_polys = vec![b];
        let c_polys = vec![c];

        let target = target_polynomial::<Fq>(1);

        let result = check_divisibility(&witness, &a_polys, &b_polys, &c_polys, &target);
        assert!(result.is_ok());
        // p(x) = 1*x * 1*x - 1*x² = 0, which is divisible by target
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_divisibility_mismatched_lengths() {
        let witness = vec![
            FieldWrapper::<Fq>::from(1u64),
            FieldWrapper::<Fq>::from(2u64),
        ];
        let a_polys = vec![Polynomial::<Fq> { coeffs: vec![] }];
        let b_polys = vec![Polynomial::<Fq> { coeffs: vec![] }];
        let c_polys = vec![Polynomial::<Fq> { coeffs: vec![] }];

        let target = target_polynomial::<Fq>(1);

        let result = check_divisibility(&witness, &a_polys, &b_polys, &c_polys, &target);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QapError::MismatchedLengths(_, _)
        ));
    }
}
