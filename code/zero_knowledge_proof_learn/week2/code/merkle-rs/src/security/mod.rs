//! Security analysis framework for Merkle trees
//!
//! This module provides comprehensive security testing capabilities including:
//! - Property-based testing for cryptographic security properties
//! - Attack simulation and vulnerability analysis
//! - Statistical analysis of hash function behavior
//! - Formal verification of security properties

pub mod collision_tester;
pub mod binding_property_tester;
pub mod attack_simulator;
pub mod property_based_harness;
pub mod statistical_analysis;
pub mod security_properties;
pub mod advanced_attacks;

pub use collision_tester::CollisionTester;
pub use binding_property_tester::BindingPropertyTester;
pub use attack_simulator::AttackSimulator;
pub use property_based_harness::PropertyBasedHarness;
pub use statistical_analysis::StatisticalAnalyzer;
pub use security_properties::SecurityProperties;
pub use advanced_attacks::{AdvancedAttackSimulator, AdvancedAttackConfig};

/// Configuration for security testing
#[derive(Debug, Clone)]
pub struct SecurityTestConfig {
    /// Number of test iterations for each property
    pub test_iterations: usize,
    /// Maximum size of test data sets
    pub max_data_size: usize,
    /// Whether to run exhaustive tests (may be slow)
    pub exhaustive: bool,
    /// Random seed for reproducible tests
    pub seed: Option<u64>,
}

impl Default for SecurityTestConfig {
    fn default() -> Self {
        Self {
            test_iterations: 1000,
            max_data_size: 1000,
            exhaustive: false,
            seed: None,
        }
    }
}

/// Results from security testing
#[derive(Debug)]
pub struct SecurityTestResults {
    /// Number of tests passed
    pub passed: usize,
    /// Number of tests failed
    pub failed: usize,
    /// Detailed failure information
    pub failures: Vec<String>,
    /// Performance metrics
    pub metrics: SecurityMetrics,
    /// Advanced attack results (if available)
    pub advanced_results: Option<crate::security::advanced_attacks::AdvancedAttackResults>,
}

/// Performance and statistical metrics
#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    /// Average time per test (microseconds)
    pub avg_test_time_us: f64,
    /// Collision resistance score (0-1, higher is better)
    pub collision_resistance: f64,
    /// Binding property score (0-1, higher is better)
    pub binding_strength: f64,
    /// Statistical randomness score (0-1, higher is better)
    pub randomness_quality: f64,
}

/// Main security testing suite
pub struct SecurityTestSuite {
    config: SecurityTestConfig,
    collision_tester: CollisionTester,
    binding_tester: BindingPropertyTester,
    attack_simulator: AttackSimulator,
    harness: PropertyBasedHarness,
    analyzer: StatisticalAnalyzer,
    advanced_attacks: Option<AdvancedAttackSimulator>,
}

impl SecurityTestSuite {
    /// Create a new security test suite with default configuration
    pub fn new() -> Self {
        Self::with_config(SecurityTestConfig::default())
    }

    /// Create a new security test suite with custom configuration
    pub fn with_config(config: SecurityTestConfig) -> Self {
        let advanced_config = AdvancedAttackConfig {
            attack_attempts: config.test_iterations / 2, // Fewer attempts for advanced attacks
            max_budget: config.test_iterations * 10,
            seed: config.seed,
            quantum_simulation: config.exhaustive, // Only quantum if exhaustive testing
        };

        Self {
            collision_tester: CollisionTester::new(&config),
            binding_tester: BindingPropertyTester::new(&config),
            attack_simulator: AttackSimulator::new(&config),
            harness: PropertyBasedHarness::new(&config),
            analyzer: StatisticalAnalyzer::new(&config),
            advanced_attacks: Some(AdvancedAttackSimulator::new(advanced_config)),
            config,
        }
    }

    /// Create a new security test suite with advanced attacks enabled
    pub fn with_advanced_attacks(config: SecurityTestConfig, quantum_enabled: bool) -> Self {
        let advanced_config = AdvancedAttackConfig {
            attack_attempts: config.test_iterations / 2,
            max_budget: config.test_iterations * 10,
            seed: config.seed,
            quantum_simulation: quantum_enabled,
        };

        Self {
            collision_tester: CollisionTester::new(&config),
            binding_tester: BindingPropertyTester::new(&config),
            attack_simulator: AttackSimulator::new(&config),
            harness: PropertyBasedHarness::new(&config),
            analyzer: StatisticalAnalyzer::new(&config),
            advanced_attacks: Some(AdvancedAttackSimulator::new(advanced_config)),
            config,
        }
    }

    /// Run all security tests
    pub fn run_all_tests(&self) -> SecurityTestResults {
        let mut results = SecurityTestResults {
            passed: 0,
            failed: 0,
            failures: Vec::new(),
            metrics: SecurityMetrics {
                avg_test_time_us: 0.0,
                collision_resistance: 0.0,
                binding_strength: 0.0,
                randomness_quality: 0.0,
            },
            advanced_results: None,
        };

        // Run collision resistance tests
        let collision_results = self.collision_tester.run_tests();
        results.passed += collision_results.passed;
        results.failed += collision_results.failed;
        results.failures.extend(collision_results.failures);

        // Run binding property tests
        let binding_results = self.binding_tester.run_tests();
        results.passed += binding_results.passed;
        results.failed += binding_results.failed;
        results.failures.extend(binding_results.failures);

        // Run attack simulations
        let attack_results = self.attack_simulator.run_attacks();
        results.passed += attack_results.passed;
        results.failed += attack_results.failed;
        results.failures.extend(attack_results.failures);

        // Run advanced attacks if available
        if let Some(ref advanced_simulator) = self.advanced_attacks {
            let advanced_results = advanced_simulator.run_advanced_attacks();
            results.advanced_results = Some(advanced_results);
        }

        // Run statistical analysis
        results.metrics = self.analyzer.analyze_properties();

        results
    }

    /// Run only advanced attacks
    pub fn run_advanced_attacks_only(&self) -> Option<crate::security::advanced_attacks::AdvancedAttackResults> {
        self.advanced_attacks.as_ref().map(|simulator| simulator.run_advanced_attacks())
    }

    /// Get current configuration
    pub fn config(&self) -> &SecurityTestConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_suite_creation() {
        let suite = SecurityTestSuite::new();
        assert_eq!(suite.config().test_iterations, 1000);
        assert!(!suite.config().exhaustive);
    }

    #[test]
    fn test_security_suite_with_config() {
        let config = SecurityTestConfig {
            test_iterations: 100,
            max_data_size: 100,
            exhaustive: true,
            seed: Some(42),
        };
        let suite = SecurityTestSuite::with_config(config);
        assert_eq!(suite.config().test_iterations, 100);
        assert!(suite.config().exhaustive);
        assert_eq!(suite.config().seed, Some(42));
    }
}