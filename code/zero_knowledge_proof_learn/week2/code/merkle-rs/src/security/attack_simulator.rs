//! Attack simulation framework for Merkle trees
//!
//! This module simulates various attack vectors against Merkle trees to test
//! their resistance to known vulnerabilities and attack patterns.

use crate::{MerkleTree, Hash32, verify};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Results from attack simulations
#[derive(Debug)]
pub struct AttackResults {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
    pub attack_success_rate: f64,
}

/// Attack simulator
pub struct AttackSimulator {
    config: AttackSimConfig,
}

/// Configuration for attack simulation
#[derive(Debug, Clone)]
pub struct AttackSimConfig {
    /// Number of attack attempts
    pub attack_attempts: usize,
    /// Random seed
    pub seed: Option<u64>,
}

impl AttackSimulator {
    /// Create a new attack simulator
    pub fn new(security_config: &crate::security::SecurityTestConfig) -> Self {
        Self {
            config: AttackSimConfig {
                attack_attempts: security_config.test_iterations,
                seed: security_config.seed,
            },
        }
    }

    /// Run all attack simulations
    pub fn run_attacks(&self) -> AttackResults {
        let mut results = AttackResults {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
            attack_success_rate: 0.0,
        };

        // Attack 1: Second preimage attack attempt
        match self.simulate_second_preimage_attack() {
            Ok(success) => {
                if !success {
                    results.passed += 1; // Attack failed as expected
                } else {
                    results.failed += 1;
                    results.failures.push("Second preimage attack succeeded!".to_string());
                }
            }
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Second preimage attack error: {}", e));
            }
        }

        // Attack 2: Proof manipulation attack
        match self.simulate_proof_manipulation() {
            Ok(success) => {
                if !success {
                    results.passed += 1;
                } else {
                    results.failed += 1;
                    results.failures.push("Proof manipulation attack succeeded!".to_string());
                }
            }
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Proof manipulation error: {}", e));
            }
        }

        // Attack 3: Root forgery attempt
        match self.simulate_root_forgery() {
            Ok(success) => {
                if !success {
                    results.passed += 1;
                } else {
                    results.failed += 1;
                    results.failures.push("Root forgery attack succeeded!".to_string());
                }
            }
            Err(e) => {
                results.failed += 1;
                results.failures.push(format!("Root forgery error: {}", e));
            }
        }

        results.attack_success_rate = results.failed as f64 / (results.passed + results.failed) as f64;
        results
    }

    /// Simulate second preimage attack
    fn simulate_second_preimage_attack(&self) -> Result<bool, String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.attack_attempts {
            // Create a legitimate tree
            let leaf_count = rng.gen_range(2..=8);
            let leaves: Vec<Vec<u8>> = (0..leaf_count)
                .map(|_| {
                    let size = rng.gen_range(1..=20);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves.clone());
            let target_root = tree.root();

            // Try to find different leaves that produce the same root
            // In practice, this should be computationally infeasible
            for attempt in 0..100 { // Limited attempts for practical testing
                let alt_leaves: Vec<Vec<u8>> = (0..leaf_count)
                    .map(|_| {
                        let size = rng.gen_range(1..=20);
                        (0..size).map(|_| rng.gen()).collect()
                    })
                    .collect();

                let alt_tree = MerkleTree::from_leaves(alt_leaves);
                if alt_tree.root() == target_root && alt_leaves != leaves {
                    return Ok(true); // Attack succeeded!
                }
            }
        }

        Ok(false) // Attack failed (good!)
    }

    /// Simulate proof manipulation attack
    fn simulate_proof_manipulation(&self) -> Result<bool, String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.attack_attempts {
            // Create a legitimate tree and proof
            let leaves: Vec<Vec<u8>> = (0..5)
                .map(|_| {
                    let size = rng.gen_range(1..=20);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves.clone());
            let root = tree.root();
            let leaf_index = rng.gen_range(0..leaves.len());
            let mut proof = tree.prove(leaf_index);

            // Try to manipulate the proof to verify a different leaf
            let fake_leaf: Vec<u8> = (0..20).map(|_| rng.gen()).collect();

            // Various manipulation attempts
            let manipulations = vec![
                // Flip bits in siblings
                || {
                    for sibling in &mut proof.siblings {
                        sibling[0] ^= 0x01;
                    }
                },
                // Modify path bits
                || {
                    for bit in &mut proof.path_bits {
                        *bit = !*bit;
                    }
                },
                // Remove siblings
                || {
                    proof.siblings.pop();
                },
                // Add fake siblings
                || {
                    proof.siblings.push([0u8; 32]);
                },
            ];

            for manipulation in manipulations {
                let mut test_proof = proof.clone();
                manipulation();

                if verify(root, &fake_leaf, test_proof) {
                    return Ok(true); // Attack succeeded!
                }
            }
        }

        Ok(false) // Attack failed (good!)
    }

    /// Simulate root forgery attack
    fn simulate_root_forgery(&self) -> Result<bool, String> {
        let mut rng = self.create_rng();

        for _ in 0..self.config.attack_attempts {
            // Generate random leaves
            let leaves: Vec<Vec<u8>> = (0..4)
                .map(|_| {
                    let size = rng.gen_range(1..=20);
                    (0..size).map(|_| rng.gen()).collect()
                })
                .collect();

            let tree = MerkleTree::from_leaves(leaves.clone());
            let real_root = tree.root();

            // Try to forge a valid proof for the real root with fake data
            let fake_leaf: Vec<u8> = (0..20).map(|_| rng.gen()).collect();

            // Generate a fake proof
            let fake_proof = crate::MerkleProof {
                siblings: vec![[0u8; 32]; tree.leaves().len().next_power_of_two().ilog2() as usize],
                path_bits: vec![false; tree.leaves().len().next_power_of_two().ilog2() as usize],
            };

            if verify(real_root, &fake_leaf, fake_proof) {
                return Ok(true); // Attack succeeded!
            }
        }

        Ok(false) // Attack failed (good!)
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
    fn test_attack_simulator_creation() {
        let config = crate::security::SecurityTestConfig::default();
        let simulator = AttackSimulator::new(&config);
        assert_eq!(simulator.config.attack_attempts, 1000);
    }

    #[test]
    fn test_attack_simulation() {
        let config = crate::security::SecurityTestConfig {
            test_iterations: 10,
            max_data_size: 10,
            exhaustive: false,
            seed: Some(42),
        };
        let simulator = AttackSimulator::new(&config);
        let results = simulator.run_attacks();

        // Attacks should fail (good for security)
        assert!(results.attack_success_rate < 0.1);
    }
}