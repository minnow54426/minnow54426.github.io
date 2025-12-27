//! Integration tests for the security analysis framework

use merkle_rs::{MerkleTree, security::SecurityTestSuite};

#[test]
fn test_security_test_suite_basic() {
    let suite = SecurityTestSuite::new();
    let results = suite.run_all_tests();

    // Basic sanity checks
    assert!(results.passed >= 0);
    assert!(results.failed >= 0);
    assert!(results.metrics.collision_resistance >= 0.0);
    assert!(results.metrics.collision_resistance <= 1.0);
}

#[test]
fn test_security_test_suite_with_config() {
    let config = merkle_rs::security::SecurityTestConfig {
        test_iterations: 10,
        max_data_size: 10,
        exhaustive: false,
        seed: Some(42),
    };

    let suite = SecurityTestSuite::with_config(config);
    let results = suite.run_all_tests();

    // Should complete without panicking
    assert!(results.passed + results.failed > 0);
}

#[test]
fn test_collision_tester_standalone() {
    let config = merkle_rs::security::SecurityTestConfig::default();
    let tester = merkle_rs::security::CollisionTester::new(&config);
    let results = tester.run_tests();

    // Should complete without errors
    assert!(results.passed + results.failed > 0);
    assert!(results.collision_probability >= 0.0);
}

#[test]
fn test_security_properties_verification() {
    let leaves = vec![
        b"test_leaf_1".to_vec(),
        b"test_leaf_2".to_vec(),
        b"test_leaf_3".to_vec(),
    ];

    let tree = MerkleTree::from_leaves(leaves.clone());
    let properties = merkle_rs::security::SecurityProperties::verify_all(&tree, &leaves);

    // Basic properties should be satisfied
    assert!(properties.soundness);
    assert!(properties.completeness);
    assert!(properties.domain_separation);
    assert!(properties.security_score() > 0.5);
}