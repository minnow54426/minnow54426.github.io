use ark_bn254::Fq;
use groth16_math::fields::FieldWrapper;
use groth16_r1cs::constraint::R1CSConstraint;

/// Merkle tree membership circuit: prove leaf is in tree
///
/// This circuit demonstrates knowledge of a valid Merkle path from a leaf to the root,
/// without revealing which leaf or the path itself. This is a PLACEHOLDER implementation
/// for demonstration purposes.
///
/// # Privacy
/// - **Private inputs**: leaf value, authentication path, path indices
/// - **Public output**: Merkle root
///
/// # Zero-Knowledge Property
/// The proof reveals that you know a valid path from some leaf to the root,
/// but does NOT reveal which leaf or what the path is.
///
/// For example, if the root is 12345, you might know that leaf 100 with path
/// [200, 300, 400] leads to this root, but the proof only shows that such a
/// valid path exists, not the leaf value or the path itself.
///
/// # Merkle Tree Background
///
/// A Merkle tree is a binary tree where:
/// - Leaves are data blocks (transactions, values, etc.)
/// - Each non-leaf node is the hash of its two children
/// - The root commits to all leaves in the tree
/// - An authentication path is the minimal information needed to verify a leaf
///
/// To verify a leaf is in the tree, you need:
/// 1. The leaf value
/// 2. The sibling hashes at each level (authentication path)
/// 3. The path indices (left/right) at each level
///
/// You hash the leaf with its sibling to get the parent, then repeat up the tree.
/// If the computed root matches the expected root, the leaf is in the tree.
///
/// # Placeholder Implementation
///
/// This is a PLACEHOLDER circuit that demonstrates the structure without
/// implementing a full Merkle proof verifier. Real Merkle circuits require:
///
/// 1. **Hash Function**: A SNARK-friendly hash like Poseidon, Rescue, or MiMC
///    - Each hash requires hundreds or thousands of constraints
///    - For a tree of depth 20, you need 20 hash computations
///
/// 2. **Path Traversal**: For each level in the tree:
///    - Hash current node with sibling (based on path index)
///    - Update current node to be the hash result
///    - Move to next level
///
/// 3. **Root Verification**: Compare computed root with expected root
///
/// A production Merkle circuit would have thousands of constraints.
/// This placeholder uses a single constraint for demonstration.
///
/// # Simplified Placeholder
///
/// For this demo, we use a trivial verification:
/// ```text
/// verify: (leaf + path_sum) * multiplier = root
/// ```
///
/// Where:
/// - `path_sum` is the sum of all path elements (simulating path commitment)
/// - `multiplier` is a constant to make the equation non-trivial
///
/// This is NOT a secure Merkle proof! Real implementations must use
/// proper hash functions and path traversal.
///
/// # R1CS Representation
///
/// For the placeholder, we use a single constraint:
///
/// Variables: [1, root, leaf, path_sum]
///
/// Constraint 1: Verify (leaf + path_sum) * multiplier = root
///   We compute the expected root using field arithmetic, then encode it as
///   a constant in the A vector, verifying: expected * 1 = root
///   A = [expected, 0, 0, 0]       // expected root as constant
///   B = [1, 0, 0, 0]              // constant 1
///   C = [0, 1, 0, 0]              // selects root
///
/// # Example
/// ```rust
/// use groth16_circuits::merkle::MerkleCircuit;
///
/// // Create circuit with leaf=100, path_sum=500, compute root
/// let leaf = 100u64;
/// let path_sum = 500u64;
/// let root = MerkleCircuit::compute_root(leaf, path_sum);
/// let circuit = MerkleCircuit::new(leaf, path_sum, root);
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
pub struct MerkleCircuit {
    /// Private leaf value (the value we're proving membership for)
    pub leaf: u64,
    /// Private authentication path (sum of path elements as placeholder)
    pub path_sum: u64,
    /// Public Merkle root (the commitment to all leaves)
    pub root: u64,
}

impl MerkleCircuit {
    /// Placeholder root computation: root = (leaf + path_sum) * 3
    ///
    /// # WARNING
    /// This is NOT a real Merkle root computation! Real Merkle trees use
    /// cryptographic hash functions and require hashing at each level of
    /// the tree. This is a trivial placeholder for demonstration only.
    ///
    /// A real Merkle root would:
    /// 1. Start with the leaf value
    /// 2. For each level in the tree:
    ///    a. Hash the current node with its sibling (based on path index)
    ///    b. Use the hash result as the current node for the next level
    /// 3. The final hash is the root
    ///
    /// This requires thousands of constraints in practice.
    ///
    /// # Arguments
    /// * `leaf` - The leaf value
    /// * `path_sum` - Sum of authentication path elements (placeholder)
    ///
    /// # Returns
    /// Placeholder root value
    pub fn compute_root(leaf: u64, path_sum: u64) -> u64 {
        (leaf.wrapping_add(path_sum)).wrapping_mul(3)
    }

    /// Creates a new Merkle membership circuit with the given inputs.
    ///
    /// # Arguments
    /// * `leaf` - Private leaf value
    /// * `path_sum` - Private authentication path (placeholder: sum of path elements)
    /// * `root` - Public Merkle root
    ///
    /// # Note
    /// This function does NOT verify that the leaf leads to the root.
    /// Use `verify()` to check the computation.
    pub fn new(leaf: u64, path_sum: u64, root: u64) -> Self {
        Self { leaf, path_sum, root }
    }

    /// Converts the circuit to R1CS constraints.
    ///
    /// For the placeholder Merkle circuit, we use a single constraint that
    /// verifies the root computation. The expected root value is pre-computed
    /// using field arithmetic and encoded as a constant in the constraint.
    ///
    /// Real Merkle circuits would require:
    /// - Hash function constraints (Poseidon: ~300 constraints per hash)
    /// - Path traversal constraints (one hash per tree level)
    /// - Root comparison constraint
    ///
    /// For a tree of depth 20, this would be ~6000 constraints.
    ///
    /// # Returns
    /// A vector containing one R1CS constraint (placeholder)
    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraints = Vec::new();

        // Variable layout: [1, root, leaf, path_sum]
        // Index:           [0, 1,    2,     3]

        // Compute the expected root using field arithmetic
        // For our simple computation: root = (leaf + path_sum) * 3
        let leaf_field = Fq::from(self.leaf);
        let path_field = Fq::from(self.path_sum);
        let multiplier = Fq::from(3u64);
        let expected = (leaf_field + path_field) * multiplier;

        // Constraint: expected * 1 = root
        // This verifies that the computed root matches the public output
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(expected)); // expected as constant
        constraint.add_b_variable(0, FieldWrapper::<Fq>::from(1u64)); // constant 1
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64)); // root
        constraints.push(constraint);

        constraints
    }

    /// Generates the witness assignment for this circuit instance.
    ///
    /// The witness is the assignment to all variables, following the standard
    /// Groth16 convention:
    /// [1, public_inputs..., private_inputs...]
    ///
    /// For the Merkle circuit: [1, root, leaf, path_sum]
    /// where:
    /// - Index 0: constant 1
    /// - Index 1: public output root
    /// - Index 2: private input leaf
    /// - Index 3: private input path_sum (authentication path placeholder)
    ///
    /// This ordering is important for the Groth16 IC computation.
    ///
    /// # Returns
    /// Vector of field elements representing the witness
    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.root), // public output root
            FieldWrapper::<Fq>::from(self.leaf), // private input leaf
            FieldWrapper::<Fq>::from(self.path_sum), // private input path_sum
        ]
    }

    /// Verifies that the circuit computation is correct (root computation).
    ///
    /// # Returns
    /// * `true` - If the root computation is correct
    /// * `false` - If the computation is incorrect
    pub fn verify(&self) -> bool {
        let computed = Self::compute_root(self.leaf, self.path_sum);
        computed == self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_root() {
        // Test that the root computation is deterministic
        let leaf = 100u64;
        let path_sum = 500u64;
        let root1 = MerkleCircuit::compute_root(leaf, path_sum);
        let root2 = MerkleCircuit::compute_root(leaf, path_sum);
        assert_eq!(root1, root2);
    }

    #[test]
    fn test_circuit_valid() {
        let leaf = 100u64;
        let path_sum = 500u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        assert!(circuit.verify());
    }

    #[test]
    fn test_circuit_invalid() {
        let leaf = 100u64;
        let path_sum = 500u64;
        let wrong_root = 99999u64; // Wrong root
        let circuit = MerkleCircuit::new(leaf, path_sum, wrong_root);
        assert!(!circuit.verify());
    }

    #[test]
    fn test_to_r1cs() {
        let leaf = 100u64;
        let path_sum = 500u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        let constraints = circuit.to_r1cs();

        assert_eq!(constraints.len(), 1);

        // Check that the constraint uses reasonable variables
        assert!(constraints[0].unique_variable_count() > 0);
    }

    #[test]
    fn test_witness() {
        let leaf = 100u64;
        let path_sum = 500u64;
        let root = 1800u64; // (100 + 500) * 3 = 1800
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        let witness = circuit.witness();

        assert_eq!(witness.len(), 4); // [1, root, leaf, path_sum]
        assert_eq!(witness[0].value, Fq::from(1u64)); // constant 1
        assert_eq!(witness[1].value, Fq::from(1800u64)); // public output root
        assert_eq!(witness[2].value, Fq::from(100u64)); // private input leaf
        assert_eq!(witness[3].value, Fq::from(500u64)); // private input path_sum
    }

    #[test]
    fn test_r1cs_satisfied() {
        let leaf = 100u64;
        let path_sum = 500u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // The single constraint should be satisfied
        assert!(constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_r1cs_not_satisfied() {
        // Create circuit with wrong root
        let leaf = 100u64;
        let path_sum = 500u64;
        let wrong_root = 99999u64;
        let circuit = MerkleCircuit::new(leaf, path_sum, wrong_root);

        let constraints = circuit.to_r1cs();

        // Witness with wrong root value (using standard ordering: [1, root, leaf, path_sum])
        let witness = vec![
            FieldWrapper::<Fq>::from(1u64),     // constant 1
            FieldWrapper::<Fq>::from(99999u64), // Wrong root!
            FieldWrapper::<Fq>::from(100u64),   // leaf
            FieldWrapper::<Fq>::from(500u64),   // path_sum
        ];

        // The constraint should NOT be satisfied
        assert!(!constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_different_leaves() {
        // Different leaves should produce different roots (most of the time)
        let leaf1 = 100u64;
        let leaf2 = 200u64;
        let path_sum = 500u64;
        let root1 = MerkleCircuit::compute_root(leaf1, path_sum);
        let root2 = MerkleCircuit::compute_root(leaf2, path_sum);

        // They should be different
        assert_ne!(root1, root2);

        // Verify both circuits
        let circuit1 = MerkleCircuit::new(leaf1, path_sum, root1);
        let circuit2 = MerkleCircuit::new(leaf2, path_sum, root2);
        assert!(circuit1.verify());
        assert!(circuit2.verify());
    }

    #[test]
    fn test_zero_leaf() {
        // Edge case: leaf = 0
        let leaf = 0u64;
        let path_sum = 500u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[2].value, Fq::from(0u64)); // leaf
    }

    #[test]
    fn test_zero_path() {
        // Edge case: path_sum = 0
        let leaf = 100u64;
        let path_sum = 0u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        assert!(circuit.verify());

        let witness = circuit.witness();
        assert_eq!(witness[3].value, Fq::from(0u64)); // path_sum
    }

    #[test]
    fn test_large_values() {
        // Test with larger values
        let leaf = 1000000u64;
        let path_sum = 2000000u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);
        let circuit = MerkleCircuit::new(leaf, path_sum, root);
        assert!(circuit.verify());

        // Verify R1CS constraints are satisfied
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();
        assert!(constraints[0].is_satisfied(&witness));
    }

    #[test]
    fn test_root_computation() {
        // Verify the root computation formula: (leaf + path_sum) * 3
        let leaf = 100u64;
        let path_sum = 500u64;
        let root = MerkleCircuit::compute_root(leaf, path_sum);

        // (100 + 500) * 3 = 600 * 3 = 1800
        assert_eq!(root, 1800u64);
    }

    #[test]
    fn test_placeholder_nature() {
        // This test documents the placeholder nature
        let leaf = 100u64;
        let path_sum = 500u64;
        let circuit = MerkleCircuit::new(leaf, path_sum, 1800);

        // The circuit should verify
        assert!(circuit.verify());

        // But this is NOT a real Merkle proof!
        // Real Merkle proofs require:
        // - Proper hash functions (Poseidon, Rescue, MiMC)
        // - Path traversal with sibling hashes
        // - Path indices (left/right at each level)
        // - Thousands of constraints for depth-20 trees
    }
}
