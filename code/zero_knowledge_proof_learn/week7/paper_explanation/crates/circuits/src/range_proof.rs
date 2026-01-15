use ark_bn254::Fq;
use groth16_math::fields::FieldWrapper;
use groth16_r1cs::constraint::R1CSConstraint;

/// Range proof circuit: prove value >= threshold without revealing the value
///
/// This circuit demonstrates that a private value is greater than or equal to a
/// public threshold, without revealing the actual value. This is a PLACEHOLDER
/// implementation for demonstration purposes.
///
/// # Privacy
/// - **Private input**: value (the secret value we're proving about)
/// - **Public input**: threshold (the minimum value)
/// - **Public output**: result (1 if value >= threshold, 0 otherwise)
///
/// # Zero-Knowledge Property
/// The proof reveals that you know a value satisfying value >= threshold,
/// but does NOT reveal the actual value.
///
/// For example, if threshold = 18, you might prove you're 18 or older
/// without revealing your exact age. The proof shows age >= 18,
/// but not whether you're 18, 25, 50, etc.
///
/// # Range Proof Applications
///
/// Range proofs are useful for:
/// - **Age verification**: Prove you're >= 18 without revealing exact age
/// - **Income verification**: Prove income >= threshold without revealing salary
/// - **Balance checks**: Prove sufficient funds without revealing account balance
/// - **Bounding values**: Prove a value is in [0, 2^n) range
///
/// # Real Range Proofs
///
/// Production range proofs require:
///
/// 1. **Bit Decomposition**: Break value into binary representation
///    ```text
///    value = Σ bit[i] * 2^i
///    ```
///
/// 2. **Bit Constraints**: Ensure each bit is 0 or 1
///    ```text
///    bit[i] * (1 - bit[i]) = 0  for each i
///    ```
///
/// 3. **Comparison Constraints**: Compare decomposed value with threshold
///    - This requires arithmetic circuit tricks for comparison
///    - Typically use "is less than" circuits with carry propagation
///
/// A full n-bit range proof requires O(n) constraints.
/// For 64-bit values, this is ~200+ constraints.
///
/// # Placeholder Implementation
///
/// This is a PLACEHOLDER circuit that demonstrates the structure without
/// implementing a full range proof. For this demo, we use a simple
/// comparison constraint:
///
/// ```text
/// verify: (value - threshold) * indicator = result
/// ```
///
/// Where:
/// - `indicator` is 1 if value >= threshold, 0 otherwise
/// - `result` is the public output
///
/// This is NOT a secure range proof! Real implementations must use
/// bit decomposition and proper comparison circuits.
///
/// # Simplified Placeholder
///
/// For this demo, we use a trivial verification:
/// ```text
/// check: value >= threshold
/// result: 1 if true, 0 if false
/// constraint: (value - threshold + 1) * 1 = result_adjusted
/// ```
///
/// Where `result_adjusted` accounts for field arithmetic to create a
/// meaningful constraint for demonstration.
///
/// # R1CS Representation
///
/// For the placeholder, we use a single constraint:
///
/// Variables: [1, result, threshold, value]
///
/// Constraint 1: Verify range condition
///   We compute an expected result using field arithmetic, then encode it as
///   a constant in the A vector, verifying: expected * 1 = result
///   A = [expected, 0, 0, 0]       // expected result as constant
///   B = [1, 0, 0, 0]              // constant 1
///   C = [0, 1, 0, 0]              // selects result
///
/// # Example
/// ```rust
/// use groth16_circuits::range_proof::RangeProofCircuit;
///
/// // Create circuit proving age >= 18
/// let age = 25u64;  // private
/// let threshold = 18u64;  // public
/// let result = RangeProofCircuit::check_range(age, threshold);
/// let circuit = RangeProofCircuit::new(age, threshold, result);
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
pub struct RangeProofCircuit {
    /// Private value to prove about
    pub value: u64,
    /// Public threshold
    pub threshold: u64,
    /// Public result (1 if value >= threshold, 0 otherwise)
    pub result: u64,
}

impl RangeProofCircuit {
    /// Simple range check: returns 1 if value >= threshold, 0 otherwise
    ///
    /// # WARNING
    /// This is NOT a secure range proof! Real range proofs require:
    /// 1. Bit decomposition of the value
    /// 2. Bit constraints to ensure bits are 0 or 1
    /// 3. Comparison circuit using arithmetic operations
    ///
    /// This is a trivial placeholder for demonstration only.
    ///
    /// A real range proof would:
    /// 1. Decompose value into bits: value = Σ bit[i] * 2^i
    /// 2. Add constraints: bit[i] * (1 - bit[i]) = 0 for each bit
    /// 3. Implement comparison circuit using the decomposed bits
    /// 4. Use O(n) constraints for n-bit values
    ///
    /// # Arguments
    /// * `value` - The value to check
    /// * `threshold` - The minimum value
    ///
    /// # Returns
    /// 1 if value >= threshold, 0 otherwise
    pub fn check_range(value: u64, threshold: u64) -> u64 {
        if value >= threshold {
            1
        } else {
            0
        }
    }

    /// Creates a new range proof circuit with the given inputs.
    ///
    /// # Arguments
    /// * `value` - Private value to prove about
    /// * `threshold` - Public threshold
    /// * `result` - Public result (1 if value >= threshold, 0 otherwise)
    ///
    /// # Note
    /// This function does NOT verify that the result is correct.
    /// Use `verify()` to check the computation.
    pub fn new(value: u64, threshold: u64, result: u64) -> Self {
        Self {
            value,
            threshold,
            result,
        }
    }

    /// Converts the circuit to R1CS constraints.
    ///
    /// For the placeholder range proof circuit, we use a single constraint that
    /// verifies the range check. The expected result is pre-computed and encoded
    /// as a constant in the constraint.
    ///
    /// Real range proof circuits would require:
    /// - Bit decomposition constraints (n constraints for n-bit value)
    /// - Bit validity constraints (n constraints: bit[i] * (1 - bit[i]) = 0)
    /// - Comparison constraints (depends on comparison method)
    ///
    /// For 64-bit values, this would be ~200+ constraints.
    ///
    /// # Returns
    /// A vector containing one R1CS constraint (placeholder)
    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraints = Vec::new();

        // Variable layout: [1, result, threshold, value]
        // Index:           [0, 1,      2,         3]

        // Compute the expected result using the range check
        // For our simple check: result = 1 if value >= threshold, else 0
        let expected = Self::check_range(self.value, self.threshold);

        // Constraint: expected * 1 = result
        // This verifies that the computed result matches the public output
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(Fq::from(expected))); // expected as constant
        constraint.add_b_variable(0, FieldWrapper::<Fq>::from(1u64)); // constant 1
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64)); // result
        constraints.push(constraint);

        constraints
    }

    /// Generates the witness assignment for this circuit instance.
    ///
    /// The witness is the assignment to all variables, following the standard
    /// Groth16 convention:
    /// [1, public_inputs..., private_inputs...]
    ///
    /// For the range proof circuit: [1, result, threshold, value]
    /// where:
    /// - Index 0: constant 1
    /// - Index 1: public output result
    /// - Index 2: public input threshold
    /// - Index 3: private input value
    ///
    /// This ordering is important for the Groth16 IC computation.
    ///
    /// # Returns
    /// Vector of field elements representing the witness
    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.result), // public output result
            FieldWrapper::<Fq>::from(self.threshold), // public input threshold
            FieldWrapper::<Fq>::from(self.value), // private input value
        ]
    }

    /// Verifies that the circuit computation is correct (result = check_range(value, threshold)).
    ///
    /// # Returns
    /// * `true` - If the result is correct
    /// * `false` - If the computation is incorrect
    pub fn verify(&self) -> bool {
        let computed = Self::check_range(self.value, self.threshold);
        computed == self.result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_range_above_threshold() {
        // Test value above threshold
        let value = 25u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_check_range_equal_threshold() {
        // Test value equal to threshold
        let value = 18u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_check_range_below_threshold() {
        // Test value below threshold
        let value = 15u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_circuit_valid_above() {
        let value = 25u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_valid_equal() {
        let value = 18u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_valid_below() {
        let value = 15u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_invalid() {
        let value = 15u64;
        let threshold = 18u64;
        let wrong_result = 1u64; // Should be 0, not 1
        let circuit = RangeProofCircuit::new(value, threshold, wrong_result);
        assert!(!circuit.verify());
    }

    #[test]
    fn test_to_r1cs() {
        let value = 25u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        let constraints = circuit.to_r1cs();

        assert_eq!(constraints.len(), 1);

        // Check that the constraint uses reasonable variables
        assert!(constraints[0].unique_variable_count() > 0);
    }

    #[test]
    fn test_witness() {
        let value = 25u64;
        let threshold = 18u64;
        let result = 1u64; // value >= threshold
        let circuit = RangeProofCircuit::new(value, threshold, result);
        let witness = circuit.witness();

        assert_eq!(witness.len(), 4); // [1, result, threshold, value]
        assert_eq!(witness[0].value, Fq::from(1u64)); // constant 1
        assert_eq!(witness[1].value, Fq::from(1u64)); // public output result
        assert_eq!(witness[2].value, Fq::from(18u64)); // public input threshold
        assert_eq!(witness[3].value, Fq::from(25u64)); // private input value
    }

    #[test]
    fn test_r1cs_satisfied() {
        let value = 25u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // The single constraint should be satisfied
        assert!(constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_r1cs_not_satisfied() {
        // Create circuit with wrong result
        let value = 15u64;
        let threshold = 18u64;
        let wrong_result = 1u64; // Should be 0
        let circuit = RangeProofCircuit::new(value, threshold, wrong_result);

        let constraints = circuit.to_r1cs();

        // Witness with wrong result value (using standard ordering: [1, result, threshold, value])
        let witness = vec![
            FieldWrapper::<Fq>::from(1u64),  // constant 1
            FieldWrapper::<Fq>::from(1u64),  // Wrong result!
            FieldWrapper::<Fq>::from(18u64), // threshold
            FieldWrapper::<Fq>::from(15u64), // value
        ];

        // The constraint should NOT be satisfied
        assert!(!constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_different_thresholds() {
        // Test different threshold values
        let value = 50u64;

        let threshold1 = 18u64;
        let result1 = RangeProofCircuit::check_range(value, threshold1);
        let circuit1 = RangeProofCircuit::new(value, threshold1, result1);
        assert!(circuit1.verify());
        assert_eq!(result1, 1); // 50 >= 18

        let threshold2 = 100u64;
        let result2 = RangeProofCircuit::check_range(value, threshold2);
        let circuit2 = RangeProofCircuit::new(value, threshold2, result2);
        assert!(circuit2.verify());
        assert_eq!(result2, 0); // 50 < 100
    }

    #[test]
    fn test_zero_value() {
        // Edge case: value = 0
        let value = 0u64;
        let threshold = 18u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[3].value, Fq::from(0u64)); // value
        assert_eq!(result, 0); // 0 < 18
    }

    #[test]
    fn test_zero_threshold() {
        // Edge case: threshold = 0
        let value = 25u64;
        let threshold = 0u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[2].value, Fq::from(0u64)); // threshold
        assert_eq!(result, 1); // 25 >= 0
    }

    #[test]
    fn test_large_values() {
        // Test with larger values
        let value = 1000000u64;
        let threshold = 500000u64;
        let result = RangeProofCircuit::check_range(value, threshold);
        let circuit = RangeProofCircuit::new(value, threshold, result);
        assert!(circuit.verify());

        // Verify R1CS constraints are satisfied
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();
        assert!(constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_age_verification_scenario() {
        // Practical scenario: age verification for voting
        // Prove you're 18 or older without revealing exact age

        let age = 25u64; // Private: actual age
        let voting_age = 18u64; // Public: minimum voting age
        let can_vote = RangeProofCircuit::check_range(age, voting_age);

        let circuit = RangeProofCircuit::new(age, voting_age, can_vote);

        // Verify the proof
        assert!(circuit.verify());
        assert_eq!(can_vote, 1); // Can vote

        // The witness shows: [1, can_vote=1, voting_age=18, age=25]
        // But in a real ZK proof, age would remain hidden!
        let witness = circuit.witness();
        assert_eq!(witness[1].value, Fq::from(1u64)); // can_vote
    }

    #[test]
    fn test_balance_check_scenario() {
        // Practical scenario: prove sufficient funds without revealing balance
        // You need to prove you have at least 1000 tokens

        let balance = 5000u64; // Private: actual balance
        let required = 1000u64; // Public: required amount
        let has_funds = RangeProofCircuit::check_range(balance, required);

        let circuit = RangeProofCircuit::new(balance, required, has_funds);

        // Verify the proof
        assert!(circuit.verify());
        assert_eq!(has_funds, 1); // Has sufficient funds
    }

    #[test]
    fn test_placeholder_nature() {
        // This test documents the placeholder nature
        let value = 25u64;
        let threshold = 18u64;
        let result = 1u64;
        let circuit = RangeProofCircuit::new(value, threshold, result);

        // The circuit should verify
        assert!(circuit.verify());

        // But this is NOT a real range proof!
        // Real range proofs require:
        // - Bit decomposition: value = Σ bit[i] * 2^i
        // - Bit validity: bit[i] * (1 - bit[i]) = 0 for each i
        // - Comparison circuit using arithmetic operations
        // - O(n) constraints for n-bit values (~200+ for 64-bit)
    }
}
