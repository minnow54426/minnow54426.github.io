use crate::error::QapError;
use ark_ff::PrimeField;
use groth16_math::fields::FieldWrapper;
use groth16_math::polynomial::Polynomial;
use groth16_r1cs::constraint::R1CSConstraint;

/// Type alias for the triple of polynomial vectors returned by R1CS to QAP transformation
pub type QapPolynomials<F> = (Vec<Polynomial<F>>, Vec<Polynomial<F>>, Vec<Polynomial<F>>);

/// Transforms an R1CS constraint system into a Quadratic Arithmetic Program.
///
/// For each variable j (0..m), this creates three polynomials Aⱼ(x), Bⱼ(x), Cⱼ(x)
/// such that for any constraint i (1..n):
///   Aⱼ(i) = coefficient of variable j in A vector of constraint i
///   Bⱼ(i) = coefficient of variable j in B vector of constraint i
///   Cⱼ(i) = coefficient of variable j in C vector of constraint i
///
/// # Arguments
/// * `constraints` - Slice of R1CS constraints (must be non-empty)
/// * `num_variables` - Total number of variables in the system
///
/// # Returns
/// * `Ok((A, B, C))` - Three vectors of polynomials, each of length num_variables
/// * `Err(QapError::EmptyConstraints)` - If constraints slice is empty
/// * `Err(QapError::InsufficientConstraints)` - If fewer than 2 constraints
///
/// # Algorithm
/// 1. Extract coefficient values for each variable across all constraints
/// 2. For each variable j:
///    a. Collect points (1, A[1,j]), (2, A[2,j]), ..., (n, A[n,j])
///    b. Interpolate polynomial Aⱼ(x) through these points using Lagrange interpolation
///    c. Repeat for B and C
///
/// # Example
/// ```rust
/// use groth16_qap::polynomials::r1cs_to_qap;
/// use groth16_r1cs::constraint::R1CSConstraint;
/// use groth16_math::fields::FieldWrapper;
/// use ark_bn254::Fq;
///
/// // Create two R1CS constraints
/// let mut c1 = R1CSConstraint::<Fq>::new();
/// c1.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));  // x
/// c1.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));  // y
/// c1.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));  // z
///
/// let mut c2 = R1CSConstraint::<Fq>::new();
/// c2.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));  // x
/// c2.add_b_variable(0, FieldWrapper::<Fq>::from(1u64));  // x
/// c2.add_c_variable(3, FieldWrapper::<Fq>::from(1u64));  // w
///
/// let constraints = vec![c1, c2];
/// let num_vars = 4;
///
/// let (A, B, C) = r1cs_to_qap(&constraints, num_vars).unwrap();
/// // A[0] evaluates to 1 at x=1 and x=2 (coefficient for x in both constraints)
/// // B[1] evaluates to 1 at x=1, 0 at x=2 (coefficient for y only in first constraint)
/// // C[2] evaluates to 1 at x=1, 0 at x=2 (coefficient for z only in first constraint)
/// ```
pub fn r1cs_to_qap<F>(
    constraints: &[R1CSConstraint<F>],
    num_variables: usize,
) -> Result<QapPolynomials<F>, QapError>
where
    F: PrimeField,
{
    // Check for empty constraints
    if constraints.is_empty() {
        return Err(QapError::EmptyConstraints);
    }

    // Need at least 2 constraints for interpolation
    if constraints.len() < 2 {
        return Err(QapError::InsufficientConstraints);
    }

    let n = constraints.len();

    // For each variable j, collect points (i, coefficient) for i=1..n
    // and interpolate to get polynomial Aⱼ(x), Bⱼ(x), Cⱼ(x)

    let mut a_polys = Vec::with_capacity(num_variables);
    let mut b_polys = Vec::with_capacity(num_variables);
    let mut c_polys = Vec::with_capacity(num_variables);

    for j in 0..num_variables {
        // Collect points for variable j across all constraints
        let mut a_points = Vec::with_capacity(n);
        let mut b_points = Vec::with_capacity(n);
        let mut c_points = Vec::with_capacity(n);

        for (i, constraint) in constraints.iter().enumerate() {
            // Constraint indices are 1-based (i+1)
            let x = F::from((i + 1) as u64);

            // Get coefficient for variable j in A vector (default to 0 if not present)
            let a_coeff = constraint
                .a
                .get(&j)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);
            let b_coeff = constraint
                .b
                .get(&j)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);
            let c_coeff = constraint
                .c
                .get(&j)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);

            a_points.push((x, a_coeff));
            b_points.push((x, b_coeff));
            c_points.push((x, c_coeff));
        }

        // Interpolate polynomials for this variable
        let a_poly = lagrange_interpolate(&a_points)?;
        let b_poly = lagrange_interpolate(&b_points)?;
        let c_poly = lagrange_interpolate(&c_points)?;

        a_polys.push(a_poly);
        b_polys.push(b_poly);
        c_polys.push(c_poly);
    }

    Ok((a_polys, b_polys, c_polys))
}

/// Performs Lagrange interpolation to find a polynomial passing through given points.
///
/// Given points (x₁, y₁), (x₂, y₂), ..., (xₙ, yₙ) with distinct x-values,
/// returns the unique polynomial P(x) of degree ≤ n-1 such that P(xᵢ) = yᵢ for all i.
///
/// # Arguments
/// * `points` - Slice of (x, y) points, where:
///  - x is the constraint index (1-based)
///  - y is the coefficient value for that variable
///
/// # Returns
/// * `Ok(polynomial)` - The interpolated polynomial
/// * `Err(QapError::EmptyPoints)` - If points slice is empty
/// * `Err(QapError::DuplicateX)` - If x-values are not distinct
///
/// # Algorithm
/// Uses Lagrange basis polynomials:
/// P(x) = Σᵢ yᵢ · Lᵢ(x)
/// where Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ) / (xᵢ - xⱼ)
///
/// # Complexity
/// O(n²) where n is the number of points
pub fn lagrange_interpolate<F>(points: &[(F, FieldWrapper<F>)]) -> Result<Polynomial<F>, QapError>
where
    F: PrimeField,
{
    // Check for empty points
    if points.is_empty() {
        return Err(QapError::EmptyPoints);
    }

    let n = points.len();

    // Check for duplicate x-values
    for i in 0..n {
        for j in (i + 1)..n {
            if points[i].0 == points[j].0 {
                return Err(QapError::DuplicateX(format!(
                    "x-value {:?} appears at indices {} and {}",
                    points[i].0, i, j
                )));
            }
        }
    }

    // Lagrange interpolation: P(x) = Σᵢ yᵢ · Lᵢ(x)
    // where Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ) / (xᵢ - xⱼ)

    // Start with zero polynomial
    let mut result_coeffs = vec![FieldWrapper::<F>::zero(); n];

    for i in 0..n {
        let (xi, yi) = &points[i];

        // Compute Lagrange basis polynomial Lᵢ(x)
        // Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ) / (xᵢ - xⱼ)

        // Start with constant polynomial 1
        let mut li_coeffs = vec![FieldWrapper::<F>::one()];
        let mut denominator = FieldWrapper::<F>::one();

        for (j, xj) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let xj = xj.0;

            // Multiply by (x - xj)
            // This shifts coefficients and adds -xj term
            let mut new_coeffs = vec![FieldWrapper::<F>::zero(); li_coeffs.len() + 1];
            for (k, coeff) in li_coeffs.iter().enumerate() {
                // x * coeff
                new_coeffs[k + 1] = new_coeffs[k + 1].clone() + coeff.clone();
                // -xj * coeff
                let xj_field = FieldWrapper::<F>::from(xj);
                new_coeffs[k] = new_coeffs[k].clone() - coeff.clone() * xj_field;
            }
            li_coeffs = new_coeffs;

            // Multiply denominator by (xi - xj)
            let xi_field = FieldWrapper::<F>::from(*xi);
            let xj_field = FieldWrapper::<F>::from(xj);
            denominator = denominator * (xi_field - xj_field);
        }

        // Divide Lᵢ(x) by denominator
        // This is equivalent to multiplying by 1/denominator
        let inv_denominator = FieldWrapper::<F>::from(denominator.value.inverse().expect(
            "Denominator should never be zero in Lagrange interpolation (x-values are distinct)",
        ));

        // Scale Lᵢ(x) by yi / denominator
        let scalar = yi.clone() * inv_denominator;

        // Add scaled Lᵢ(x) to result
        for (k, coeff) in li_coeffs.iter().enumerate() {
            if k < result_coeffs.len() {
                result_coeffs[k] = result_coeffs[k].clone() + coeff.clone() * scalar.clone();
            }
        }
    }

    // Remove trailing zero coefficients
    while result_coeffs.len() > 1 && result_coeffs.last().unwrap().value.is_zero() {
        result_coeffs.pop();
    }

    Ok(Polynomial::new(result_coeffs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::fields::FieldWrapper;
    use groth16_r1cs::constraint::R1CSConstraint;

    #[test]
    fn test_single_constraint() {
        // Single constraint: x * y = z
        // A = [1, 0, 0], B = [0, 1, 0], C = [0, 0, 1]
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
        constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));

        let constraints = vec![constraint];
        let num_vars = 3;

        let result = r1cs_to_qap(&constraints, num_vars);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            QapError::InsufficientConstraints
        ));
    }

    #[test]
    fn test_two_constraints() {
        // Constraint 1: x * y = z  → A=[1,0,0], B=[0,1,0], C=[0,0,1]
        // Constraint 2: x * x = w  → A=[1,0,0], B=[1,0,0], C=[0,0,1]
        let mut c1 = R1CSConstraint::<Fq>::new();
        c1.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        c1.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
        c1.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));

        let mut c2 = R1CSConstraint::<Fq>::new();
        c2.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        c2.add_b_variable(0, FieldWrapper::<Fq>::from(1u64));
        c2.add_c_variable(3, FieldWrapper::<Fq>::from(1u64));

        let constraints = vec![c1, c2];
        let num_vars = 4;

        let (a, b, c) = r1cs_to_qap(&constraints, num_vars).unwrap();

        assert_eq!(a.len(), num_vars);
        assert_eq!(b.len(), num_vars);
        assert_eq!(c.len(), num_vars);

        // a[0] should interpolate: at x=1 → 1, at x=2 → 1
        // This is constant polynomial 1
        let x1 = FieldWrapper::<Fq>::from(1u64);
        let x2 = FieldWrapper::<Fq>::from(2u64);
        assert_eq!(a[0].evaluate(&x1).value, Fq::from(1u64));
        assert_eq!(a[0].evaluate(&x2).value, Fq::from(1u64));
    }

    #[test]
    fn test_lagrange_interpolate() {
        // Points: (1, 2), (2, 4), (3, 6)
        // This should give polynomial 2x (through origin)
        let points = vec![
            (Fq::from(1u64), FieldWrapper::<Fq>::from(2u64)),
            (Fq::from(2u64), FieldWrapper::<Fq>::from(4u64)),
            (Fq::from(3u64), FieldWrapper::<Fq>::from(6u64)),
        ];

        let poly = lagrange_interpolate(&points).unwrap();

        // Test at x=1, 2, 3
        let x1 = FieldWrapper::<Fq>::from(1u64);
        let x2 = FieldWrapper::<Fq>::from(2u64);
        let x3 = FieldWrapper::<Fq>::from(3u64);

        assert_eq!(poly.evaluate(&x1).value, Fq::from(2u64));
        assert_eq!(poly.evaluate(&x2).value, Fq::from(4u64));
        assert_eq!(poly.evaluate(&x3).value, Fq::from(6u64));
    }

    #[test]
    fn test_empty_constraints() {
        let constraints: Vec<R1CSConstraint<Fq>> = vec![];
        let result = r1cs_to_qap(&constraints, 3);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QapError::EmptyConstraints));
    }

    #[test]
    fn test_empty_points() {
        let points: Vec<(Fq, FieldWrapper<Fq>)> = vec![];
        let result = lagrange_interpolate(&points);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QapError::EmptyPoints));
    }
}
