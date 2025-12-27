//! Property-based testing harness for Merkle trees
//!
//! This module provides property-based testing capabilities using random generation
//! to systematically test Merkle tree properties across a wide range of inputs.

use crate::{MerkleTree, verify};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

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
    fn test_root_determinism(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.test_cases.min(100) {
            let leaves: Vec<Vec<u8>> = (0..rng.gen_range(1..=10))
                .map(|_| {
                    let size = rng.gen_range(1..=50);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree1 = MerkleTree::from_leaves(leaves.clone());
            let tree2 = MerkleTree::from_leaves(leaves);

            if tree1.root() != tree2.root() {
                return Err("Non-deterministic root detected".to_string());
            }
        }

        Ok(())
    }

    /// Test proof verification property
    fn test_proof_verification(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.test_cases.min(100) {
            let leaves: Vec<Vec<u8>> = (0..rng.gen_range(1..=10))
                .map(|_| {
                    let size = rng.gen_range(1..=50);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves.clone());
            let root = tree.root();

            // All leaves should have valid proofs
            for (i, leaf) in leaves.iter().enumerate() {
                let proof = tree.prove(i);
                if !verify(root, leaf, proof) {
                    return Err(format!("Proof verification failed for leaf {}", i));
                }
            }
        }

        Ok(())
    }

    /// Test tree construction consistency
    fn test_tree_consistency(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.test_cases.min(100) {
            let leaves: Vec<Vec<u8>> = (0..rng.gen_range(2..=10))
                .map(|_| {
                    let size = rng.gen_range(1..=50);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves);

            // Tree should have at least one level (the leaves)
            if tree.leaves().is_empty() {
                return Err("Empty tree detected".to_string());
            }

            // Root should not be all zeros (unless by extreme coincidence)
            if tree.root() == [0u8; 32] {
                return Err("Zero root detected (unlikely coincidence)".to_string());
            }
        }

        Ok(())
    }

    /// Create random number generator with configured seed
    fn create_rng(&self) -> ChaCha8Rng {
        match self.config.seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        }
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