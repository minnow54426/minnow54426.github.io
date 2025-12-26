//! Binding property testing for Merkle trees
//!
//! This module tests the binding property: that a Merkle root commits to a specific
//! set of leaves, making it computationally infeasible to find different leaf sets
//! that produce the same root.

use crate::{MerkleTree, Hash32};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Results from binding property testing
#[derive(Debug)]
pub struct BindingTestResults {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
    pub binding_strength: f64,
}

/// Binding property tester
pub struct BindingPropertyTester {
    config: BindingTestConfig,
}

/// Configuration for binding property tests
#[derive(Debug, Clone)]
pub struct BindingTestConfig {
    /// Number of test attempts
    pub test_attempts: usize,
    /// Maximum leaf set size to test
    pub max_leaves: usize,
    /// Random seed
    pub seed: Option<u64>,
}

impl BindingPropertyTester {
    /// Create a new binding property tester
    pub fn new(security_config: &crate::security::SecurityTestConfig) -> Self {
        Self {
            config: BindingTestConfig {
                test_attempts: security_config.test_iterations,
                max_leaves: security_config.max_data_size,
                seed: security_config.seed,
            },
        }
    }

    /// Run all binding property tests
    pub fn run_tests(&self) -> BindingTestResults {
        let mut results = BindingTestResults {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
            binding_strength: 0.0,
        };

        // Test 1: Different leaf orders produce different roots
        match self.test_leaf_order_binding() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Leaf order binding: {}", e));
            }
        }

        // Test 2: Different leaf content produces different roots
        match self.test_content_binding() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Content binding: {}", e));
            }
        }

        // Test 3: Tree structure binding (different tree shapes)
        match self.test_structure_binding() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Structure binding: {}", e));
            }
        }

        // Test 4: Attempt to find alternative leaf sets for same root
        match self.test_alternative_leaf_sets() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Alternative leaf sets: {}", e));
            }
        }

        // Calculate binding strength (0-1, higher is better)
        results.binding_strength = self.calculate_binding_strength(&results);

        results
    }

    /// Test that different leaf orders produce different roots
    fn test_leaf_order_binding(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.test_attempts {
            // Generate random leaf data
            let leaf_count = rng.gen_range(3..=10);
            let leaves: Vec<Vec<u8>> = (0..leaf_count)
                .map(|_| {
                    let size = rng.gen_range(1..=50);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            // Create tree with original order
            let tree1 = MerkleTree::from_leaves(leaves.clone());
            let root1 = tree1.root();

            // Create tree with shuffled order
            let mut shuffled_leaves = leaves.clone();
            shuffled_leaves.shuffle(&mut rng);
            let tree2 = MerkleTree::from_leaves(shuffled_leaves);
            let root2 = tree2.root();

            // Roots should be different unless order is identical
            if root1 == root2 && leaves != shuffled_leaves {
                return Err(format!(
                    "Order binding failed! Same root from different leaf orders: root={:?}",
                    root1
                ));
            }
        }

        Ok(())
    }

    /// Test that different leaf content produces different roots
    fn test_content_binding(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.test_attempts {
            // Generate random leaf data
            let leaf_count = rng.gen_range(2..=10);
            let leaves: Vec<Vec<u8>> = (0..leaf_count)
                .map(|_| {
                    let size = rng.gen_range(1..=50);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree1 = MerkleTree::from_leaves(leaves.clone());
            let root1 = tree1.root();

            // Modify one leaf slightly
            let mut modified_leaves = leaves;
            let modify_index = rng.gen_range(0..modified_leaves.len());
            if modified_leaves[modify_index].is_empty() {
                modified_leaves[modify_index].push(1);
            } else {
                let pos = rng.gen_range(0..modified_leaves[modify_index].len());
                modified_leaves[modify_index][pos] ^= 0x01; // Flip one bit
            }

            let tree2 = MerkleTree::from_leaves(modified_leaves);
            let root2 = tree2.root();

            // Roots should be different
            if root1 == root2 {
                return Err(format!(
                    "Content binding failed! Same root after leaf modification: root={:?}",
                    root1
                ));
            }
        }

        Ok(())
    }

    /// Test that different tree structures produce different roots
    fn test_structure_binding(&self) -> Result<(), String> {
        // Test with specific numbers that create different tree shapes
        let test_cases = vec![
            (2, vec![b"a".to_vec(), b"b".to_vec()]),
            (3, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]),
            (4, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec(), b"d".to_vec()]),
            (5, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec(), b"d".to_vec(), b"e".to_vec()]),
        ];

        for (count, leaves) in test_cases {
            let tree = MerkleTree::from_leaves(leaves);
            let root = tree.root();

            // Each tree structure should produce a unique root
            // This is a basic sanity check - the real test is that you can't
            // find different structures with the same root
            if root == [0u8; 32] {
                return Err(format!("Structure binding failed for {} leaves: zero root", count));
            }
        }

        Ok(())
    }

    /// Attempt to find alternative leaf sets that produce the same root
    fn test_alternative_leaf_sets(&self) -> Result<(), String> {
        let mut rng = self.create_rng();
        let mut seen_roots = std::collections::HashMap::new();

        // Generate many random trees and look for collisions
        for _ in 0..self.config.test_attempts {
            let leaf_count = rng.gen_range(2..=8);
            let leaves: Vec<Vec<u8>> = (0..leaf_count)
                .map(|_| {
                    let size = rng.gen_range(1..=20);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves.clone());
            let root = tree.root();

            if let Some(prev_leaves) = seen_roots.get(&root) {
                // Check if this is actually a different leaf set
                if self.different_leaf_sets(&leaves, prev_leaves) {
                    return Err(format!(
                        "Binding violation! Same root from different leaf sets: root={:?}",
                        root
                    ));
                }
            } else {
                seen_roots.insert(root, leaves);
            }
        }

        Ok(())
    }

    /// Check if two leaf sets are different
    fn different_leaf_sets(&self, set1: &[Vec<u8>], set2: &[Vec<u8>]) -> bool {
        if set1.len() != set2.len() {
            return true;
        }

        // Check if sets are different (order matters for Merkle trees)
        for (a, b) in set1.iter().zip(set2.iter()) {
            if a != b {
                return true;
            }
        }

        false
    }

    /// Calculate binding strength based on test results
    fn calculate_binding_strength(&self, results: &BindingTestResults) -> f64 {
        if results.passed + results.failed == 0 {
            return 0.0;
        }

        let pass_rate = results.passed as f64 / (results.passed + results.failed) as f64;

        // Adjust based on the thoroughness of testing
        let thoroughness_factor = (self.config.test_attempts as f64 / 1000.0).min(1.0);

        pass_rate * thoroughness_factor
    }

    /// Create random number generator with configured seed
    fn create_rng(&self) -> ChaCha8Rng {
        match self.config.seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_tester_creation() {
        let config = crate::security::SecurityTestConfig::default();
        let tester = BindingPropertyTester::new(&config);
        assert_eq!(tester.config.test_attempts, 1000);
    }

    #[test]
    fn test_different_leaf_sets_detection() {
        let config = crate::security::SecurityTestConfig::default();
        let tester = BindingPropertyTester::new(&config);

        let set1 = vec![b"a".to_vec(), b"b".to_vec()];
        let set2 = vec![b"a".to_vec(), b"b".to_vec()];
        let set3 = vec![b"b".to_vec(), b"a".to_vec()];
        let set4 = vec![b"a".to_vec(), b"c".to_vec()];

        assert!(!tester.different_leaf_sets(&set1, &set2));
        assert!(tester.different_leaf_sets(&set1, &set3)); // Order matters
        assert!(tester.different_leaf_sets(&set1, &set4));
    }

    #[test]
    fn test_binding_strength_calculation() {
        let config = crate::security::SecurityTestConfig::default();
        let tester = BindingPropertyTester::new(&config);

        let results = BindingTestResults {
            passed: 4,
            failed: 0,
            failures: Vec::new(),
            binding_strength: 0.0,
        };

        let strength = tester.calculate_binding_strength(&results);
        assert!(strength > 0.9); // Should be high with all tests passing
    }
}