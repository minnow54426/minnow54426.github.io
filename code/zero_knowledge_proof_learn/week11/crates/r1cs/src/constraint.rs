use ark_ff::PrimeField;
use groth16_math::fields::FieldWrapper;
use std::collections::HashMap;

/// Represents a single Rank-1 Constraint System (R1CS) constraint.
///
/// An R1CS constraint has the form: <a, x> · <b, x> = <c, x>
///
/// where:
/// - a, b, c are vectors of coefficients (sparse, represented as HashMaps)
/// - x is the assignment vector (witness)
/// - <·, ·> denotes the dot product
/// - · denotes scalar multiplication
///
/// # Sparse Representation
/// We use HashMap<usize, FieldWrapper<F>> to represent each vector sparsely.
/// Only non-zero coefficients are stored, making this efficient for large
/// constraint systems where most variables don't appear in most constraints.
///
/// # Variable Indexing
/// Variables are 0-indexed. Index 0 typically represents the constant ONE
/// in Groth16, while indices 1+ represent actual variables.
///
/// # Example
/// ```
/// use groth16_r1cs::constraint::R1CSConstraint;
/// use groth16_math::fields::FieldWrapper;
/// use ark_bn254::Fq;
///
/// // Constraint: a * b = c where a=var[0], b=var[1], c=var[2]
/// let mut constraint = R1CSConstraint::<Fq>::new();
/// constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
/// constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
/// constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));
/// ```
#[derive(Clone)]
pub struct R1CSConstraint<F: PrimeField> {
    /// Coefficients for the A vector (left input of multiplication)
    pub a: HashMap<usize, FieldWrapper<F>>,
    /// Coefficients for the B vector (right input of multiplication)
    pub b: HashMap<usize, FieldWrapper<F>>,
    /// Coefficients for the C vector (output of multiplication)
    pub c: HashMap<usize, FieldWrapper<F>>,
}

impl<F: PrimeField> Default for R1CSConstraint<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: PrimeField> R1CSConstraint<F> {
    /// Creates a new empty R1CS constraint.
    pub fn new() -> Self {
        Self {
            a: HashMap::new(),
            b: HashMap::new(),
            c: HashMap::new(),
        }
    }

    /// Adds a variable with coefficient to the A vector.
    pub fn add_a_variable(&mut self, index: usize, coeff: FieldWrapper<F>) {
        self.a.insert(index, coeff);
    }

    /// Adds a variable with coefficient to the B vector.
    pub fn add_b_variable(&mut self, index: usize, coeff: FieldWrapper<F>) {
        self.b.insert(index, coeff);
    }

    /// Adds a variable with coefficient to the C vector.
    pub fn add_c_variable(&mut self, index: usize, coeff: FieldWrapper<F>) {
        self.c.insert(index, coeff);
    }

    /// Returns the count of unique variable indices used in this constraint.
    ///
    /// This is a per-constraint count only. For the full R1CS system,
    /// you need to find the maximum variable index across ALL constraints.
    ///
    /// # Example
    /// If this constraint uses variables at indices [0, 1, 2],
    /// returns 3.
    pub fn unique_variable_count(&self) -> usize {
        let all_indices: std::collections::HashSet<_> = self
            .a
            .keys()
            .chain(self.b.keys())
            .chain(self.c.keys())
            .collect();
        all_indices.len()
    }

    /// Checks if a witness satisfies this constraint.
    ///
    /// A witness satisfies the constraint if: <a, witness> · <b, witness> = <c, witness>
    ///
    /// # Arguments
    /// * `witness` - The assignment vector to check
    ///
    /// # Returns
    /// * `true` if the witness satisfies the constraint
    /// * `false` otherwise
    ///
    /// # Example
    /// ```
    /// use groth16_r1cs::constraint::R1CSConstraint;
    /// use groth16_math::fields::FieldWrapper;
    /// use ark_bn254::Fq;
    ///
    /// // Constraint: a * b = c
    /// let mut constraint = R1CSConstraint::<Fq>::new();
    /// constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
    /// constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
    /// constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));
    ///
    /// let witness = vec![
    ///     FieldWrapper::<Fq>::from(3u64),  // a = 3
    ///     FieldWrapper::<Fq>::from(4u64),  // b = 4
    ///     FieldWrapper::<Fq>::from(12u64), // c = 12 (3 * 4 = 12 ✓)
    /// ];
    ///
    /// assert!(constraint.is_satisfied(&witness));
    /// ```
    pub fn is_satisfied(&self, witness: &[FieldWrapper<F>]) -> bool {
        let a_value = self.evaluate_linear_combination(&self.a, witness);
        let b_value = self.evaluate_linear_combination(&self.b, witness);
        let c_value = self.evaluate_linear_combination(&self.c, witness);

        // Check: a · b = c
        let product = a_value.clone() * b_value;
        product.value == c_value.value
    }

    /// Evaluates a linear combination of witness values.
    ///
    /// Computes: sum(coeffs[i] * witness[i])
    fn evaluate_linear_combination(
        &self,
        coeffs: &HashMap<usize, FieldWrapper<F>>,
        witness: &[FieldWrapper<F>],
    ) -> FieldWrapper<F> {
        let mut result = FieldWrapper::zero();
        for (idx, coeff) in coeffs {
            if let Some(w) = witness.get(*idx) {
                result = result + coeff.clone() * w.clone();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::fields::FieldWrapper;

    #[test]
    fn test_r1cs_constraint_creation() {
        // Simple constraint: a * b = c
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64)); // a
        constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64)); // b
        constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64)); // c

        assert_eq!(constraint.unique_variable_count(), 3);
    }

    #[test]
    fn test_empty_constraint() {
        let constraint = R1CSConstraint::<Fq>::new();
        assert_eq!(constraint.unique_variable_count(), 0);
    }

    #[test]
    fn test_same_variable_multiple_vectors() {
        // x * x = x^2 (same variable in a and b)
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        constraint.add_b_variable(0, FieldWrapper::<Fq>::from(1u64)); // Same index
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64));

        // Should count as 2 unique variables (0 and 1)
        assert_eq!(constraint.unique_variable_count(), 2);
    }

    #[test]
    fn test_default_implementation() {
        let constraint = R1CSConstraint::<Fq>::default();
        assert_eq!(constraint.unique_variable_count(), 0);
        assert!(constraint.a.is_empty());
        assert!(constraint.b.is_empty());
        assert!(constraint.c.is_empty());
    }
}
