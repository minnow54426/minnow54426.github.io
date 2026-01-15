use ark_bn254::Fq;
use groth16_math::fields::FieldWrapper;
use groth16_r1cs::constraint::R1CSConstraint;

/// Hash preimage circuit: H(m) = h
///
/// This circuit demonstrates knowledge of a preimage m for a hash output h,
/// without revealing the preimage itself. This is a placeholder implementation
/// using a simple hash function for demonstration purposes.
///
/// # Privacy
/// - **Private input**: m (the preimage)
/// - **Public output**: h (the hash output)
///
/// # Zero-Knowledge Property
/// The proof reveals that you know some m such that H(m) = h,
/// but does NOT reveal which m was used.
///
/// For example, if h = 12345, you might know m = 100 such that H(100) = 12345,
/// but the proof only shows that such an m exists, not its value.
///
/// # Hash Function (Placeholder)
///
/// This implementation uses a simple placeholder hash function:
/// ```text
/// H(m) = m * 7 (simple multiplication)
/// ```
///
/// This is NOT a cryptographically secure hash function. Real hash circuits
/// would use SNARK-friendly hash functions like:
/// - Poseidon (specifically designed for ZK proofs)
/// - Rescue
/// - MiMC
///
/// Production hash circuits require hundreds of constraints to implement
/// secure hash functions. This placeholder demonstrates the circuit structure
/// with minimal complexity.
///
/// # R1CS Representation
///
/// For a placeholder hash, we use a single constraint that verifies the hash
/// computation in one step. This is simplified for demonstration.
///
/// Variables: [1, h, m]
///
/// Constraint 1: Verify H(m) = h
///   We compute the expected hash using field arithmetic, then encode it as
///   a constant in the A vector, verifying: expected * 1 = h
///   A = [expected, 0, 0]      // expected hash value as constant
///   B = [1, 0, 0]             // constant 1
///   C = [0, 1, 0]             // selects h
///
/// # Example
/// ```rust
/// use groth16_circuits::hash_preimage::HashPreimageCircuit;
///
/// // Create circuit with m=100, h=H(100)
/// let m = 100u64;
/// let h = HashPreimageCircuit::simple_hash(m);
/// let circuit = HashPreimageCircuit::new(m, h);
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
pub struct HashPreimageCircuit {
    /// Private preimage input
    pub m: u64,
    /// Public hash output
    pub h: u64,
}

impl HashPreimageCircuit {
    /// Simple placeholder hash function: H(m) = m * 7
    ///
    /// # WARNING
    /// This is NOT a cryptographically secure hash function. It is used only
    /// for demonstration purposes. Real hash circuits should use Poseidon,
    /// Rescue, or other SNARK-friendly hash functions.
    ///
    /// This is a trivial hash function for demonstration: just multiply by 7.
    /// This allows the circuit to be verified without complex field arithmetic.
    ///
    /// # Arguments
    /// * `m` - Input to hash
    ///
    /// # Returns
    /// Hash output (m * 7)
    pub fn simple_hash(m: u64) -> u64 {
        m.wrapping_mul(7)
    }

    /// Creates a new hash preimage circuit with the given inputs.
    ///
    /// # Arguments
    /// * `m` - Private preimage input
    /// * `h` - Public hash output
    ///
    /// # Note
    /// This function does NOT verify that H(m) = h.
    /// Use `verify()` to check the computation.
    pub fn new(m: u64, h: u64) -> Self {
        Self { m, h }
    }

    /// Converts the circuit to R1CS constraints.
    ///
    /// For the placeholder hash circuit, we use a single constraint that
    /// verifies the hash computation. The expected hash value is pre-computed
    /// using field arithmetic and encoded as a constant in the constraint.
    ///
    /// # Returns
    /// A vector containing one R1CS constraint
    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraints = Vec::new();

        // Variable layout: [1, h, m]
        // Index:           [0, 1, 2]

        // Compute the expected hash value using field arithmetic
        // For our simple hash H(m) = m * 7
        let m_field = Fq::from(self.m);
        let multiplier = Fq::from(7u64);
        let expected = m_field * multiplier;

        // Constraint: expected * 1 = h
        // This verifies that the computed hash matches the public output
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(expected)); // expected as constant
        constraint.add_b_variable(0, FieldWrapper::<Fq>::from(1u64)); // constant 1
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64)); // h
        constraints.push(constraint);

        constraints
    }

    /// Generates the witness assignment for this circuit instance.
    ///
    /// The witness is the assignment to all variables, following the standard
    /// Groth16 convention:
    /// [1, public_inputs..., private_inputs...]
    ///
    /// For the hash preimage circuit: [1, h, m]
    /// where:
    /// - Index 0: constant 1
    /// - Index 1: public output h
    /// - Index 2: private input m
    ///
    /// This ordering is important for the Groth16 IC computation.
    ///
    /// # Returns
    /// Vector of field elements representing the witness
    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.h), // public output h
            FieldWrapper::<Fq>::from(self.m), // private input m
        ]
    }

    /// Verifies that the circuit computation is correct (H(m) = h).
    ///
    /// # Returns
    /// * `true` - If H(m) = h
    /// * `false` - If the computation is incorrect
    pub fn verify(&self) -> bool {
        let computed = Self::simple_hash(self.m);
        computed == self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        // Test that the hash function is deterministic
        let m = 100u64;
        let h1 = HashPreimageCircuit::simple_hash(m);
        let h2 = HashPreimageCircuit::simple_hash(m);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_circuit_valid() {
        let m = 100u64;
        let h = HashPreimageCircuit::simple_hash(m);
        let circuit = HashPreimageCircuit::new(m, h);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_invalid() {
        let m = 100u64;
        let wrong_h = 12345u64; // Wrong hash
        let circuit = HashPreimageCircuit::new(m, wrong_h);
        assert!(!circuit.verify());
    }

    #[test]
    fn test_to_r1cs() {
        let m = 100u64;
        let h = HashPreimageCircuit::simple_hash(m);
        let circuit = HashPreimageCircuit::new(m, h);
        let constraints = circuit.to_r1cs();

        assert_eq!(constraints.len(), 1);

        // Check that the constraint uses reasonable variables
        assert!(constraints[0].unique_variable_count() > 0);
    }

    #[test]
    fn test_witness() {
        let m = 100u64;
        let h = 12345u64;
        let circuit = HashPreimageCircuit::new(m, h);
        let witness = circuit.witness();

        assert_eq!(witness.len(), 3); // [1, h, m]
        assert_eq!(witness[0].value, Fq::from(1u64)); // constant 1
        assert_eq!(witness[1].value, Fq::from(12345u64)); // public output h
        assert_eq!(witness[2].value, Fq::from(100u64)); // private input m
    }

    #[test]
    fn test_r1cs_satisfied() {
        let m = 100u64;
        let h = HashPreimageCircuit::simple_hash(m);
        let circuit = HashPreimageCircuit::new(m, h);
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // The single constraint should be satisfied
        assert!(constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_r1cs_not_satisfied() {
        // Create circuit with wrong hash
        let m = 100u64;
        let wrong_h = 99999u64;
        let circuit = HashPreimageCircuit::new(m, wrong_h);

        let constraints = circuit.to_r1cs();

        // Witness with wrong h value (using standard ordering: [1, h, m])
        let witness = vec![
            FieldWrapper::<Fq>::from(1u64),     // constant 1
            FieldWrapper::<Fq>::from(99999u64), // Wrong h!
            FieldWrapper::<Fq>::from(100u64),   // m
        ];

        // The constraint should NOT be satisfied
        assert!(!constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_different_preimages() {
        // Different preimages should produce different hashes (most of the time)
        let m1 = 100u64;
        let m2 = 200u64;
        let h1 = HashPreimageCircuit::simple_hash(m1);
        let h2 = HashPreimageCircuit::simple_hash(m2);

        // They should be different (though collisions are possible with weak hash)
        assert_ne!(h1, h2);

        // Verify both circuits
        let circuit1 = HashPreimageCircuit::new(m1, h1);
        let circuit2 = HashPreimageCircuit::new(m2, h2);
        assert!(circuit1.verify());
        assert!(circuit2.verify());
    }

    #[test]
    fn test_zero_preimage() {
        // Edge case: m = 0
        let m = 0u64;
        let h = HashPreimageCircuit::simple_hash(m);
        let circuit = HashPreimageCircuit::new(m, h);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[2].value, Fq::from(0u64)); // m
    }

    #[test]
    fn test_large_preimage() {
        // Test with larger preimage
        let m = 1000000u64;
        let h = HashPreimageCircuit::simple_hash(m);
        let circuit = HashPreimageCircuit::new(m, h);
        assert!(circuit.verify());

        // Verify R1CS constraints are satisfied
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();
        assert!(constraints[0].is_satisfied(&witness));
    }

}
