use ark_bn254::Fq;
use groth16_math::fields::FieldWrapper;
use groth16_r1cs::constraint::R1CSConstraint;

/// Multiplier circuit: a × b = c
///
/// This is the simplest non-trivial zero-knowledge circuit, demonstrating
/// that you know two numbers a and b whose product is c, without revealing a or b.
///
/// # Privacy
/// - **Private inputs**: a, b (the factors being multiplied)
/// - **Public output**: c (the product, revealed to verifier)
///
/// # Zero-Knowledge Property
/// The proof reveals that valid (a, b) exist for the public c,
/// but does NOT reveal which specific a and b were used.
/// For example, if c = 12, the prover could have used (a=2, b=6) or (a=3, b=4).
///
/// # R1CS Representation
/// Single constraint with 4 variables (1, a, b, c):
///
/// A = [0, 1, 0, 0]  // selects variable a
/// B = [0, 0, 1, 0]  // selects variable b
/// C = [0, 0, 0, 1]  // selects variable c
///
/// Verification: (0*1 + 1*a + 0*b + 0*c) * (0*1 + 0*a + 1*b + 0*c) = (0*1 + 0*a + 0*b + 1*c)
///             a * b = c
///
/// # Example
/// ```rust
/// use groth16_circuits::multiplier::MultiplierCircuit;
///
/// // Create circuit with a=3, b=4, c=12
/// let circuit = MultiplierCircuit::new(3, 4, 12);
///
/// // Get R1CS constraints
/// let constraints = circuit.to_r1cs();
///
/// // Generate witness assignment
/// let witness = circuit.witness();
///
/// // Verify witness satisfies constraints
/// assert!(constraints[0].is_satisfied(&witness));
/// ```
pub struct MultiplierCircuit {
    /// Private input a
    pub a: u64,
    /// Private input b
    pub b: u64,
    /// Public output c
    pub c: u64,
}

impl MultiplierCircuit {
    /// Creates a new multiplier circuit with the given inputs.
    ///
    /// # Arguments
    /// * `a` - First private input (factor)
    /// * `b` - Second private input (factor)
    /// * `c` - Public output (product)
    ///
    /// # Note
    /// This function does NOT verify that a × b = c.
    /// Use `verify()` to check the computation.
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }

    /// Converts the circuit to R1CS constraints.
    ///
    /// For the multiplier circuit, there is a single constraint:
    /// a × b = c
    ///
    /// # Returns
    /// A vector containing one R1CS constraint
    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraint = R1CSConstraint::<Fq>::new();

        // A vector: selects variable a (index 2, after 1 and c)
        constraint.add_a_variable(2, FieldWrapper::<Fq>::from(1u64));

        // B vector: selects variable b (index 3)
        constraint.add_b_variable(3, FieldWrapper::<Fq>::from(1u64));

        // C vector: selects variable c (index 1, the public output)
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64));

        vec![constraint]
    }

    /// Generates the witness assignment for this circuit instance.
    ///
    /// The witness is the assignment to all variables, following the standard
    /// Groth16 convention:
    /// [1, public_outputs..., private_inputs...]
    ///
    /// For the multiplier circuit: [1, c, a, b]
    /// where:
    /// - Index 0: constant 1
    /// - Index 1: public output c
    /// - Index 2: private input a
    /// - Index 3: private input b
    ///
    /// This ordering is important for the Groth16 IC computation, which assumes
    /// public inputs come first (after the constant).
    ///
    /// # Returns
    /// Vector of field elements representing the witness
    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.c), // public output c
            FieldWrapper::<Fq>::from(self.a), // private input a
            FieldWrapper::<Fq>::from(self.b), // private input b
        ]
    }

    /// Verifies that the circuit computation is correct (a × b = c).
    ///
    /// # Returns
    /// * `true` - If a × b = c
    /// * `false` - If a × b ≠ c
    pub fn verify(&self) -> bool {
        // Use field arithmetic to avoid overflow
        let a_field = Fq::from(self.a);
        let b_field = Fq::from(self.b);
        let c_field = Fq::from(self.c);

        a_field * b_field == c_field
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_valid() {
        let circuit = MultiplierCircuit::new(3, 4, 12);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_invalid() {
        let circuit = MultiplierCircuit::new(3, 4, 13); // 3*4 ≠ 13
        assert!(!circuit.verify());
    }

    #[test]
    fn test_to_r1cs() {
        let circuit = MultiplierCircuit::new(3, 4, 12);
        let constraints = circuit.to_r1cs();

        assert_eq!(constraints.len(), 1);

        let constraint = &constraints[0];
        assert_eq!(constraint.unique_variable_count(), 3); // a, b, c
    }

    #[test]
    fn test_witness() {
        let circuit = MultiplierCircuit::new(3, 4, 12);
        let witness = circuit.witness();

        assert_eq!(witness.len(), 4); // [1, c, a, b]
        assert_eq!(witness[0].value, Fq::from(1u64)); // constant 1
        assert_eq!(witness[1].value, Fq::from(12u64)); // public output c
        assert_eq!(witness[2].value, Fq::from(3u64)); // private input a
        assert_eq!(witness[3].value, Fq::from(4u64)); // private input b
    }

    #[test]
    fn test_r1cs_satisfied() {
        let circuit = MultiplierCircuit::new(3, 4, 12);
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // The single constraint should be satisfied
        assert!(constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_r1cs_not_satisfied() {
        // Create circuit with wrong result
        let circuit = MultiplierCircuit::new(3, 4, 13);
        let constraints = circuit.to_r1cs();

        // Witness with wrong c value (using standard ordering: [1, c, a, b])
        let witness = vec![
            FieldWrapper::<Fq>::from(1u64),  // constant 1
            FieldWrapper::<Fq>::from(13u64), // Wrong c!
            FieldWrapper::<Fq>::from(3u64),  // a
            FieldWrapper::<Fq>::from(4u64),  // b
        ];

        // The constraint should NOT be satisfied
        assert!(!constraints[0].is_satisfied(&witness));
    }
}
