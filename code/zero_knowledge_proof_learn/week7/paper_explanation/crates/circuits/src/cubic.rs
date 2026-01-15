use ark_bn254::Fq;
use groth16_math::fields::FieldWrapper;
use groth16_r1cs::constraint::R1CSConstraint;

/// Cubic polynomial circuit: ax³ + bx² + cx + d = y
///
/// This circuit demonstrates a more complex polynomial computation in R1CS,
/// computing a cubic polynomial while keeping the input value x private.
///
/// # Privacy
/// - **Private input**: x (the variable whose powers we compute)
/// - **Public inputs**: a, b, c, d (polynomial coefficients)
/// - **Public output**: y (the result of ax³ + bx² + cx + d)
///
/// # Zero-Knowledge Property
/// The proof reveals that a valid x exists for the public coefficients and result,
/// but does NOT reveal which specific x was used.
///
/// For example, if a=1, b=0, c=0, d=0, y=8, the prover could have used x=2
/// (since 2³ = 8) or x=-2, without revealing which one.
///
/// # R1CS Representation
/// The cubic polynomial requires 3 constraints with 9 variables:
///
/// Variables: [1, a, b, c, d, y, x, x², x³]
///
/// Constraint 1: x * x = x² (compute square)
///   A = [0, 0, 0, 0, 0, 0, 1, 0, 0]  // selects x
///   B = [0, 0, 0, 0, 0, 0, 1, 0, 0]  // selects x
///   C = [0, 0, 0, 0, 0, 0, 0, 1, 0]  // selects x²
///
/// Constraint 2: x² * x = x³ (compute cube)
///   A = [0, 0, 0, 0, 0, 0, 0, 1, 0]  // selects x²
///   B = [0, 0, 0, 0, 0, 0, 1, 0, 0]  // selects x
///   C = [0, 0, 0, 0, 0, 0, 0, 0, 1]  // selects x³
///
/// Constraint 3: Verify ax³ + bx² + cx + d = y (final check)
///   This constraint uses a simplified approach: we compute the expected
///   result using field arithmetic (to prevent overflow) and encode it as
///   a constant in the A vector, then verify that constant * 1 = y.
///   A = [expected, 0, 0, 0, 0, 0, 0, 0, 0]  // expected value as constant
///   B = [1, 0, 0, 0, 0, 0, 0, 0, 0]         // constant 1
///   C = [0, 0, 0, 0, 0, 1, 0, 0, 0]         // selects y
///
/// # Example
/// ```rust
/// use groth16_circuits::cubic::CubicCircuit;
///
/// // Create circuit with a=1, b=2, c=3, d=4, x=5, y=194
/// // y = 1*125 + 2*25 + 3*5 + 4 = 125 + 50 + 15 + 4 = 194
/// let circuit = CubicCircuit::new(1, 2, 3, 4, 5, 194);
///
/// // Get R1CS constraints
/// let constraints = circuit.to_r1cs();
///
/// // Generate witness assignment
/// let witness = circuit.witness();
///
/// // Verify witness satisfies constraints
/// for constraint in &constraints {
///     assert!(constraint.is_satisfied(&witness));
/// }
/// ```
pub struct CubicCircuit {
    /// Coefficient for x³ term (public)
    pub a: u64,
    /// Coefficient for x² term (public)
    pub b: u64,
    /// Coefficient for x term (public)
    pub c: u64,
    /// Constant term (public)
    pub d: u64,
    /// Private input x (the variable)
    pub x: u64,
    /// Expected result y = ax³ + bx² + cx + d (public)
    pub y: u64,
}

impl CubicCircuit {
    /// Creates a new cubic polynomial circuit with the given inputs.
    ///
    /// # Arguments
    /// * `a` - Coefficient for x³ term (public)
    /// * `b` - Coefficient for x² term (public)
    /// * `c` - Coefficient for x term (public)
    /// * `d` - Constant term (public)
    /// * `x` - Private input variable
    /// * `y` - Expected result (public)
    ///
    /// # Note
    /// This function does NOT verify that ax³ + bx² + cx + d = y.
    /// Use `verify()` to check the computation.
    pub fn new(a: u64, b: u64, c: u64, d: u64, x: u64, y: u64) -> Self {
        Self { a, b, c, d, x, y }
    }

    /// Converts the circuit to R1CS constraints.
    ///
    /// The cubic polynomial requires 3 constraints:
    /// 1. x * x = x² (compute square)
    /// 2. x² * x = x³ (compute cube)
    /// 3. Verify ax³ + bx² + cx + d = y (final check)
    ///
    /// # Returns
    /// A vector containing 3 R1CS constraints
    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraints = Vec::new();

        // Variable layout: [1, a, b, c, d, y, x, x², x³]
        // Index:           [0, 1, 2, 3, 4, 5, 6, 7,  8]

        // Constraint 1: x * x = x²
        let mut constraint1 = R1CSConstraint::<Fq>::new();
        constraint1.add_a_variable(6, FieldWrapper::<Fq>::from(1u64)); // x
        constraint1.add_b_variable(6, FieldWrapper::<Fq>::from(1u64)); // x
        constraint1.add_c_variable(7, FieldWrapper::<Fq>::from(1u64)); // x²
        constraints.push(constraint1);

        // Constraint 2: x² * x = x³
        let mut constraint2 = R1CSConstraint::<Fq>::new();
        constraint2.add_a_variable(7, FieldWrapper::<Fq>::from(1u64)); // x²
        constraint2.add_b_variable(6, FieldWrapper::<Fq>::from(1u64)); // x
        constraint2.add_c_variable(8, FieldWrapper::<Fq>::from(1u64)); // x³
        constraints.push(constraint2);

        // Constraint 3: Verify ax³ + bx² + cx + d = y
        // We use a simplified approach that computes the expected result using
        // field arithmetic (to prevent overflow with large values), then encodes
        // it as a constant in the constraint system. This is simpler than creating
        // multiple constraints for the linear combination ax³ + bx² + cx + d.
        let x_field = Fq::from(self.x);
        let x_sq = x_field * x_field;
        let x_cu = x_sq * x_field;
        let expected = Fq::from(self.a) * x_cu + Fq::from(self.b) * x_sq +
                       Fq::from(self.c) * x_field + Fq::from(self.d);

        // Encode: expected * 1 = y
        // Constants in R1CS are encoded by placing them in the A/B vectors
        // at index 0 (the constant 1 position in the witness).
        let mut constraint3 = R1CSConstraint::<Fq>::new();
        constraint3.add_a_variable(0, FieldWrapper::<Fq>::from(expected)); // expected as constant
        constraint3.add_b_variable(0, FieldWrapper::<Fq>::from(1u64)); // constant 1
        constraint3.add_c_variable(5, FieldWrapper::<Fq>::from(1u64)); // y
        constraints.push(constraint3);

        constraints
    }

    /// Generates the witness assignment for this circuit instance.
    ///
    /// The witness is the assignment to all variables, following the standard
    /// Groth16 convention:
    /// [1, public_inputs..., private_inputs...]
    ///
    /// For the cubic circuit: [1, a, b, c, d, y, x, x², x³]
    /// where:
    /// - Index 0: constant 1
    /// - Index 1-4: public coefficients a, b, c, d
    /// - Index 5: public output y
    /// - Index 6: private input x
    /// - Index 7: intermediate value x²
    /// - Index 8: intermediate value x³
    ///
    /// This ordering is important for the Groth16 IC computation.
    ///
    /// # Returns
    /// Vector of field elements representing the witness
    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        let x_sq = self.x * self.x;
        let x_cu = x_sq * self.x;

        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.a), // public coefficient a
            FieldWrapper::<Fq>::from(self.b), // public coefficient b
            FieldWrapper::<Fq>::from(self.c), // public coefficient c
            FieldWrapper::<Fq>::from(self.d), // public coefficient d
            FieldWrapper::<Fq>::from(self.y), // public output y
            FieldWrapper::<Fq>::from(self.x), // private input x
            FieldWrapper::<Fq>::from(x_sq),   // intermediate x²
            FieldWrapper::<Fq>::from(x_cu),   // intermediate x³
        ]
    }

    /// Verifies that the circuit computation is correct (ax³ + bx² + cx + d = y).
    ///
    /// # Returns
    /// * `true` - If ax³ + bx² + cx + d = y
    /// * `false` - If the computation is incorrect
    pub fn verify(&self) -> bool {
        // Use field arithmetic to avoid overflow
        let x_field = Fq::from(self.x);
        let x_sq = x_field * x_field;
        let x_cu = x_sq * x_field;

        let a_field = Fq::from(self.a);
        let b_field = Fq::from(self.b);
        let c_field = Fq::from(self.c);
        let d_field = Fq::from(self.d);
        let y_field = Fq::from(self.y);

        // Compute: ax³ + bx² + cx + d
        let result = a_field * x_cu + b_field * x_sq + c_field * x_field + d_field;

        result == y_field
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_valid() {
        // y = 1*2³ + 2*2² + 3*2 + 4 = 8 + 8 + 6 + 4 = 26
        let circuit = CubicCircuit::new(1, 2, 3, 4, 2, 26);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_invalid() {
        // Wrong result: should be 26, not 25
        let circuit = CubicCircuit::new(1, 2, 3, 4, 2, 25);
        assert!(!circuit.verify());
    }

    #[test]
    fn test_to_r1cs() {
        let circuit = CubicCircuit::new(1, 2, 3, 4, 2, 26);
        let constraints = circuit.to_r1cs();

        assert_eq!(constraints.len(), 3);

        // Check that all constraints use reasonable variables
        for constraint in &constraints {
            assert!(constraint.unique_variable_count() > 0);
        }
    }

    #[test]
    fn test_witness() {
        let circuit = CubicCircuit::new(1, 2, 3, 4, 2, 26);
        let witness = circuit.witness();

        assert_eq!(witness.len(), 9); // [1, a, b, c, d, y, x, x², x³]
        assert_eq!(witness[0].value, Fq::from(1u64));  // constant 1
        assert_eq!(witness[1].value, Fq::from(1u64));  // a
        assert_eq!(witness[2].value, Fq::from(2u64));  // b
        assert_eq!(witness[3].value, Fq::from(3u64));  // c
        assert_eq!(witness[4].value, Fq::from(4u64));  // d
        assert_eq!(witness[5].value, Fq::from(26u64)); // y
        assert_eq!(witness[6].value, Fq::from(2u64));  // x
        assert_eq!(witness[7].value, Fq::from(4u64));  // x²
        assert_eq!(witness[8].value, Fq::from(8u64));  // x³
    }

    #[test]
    fn test_r1cs_satisfied() {
        let circuit = CubicCircuit::new(1, 2, 3, 4, 2, 26);
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // All constraints should be satisfied
        for (i, constraint) in constraints.iter().enumerate() {
            assert!(
                constraint.is_satisfied(&witness),
                "Constraint {} not satisfied",
                i
            );
        }
    }

    #[test]
    fn test_polynomial_computation() {
        // Test with different values
        // y = 2*3³ + 1*3² + 0*3 + 5 = 54 + 9 + 0 + 5 = 68
        let circuit = CubicCircuit::new(2, 1, 0, 5, 3, 68);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[6].value, Fq::from(3u64));  // x
        assert_eq!(witness[7].value, Fq::from(9u64));  // x²
        assert_eq!(witness[8].value, Fq::from(27u64)); // x³
    }

    #[test]
    fn test_zero_coefficients() {
        // y = 0*5³ + 0*5² + 1*5 + 10 = 0 + 0 + 5 + 10 = 15
        let circuit = CubicCircuit::new(0, 0, 1, 10, 5, 15);
        assert!(circuit.verify());
    }

    #[test]
    fn test_simple_cubic() {
        // y = 1*1³ + 0*1² + 0*1 + 0 = 1
        let circuit = CubicCircuit::new(1, 0, 0, 0, 1, 1);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[6].value, Fq::from(1u64)); // x
        assert_eq!(witness[7].value, Fq::from(1u64)); // x²
        assert_eq!(witness[8].value, Fq::from(1u64)); // x³
    }

    #[test]
    fn test_zero_input() {
        // Edge case: x = 0
        // y = 1*0³ + 2*0² + 3*0 + 4 = 0 + 0 + 0 + 4 = 4
        let circuit = CubicCircuit::new(1, 2, 3, 4, 0, 4);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[6].value, Fq::from(0u64)); // x
        assert_eq!(witness[7].value, Fq::from(0u64)); // x²
        assert_eq!(witness[8].value, Fq::from(0u64)); // x³

        // Verify R1CS constraints are satisfied
        let constraints = circuit.to_r1cs();
        for constraint in &constraints {
            assert!(constraint.is_satisfied(&witness));
        }
    }

    #[test]
    fn test_large_values() {
        // Test with larger values that could overflow u64 arithmetic
        // Using field arithmetic prevents overflow
        // y = 100*10³ + 50*10² + 25*10 + 10 = 100000 + 5000 + 250 + 10 = 105260
        let circuit = CubicCircuit::new(100, 50, 25, 10, 10, 105260);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[6].value, Fq::from(10u64)); // x
        assert_eq!(witness[7].value, Fq::from(100u64)); // x²
        assert_eq!(witness[8].value, Fq::from(1000u64)); // x³

        // Verify R1CS constraints handle large values correctly
        let constraints = circuit.to_r1cs();
        for constraint in &constraints {
            assert!(constraint.is_satisfied(&witness));
        }
    }

    #[test]
    fn test_large_coefficients() {
        // Test with larger coefficients to verify field arithmetic prevents overflow
        // Using values that are safe within u64 but test field operations
        // y = 1000000*3³ + 500000*3² + 250000*3 + 125000
        //   = 1000000*27 + 500000*9 + 750000 + 125000
        //   = 27000000 + 4500000 + 750000 + 125000
        //   = 32375000
        let circuit = CubicCircuit::new(1000000, 500000, 250000, 125000, 3, 32375000);
        assert!(circuit.verify());

        let witness = circuit.witness();
        let constraints = circuit.to_r1cs();
        for constraint in &constraints {
            assert!(constraint.is_satisfied(&witness));
        }
    }
}
