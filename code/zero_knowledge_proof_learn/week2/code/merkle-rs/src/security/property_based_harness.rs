//! Property-based testing harness for Merkle trees
//!
//! This module provides property-based testing capabilities using proptest
//! to systematically test Merkle tree properties across a wide range of inputs.

use crate::{MerkleTree, Hash32, verify};
use proptest::prelude::*;

/// Property-based testing harness
pub struct PropertyBasedHarness {
    config: PropertyTestConfig,
}

/// Configuration for property-based testing
#[derive(Debug, Clone)]
pub struct PropertyTestConfig {
    /// Number of test cases
    pub test_cases: usize,
    /// Random seed
    pub seed: Option<u64>,
}

impl PropertyBasedHarness {
    /// Create a new property-based harness
    pub fn new(security_config: &crate::security::SecurityTestConfig) -> Self {
        Self {
            config: PropertyTestConfig {
                test_cases: security_config.test_iterations,
                seed: security_config.seed,
            },
        }
    }

    /// Run all property-based tests
    pub fn run_property_tests(&self) -> PropertyTestResults {
        let mut results = PropertyTestResults {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
        };

        // Property 1: Merkle root determinism
        if self.test_root_determinism().is_ok() {
            results.passed += 1;
        } else {
            results.failed += 1;
            results.failures.push("Root determinism property failed".to_string());
        }

        // Property 2: Proof verification correctness
        if self.test_proof_verification().is_ok() {
            results.passed += 1;
        } else {
            results.failed += 1;
            results.failures.push("Proof verification property failed".to_string());
        }

        // Property 3: Tree construction consistency
        if self.test_tree_consistency().is_ok() {
            results.passed += 1;
        } else {
            results.failed += 1;
            results.failures.push("Tree consistency property failed".to_string());
        }

        results
    }

    /// Test that Merkle root is deterministic
    fn test_root_determinism(&self) -> Result<(), TestCaseError> {
        proptest!(|(leaves in prop::collection::vec(prop::collection::vec(any::<u8>(), 1..=50), 1..=10))| {
            let tree1 = MerkleTree::from_leaves(leaves.clone());
            let tree2 = MerkleTree::from_leaves(leaves);
            prop_assert_eq!(tree1.root(), tree2.root());
        });

        Ok(())
    }

    /// Test proof verification property
    fn test_proof_verification(&self) -> Result<(), TestCaseError> {
        proptest!(|(leaves in prop::collection::vec(prop::collection::vec(any::<u8>(), 1..=50), 1..=10))| {
            let tree = MerkleTree::from_leaves(leaves.clone());
            let root = tree.root();

            // All leaves should have valid proofs
            for (i, leaf) in leaves.iter().enumerate() {
                let proof = tree.prove(i);
                prop_assert!(verify(root, leaf, proof));
            }
        });

        Ok(())
    }

    /// Test tree construction consistency
    fn test_tree_consistency(&self) -> Result<(), TestCaseError> {
        proptest!(|(leaves in prop::collection::vec(prop::collection::vec(any::<u8>(), 1..=50), 2..=10))| {
            let tree = MerkleTree::from_leaves(leaves.clone());

            // Tree should have at least one level (the leaves)
            prop_assert!(!tree.leaves().is_empty());

            // Root should not be all zeros (unless by extreme coincidence)
            prop_assert!(tree.root() != [0u8; 32]);
        });

        Ok(())
    }
}

/// Results from property-based testing
#[derive(Debug)]
pub struct PropertyTestResults {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_harness_creation() {
        let config = crate::security::SecurityTestConfig::default();
        let harness = PropertyBasedHarness::new(&config);
        assert_eq!(harness.config.test_cases, 1000);
    }
}