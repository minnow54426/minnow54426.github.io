//! Security Analysis Framework Demo
//!
//! This example demonstrates the comprehensive security analysis capabilities
//! of the Merkle tree security framework.

use merkle_rs::{MerkleTree, security::{SecurityTestSuite, SecurityTestConfig}};

fn main() {
    println!("🔐 Merkle Tree Security Analysis Framework Demo");
    println!("=============================================");

    // Create test data
    let leaves = vec![
        b"alice".to_vec(),
        b"bob".to_vec(),
        b"charlie".to_vec(),
        b"diana".to_vec(),
        b"eve".to_vec(),
    ];

    println!("\n📊 Test Data: {} leaves", leaves.len());
    for (i, leaf) in leaves.iter().enumerate() {
        println!("  Leaf {}: {:?}", i, String::from_utf8_lossy(leaf));
    }

    // Create Merkle tree
    let tree = MerkleTree::from_leaves(leaves.clone());
    println!("\n🌳 Merkle Root: {:02x?}", tree.root());

    // Configure security testing
    let config = SecurityTestConfig {
        test_iterations: 100,
        max_data_size: 50,
        exhaustive: false,
        seed: Some(42), // Reproducible results
    };

    println!("\n🧪 Security Configuration:");
    println!("  Test iterations: {}", config.test_iterations);
    println!("  Max data size: {}", config.max_data_size);
    println!("  Random seed: {:?}", config.seed);

    // Run security analysis
    println!("\n🔍 Running Security Analysis...");
    let suite = SecurityTestSuite::with_config(config);
    let results = suite.run_all_tests();

    // Display results
    println!("\n📈 Security Analysis Results:");
    println!("  Tests passed: {}", results.passed);
    println!("  Tests failed: {}", results.failed);

    if !results.failures.is_empty() {
        println!("\n❌ Failures:");
        for failure in &results.failures {
            println!("  - {}", failure);
        }
    }

    println!("\n📊 Security Metrics:");
    println!("  Collision Resistance: {:.2}%", results.metrics.collision_resistance * 100.0);
    println!("  Binding Strength: {:.2}%", results.metrics.binding_strength * 100.0);
    println!("  Randomness Quality: {:.2}%", results.metrics.randomness_quality * 100.0);

    // Demonstrate basic functionality
    println!("\n🔧 Basic Functionality Demo:");

    // Generate and verify a proof
    let leaf_index = 2; // charlie
    let proof = tree.prove(leaf_index);
    println!("  Generated proof for leaf {}: {} siblings", leaf_index, proof.siblings.len());

    let verification_result = merkle_rs::verify(tree.root(), &leaves[leaf_index], proof);
    println!("  Proof verification: {}", if verification_result { "✅ Valid" } else { "❌ Invalid" });

    // Test with tampered data
    let tampered_leaf = b"tampered".to_vec();
    let proof = tree.prove(leaf_index);
    let tampered_result = merkle_rs::verify(tree.root(), &tampered_leaf, proof);
    println!("  Tampered leaf verification: {}", if tampered_result { "❌ Unexpectedly valid" } else { "✅ Correctly rejected" });

    println!("\n🎯 Security Analysis Complete!");
    println!("This framework provides comprehensive cryptographic security testing");
    println!("for educational purposes and deeper understanding of Merkle tree properties.");
}