//! Statistical analysis tools for Merkle tree security properties
//!
//! This module provides statistical analysis capabilities including randomness
//! quality testing, avalanche effect measurement, and distribution analysis.

use crate::{MerkleTree, Hash32, hash_leaf};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use statrs::statistics::*;

/// Statistical analyzer for Merkle tree security properties
pub struct StatisticalAnalyzer {
    config: StatisticalConfig,
}

/// Configuration for statistical analysis
#[derive(Debug, Clone)]
pub struct StatisticalConfig {
    /// Number of samples for analysis
    pub sample_size: usize,
    /// Random seed
    pub seed: Option<u64>,
}

/// Statistical analysis results
#[derive(Debug, Clone)]
pub struct StatisticalResults {
    pub randomness_quality: f64,
    pub avalanche_effect: f64,
    pub distribution_uniformity: f64,
}

impl StatisticalAnalyzer {
    /// Create a new statistical analyzer
    pub fn new(security_config: &crate::security::SecurityTestConfig) -> Self {
        Self {
            config: StatisticalConfig {
                sample_size: security_config.test_iterations.min(10000),
                seed: security_config.seed,
            },
        }
    }

    /// Analyze all statistical properties
    pub fn analyze_properties(&self) -> crate::security::SecurityMetrics {
        let randomness = self.analyze_randomness_quality();
        let avalanche = self.analyze_avalanche_effect();
        let uniformity = self.analyze_distribution_uniformity();

        crate::security::SecurityMetrics {
            avg_test_time_us: 0.0, // Would need timing instrumentation
            collision_resistance: randomness,
            binding_strength: avalanche,
            randomness_quality: uniformity,
        }
    }

    /// Analyze randomness quality of hash outputs
    fn analyze_randomness_quality(&self) -> f64 {
        let mut rng = self.create_rng();
        let mut hash_bytes = Vec::new();

        // Collect hash outputs
        for _ in 0..self.config.sample_size {
            let data: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
            let hash = hash_leaf(&data);
            hash_bytes.extend_from_slice(&hash);
        }

        // Simple randomness test: check bit distribution
        let ones = hash_bytes.iter().map(|&b| b.count_ones() as usize).sum::<usize>();
        let total_bits = hash_bytes.len() * 8;
        let ones_ratio = ones as f64 / total_bits as f64;

        // Perfect randomness would have 0.5 ratio
        1.0 - (ones_ratio - 0.5).abs() * 2.0
    }

    /// Analyze avalanche effect
    fn analyze_avalanche_effect(&self) -> f64 {
        let mut rng = self.create_rng();
        let mut total_changes = 0;

        for _ in 0..self.config.sample_size.min(1000) {
            // Generate random data
            let mut data: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
            let original_hash = hash_leaf(&data);

            // Flip one random bit
            let byte_index = rng.gen_range(0..data.len());
            let bit_index = rng.gen_range(0..8);
            data[byte_index] ^= 1 << bit_index;

            let modified_hash = hash_leaf(&data);

            // Count differing bits
            let diff_bits = original_hash
                .iter()
                .zip(modified_hash.iter())
                .map(|(&a, &b)| (a ^ b).count_ones() as usize)
                .sum::<usize>();

            total_changes += diff_bits;
        }

        // Calculate average bit changes (ideal is 50% of bits)
        let avg_changes = total_changes as f64 / (self.config.sample_size.min(1000) as f64);
        let ideal_changes = 256.0 * 0.5; // 256 bits * 50%

        1.0 - (avg_changes - ideal_changes).abs() / ideal_changes
    }

    /// Analyze distribution uniformity
    fn analyze_distribution_uniformity(&self) -> f64 {
        let mut rng = self.create_rng();
        let mut hash_values = Vec::new();

        // Collect hash values as integers for distribution analysis
        for _ in 0..self.config.sample_size.min(10000) {
            let data: Vec<u8> = (0..8).map(|_| rng.gen()).collect();
            let hash = hash_leaf(&data);

            // Convert first 8 bytes to u64 for statistical analysis
            let mut value = 0u64;
            for (i, &byte) in hash.iter().take(8).enumerate() {
                value |= (byte as u64) << (i * 8);
            }
            hash_values.push(value as f64);
        }

        if hash_values.len() < 2 {
            return 0.0;
        }

        // Calculate basic statistics
        let mean = hash_values.iter().sum::<f64>() / hash_values.len() as f64;
        let variance = hash_values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / hash_values.len() as f64;

        // Uniformity check: variance should be high for uniform distribution
        let max_possible_variance = (u64::MAX as f64).powi(2) / 12.0; // Variance of uniform distribution
        let normalized_variance = variance / max_possible_variance;

        normalized_variance.min(1.0)
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
    fn test_statistical_analyzer_creation() {
        let config = crate::security::SecurityTestConfig::default();
        let analyzer = StatisticalAnalyzer::new(&config);
        assert_eq!(analyzer.config.sample_size, 1000);
    }

    #[test]
    fn test_randomness_analysis() {
        let config = crate::security::SecurityTestConfig {
            test_iterations: 100,
            max_data_size: 10,
            exhaustive: false,
            seed: Some(42),
        };
        let analyzer = StatisticalAnalyzer::new(&config);
        let randomness = analyzer.analyze_randomness_quality();
        assert!(randomness >= 0.0 && randomness <= 1.0);
    }

    #[test]
    fn test_avalanche_analysis() {
        let config = crate::security::SecurityTestConfig {
            test_iterations: 50,
            max_data_size: 10,
            exhaustive: false,
            seed: Some(42),
        };
        let analyzer = StatisticalAnalyzer::new(&config);
        let avalanche = analyzer.analyze_avalanche_effect();
        assert!(avalanche >= 0.0 && avalanche <= 1.0);
    }
}