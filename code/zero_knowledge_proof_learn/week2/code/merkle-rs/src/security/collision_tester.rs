//! Collision resistance testing for Merkle trees
//!
//! This module implements comprehensive tests to verify the collision resistance
//! properties of the Merkle tree implementation, including birthday attack simulations
//! and domain separation verification.

use crate::{MerkleTree, Hash32};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

/// Configuration for collision testing
#[derive(Debug, Clone)]
pub struct CollisionTestConfig {
    /// Number of hash attempts per test
    pub hash_attempts: usize,
    /// Number of test rounds
    pub test_rounds: usize,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for CollisionTestConfig {
    fn default() -> Self {
        Self {
            hash_attempts: 10000,
            test_rounds: 100,
            seed: None,
        }
    }
}

/// Results from collision testing
#[derive(Debug)]
pub struct CollisionTestResults {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
    pub collision_probability: f64,
    pub birthday_attack_results: Vec<BirthdayAttackResult>,
}

/// Result of a birthday attack simulation
#[derive(Debug, Clone)]
pub struct BirthdayAttackResult {
    pub hash_output_bits: usize,
    pub attempts: usize,
    pub collision_found: bool,
    pub theoretical_probability: f64,
}

/// Collision resistance tester for Merkle trees
pub struct CollisionTester {
    config: CollisionTestConfig,
}

impl CollisionTester {
    /// Create a new collision tester
    pub fn new(security_config: &crate::security::SecurityTestConfig) -> Self {
        Self {
            config: CollisionTestConfig {
                hash_attempts: security_config.test_iterations,
                test_rounds: 10,
                seed: security_config.seed,
            },
        }
    }

    /// Run all collision resistance tests
    pub fn run_tests(&self) -> CollisionTestResults {
        let mut results = CollisionTestResults {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
            collision_probability: 0.0,
            birthday_attack_results: Vec::new(),
        };

        // Test 1: Basic collision resistance
        match self.test_basic_collision_resistance() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Basic collision resistance: {}", e));
            }
        }

        // Test 2: Domain separation verification
        match self.test_domain_separation() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Domain separation: {}", e));
            }
        }

        // Test 3: Birthday attack simulation
        match self.test_birthday_attack() {
            Ok(birthday_results) => {
                results.passed += 1;
                results.birthday_attack_results = birthday_results;
            }
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Birthday attack: {}", e));
            }
        }

        // Test 4: Leaf vs internal node hash distinction
        match self.test_leaf_internal_distinction() {
            Ok(()) => results.passed += 1,
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Leaf/internal distinction: {}", e));
            }
        }

        // Calculate overall collision probability
        results.collision_probability = self.estimate_collision_probability();

        results
    }

    /// Test basic collision resistance with random data
    fn test_basic_collision_resistance(&self) -> Result<(), String> {
        let mut rng = self.create_rng();
        let mut seen_roots = HashMap::new();

        for _ in 0..self.config.hash_attempts {
            // Generate random leaf data
            let leaf_count = rng.gen_range(2..=20);
            let leaves: Vec<Vec<u8>> = (0..leaf_count)
                .map(|_| {
                    let size = rng.gen_range(1..=100);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves);
            let root = tree.root();

            // Check for collision
            if let Some(prev_leaves) = seen_roots.get(&root) {
                // Verify this is actually a collision (different leaves)
                if prev_leaves != &tree.leaves() {
                    return Err(format!(
                        "Collision found! Same root {:?} from different leaf sets",
                        root
                    ));
                }
            } else {
                seen_roots.insert(root, tree.leaves());
            }
        }

        Ok(())
    }

    /// Test that domain separation (0x00/0x01 prefixes) works correctly
    fn test_domain_separation(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..1000 {
            // Generate random data
            let data: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

            // Simulate leaf hash (0x00 prefix)
            let leaf_hash = crate::hash_leaf(&data);

            // Simulate internal node hash with two identical children (0x01 prefix)
            let internal_hash = crate::hash_internal(&leaf_hash, &leaf_hash);

            // These should be different due to domain separation
            if leaf_hash == internal_hash {
                return Err(format!(
                    "Domain separation failed! Leaf and internal hashes are equal: {:?}",
                    leaf_hash
                ));
            }
        }

        Ok(())
    }

    /// Simulate birthday attacks with different hash output sizes
    fn test_birthday_attack(&self) -> Result<Vec<BirthdayAttackResult>, String> {
        let mut results = Vec::new();
        let mut rng = self.create_rng();

        // Test with reduced hash sizes to observe collisions
        for bits in [8, 12, 16, 20] {
            let attempts = 2usize.pow(bits as u32 / 2 + 1); // sqrt(2^bits) attempts
            let mut hashes = Vec::new();
            let mut collision_found = false;

            for _ in 0..attempts {
                let data: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
                let tree = MerkleTree::from_leaves(vec![data]);
                let full_hash = tree.root();

                // Truncate hash to desired bit length
                let truncated = self.truncate_hash(&full_hash, bits);

                if hashes.contains(&truncated) {
                    collision_found = true;
                    break;
                }
                hashes.push(truncated);
            }

            // Calculate theoretical collision probability
            let theoretical_prob = self.birthday_probability(attempts, bits);

            results.push(BirthdayAttackResult {
                hash_output_bits: bits,
                attempts,
                collision_found,
                theoretical_probability: theoretical_prob,
            });
        }

        Ok(results)
    }

    /// Test that leaf hashes and internal node hashes are properly distinguished
    fn test_leaf_internal_distinction(&self) -> Result<(), String> {
        let mut rng = self.create_rng();

        for _ in 0..1000 {
            // Generate different data that could potentially produce same hash
            let data1: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
            let data2: Vec<u8> = (0..32).map(|_| rng.gen()).collect();

            // Leaf hashes
            let leaf1 = crate::hash_leaf(&data1);
            let leaf2 = crate::hash_leaf(&data2);

            // Internal hash (hash the leaf hashes, not the raw data)
            let internal = crate::hash_internal(&leaf1, &leaf2);

            // Internal hash should not match any individual leaf hash
            if internal == leaf1 || internal == leaf2 {
                return Err(format!(
                    "Internal hash collision with leaf hash: internal={:?}, leaf1={:?}, leaf2={:?}",
                    internal, leaf1, leaf2
                ));
            }
        }

        Ok(())
    }

    /// Estimate collision probability based on test results
    fn estimate_collision_probability(&self) -> f64 {
        // Simplified estimation based on hash output size and test attempts
        let hash_bits = 256; // SHA-256
        let attempts = self.config.hash_attempts;
        self.birthday_probability(attempts, hash_bits)
    }

    /// Calculate birthday attack probability
    fn birthday_probability(&self, attempts: usize, hash_bits: usize) -> f64 {
        let hash_space = 2u64.pow(hash_bits.min(63) as u32) as f64;
        if attempts as f64 >= hash_space {
            return 1.0;
        }

        // Approximation: p ≈ 1 - e^(-n(n-1)/(2*2^k))
        let n = attempts as f64;
        let exponent = -n * (n - 1.0) / (2.0 * hash_space);
        1.0 - exponent.exp()
    }

    /// Truncate hash to specified number of bits
    fn truncate_hash(&self, hash: &Hash32, bits: usize) -> u32 {
        let bytes_needed = (bits + 7) / 8;
        let mut result = 0u32;
        for (i, &byte) in hash.iter().take(bytes_needed).enumerate() {
            result |= (byte as u32) << (i * 8);
        }
        let mask = (1u32 << bits.min(32)) - 1;
        result & mask
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
    fn test_collision_tester_creation() {
        let config = crate::security::SecurityTestConfig::default();
        let tester = CollisionTester::new(&config);
        assert_eq!(tester.config.hash_attempts, 1000);
    }

    #[test]
    fn test_birthday_probability_calculation() {
        let config = crate::security::SecurityTestConfig::default();
        let tester = CollisionTester::new(&config);

        // Small hash space, high collision probability
        let prob = tester.birthday_probability(100, 8);
        assert!(prob > 0.5);

        // Large hash space, low collision probability
        let prob = tester.birthday_probability(1000, 256);
        assert!(prob < 0.0001);
    }

    #[test]
    fn test_hash_truncation() {
        let config = crate::security::SecurityTestConfig::default();
        let tester = CollisionTester::new(&config);

        let hash = Hash32([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                          0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                          0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00,
                          0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);

        let truncated = tester.truncate_hash(&hash, 16);
        assert_eq!(truncated, 0x5678); // Last 16 bits
    }
}