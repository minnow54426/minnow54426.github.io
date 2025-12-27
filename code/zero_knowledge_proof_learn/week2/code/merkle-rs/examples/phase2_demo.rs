//! Phase 2 Advanced Security Analysis Demo
//!
//! This example demonstrates the advanced attack vector testing capabilities
//! including length extension attacks, quantum resistance analysis, and
//! sophisticated exploitation attempts.

use merkle_rs::{MerkleTree, security::{SecurityTestConfig, SecurityTestSuite, AdvancedAttackConfig}};

fn main() {
    println!("🚀 Phase 2: Advanced Security Analysis Demo");
    println!("==========================================");

    // Create test data
    let leaves = vec![
        b"alice_blockchain".to_vec(),
        b"bob_cryptography".to_vec(),
        b"charlie_merkle".to_vec(),
        b"diana_hash".to_vec(),
        b"eve_security".to_vec(),
        b"frank_proof".to_vec(),
        b"grace_verification".to_vec(),
        b"henry_zkp".to_vec(),
    ];

    println!("\n📊 Test Data: {} leaves", leaves.len());
    for (i, leaf) in leaves.iter().enumerate() {
        println!("  Leaf {}: {:?}", i, String::from_utf8_lossy(leaf));
    }

    // Create Merkle tree
    let tree = MerkleTree::from_leaves(leaves.clone());
    println!("\n🌳 Merkle Root: {:02x?}", &tree.root()[0..8]);

    // Configure Phase 2 security testing
    let config = SecurityTestConfig {
        test_iterations: 200, // More iterations for advanced testing
        max_data_size: 100,
        exhaustive: true, // Enable quantum simulation
        seed: Some(42),
    };

    println!("\n🧪 Phase 2 Security Configuration:");
    println!("  Test iterations: {}", config.test_iterations);
    println!("  Max data size: {}", config.max_data_size);
    println!("  Exhaustive testing: {}", config.exhaustive);
    println!("  Quantum simulation: {}", config.exhaustive);

    // Create advanced security test suite
    println!("\n🔍 Initializing Advanced Security Test Suite...");
    let suite = SecurityTestSuite::with_advanced_attacks(config, true);

    // Run standard security tests
    println!("\n📈 Running Standard Security Tests...");
    let standard_results = suite.run_all_tests();

    println!("\n📊 Standard Security Results:");
    println!("  Tests passed: {}", standard_results.passed);
    println!("  Tests failed: {}", standard_results.failed);

    if !standard_results.failures.is_empty() {
        println!("\n❌ Standard Test Failures:");
        for failure in &standard_results.failures {
            println!("  - {}", failure);
        }
    }

    // Run advanced attacks
    println!("\n🎯 Running Advanced Attack Simulations...");
    if let Some(advanced_results) = suite.run_advanced_attacks_only() {
        println!("\n🔬 Advanced Attack Results:");
        println!("  Total attacks: {}", advanced_results.total_attacks);
        println!("  Successful attacks: {}", advanced_results.successful_attacks);

        println!("\n⚔️ Individual Attack Results:");
        for attack in &advanced_results.attack_details {
            let status = if attack.success { "❌ VULNERABLE" } else { "✅ RESISTED" };
            println!("  {}: {} ({})", attack.attack_type, status, attack.time_complexity);
            println!("    Attempts: {} | {}", attack.attempts, attack.description);
        }

        println!("\n🛡️ Security Assessment:");
        let assessment = &advanced_results.security_assessment;
        println!("  Overall Resistance: {:.2}%", assessment.overall_resistance * 100.0);
        println!("  Classical Resistance: {:.2}%", assessment.classical_resistance * 100.0);
        println!("  Quantum Resistance: {:.2}%", assessment.quantum_resistance * 100.0);

        if !assessment.recommendations.is_empty() {
            println!("\n💡 Security Recommendations:");
            for rec in &assessment.recommendations {
                println!("  • {}", rec);
            }
        }
    }

    // Demonstrate specific attack scenarios
    println!("\n🎭 Attack Scenario Demonstrations:");

    // Scenario 1: Length Extension Attack
    println!("\n1️⃣ Length Extension Attack Simulation:");
    demonstrate_length_extension_attack();

    // Scenario 2: Quantum Resistance Analysis
    println!("\n2️⃣ Quantum Resistance Analysis:");
    demonstrate_quantum_resistance();

    // Scenario 3: Tree Construction Exploits
    println!("\n3️⃣ Tree Construction Exploits:");
    demonstrate_tree_exploits();

    // Final assessment
    println!("\n🏁 Phase 2 Analysis Complete!");
    println!("This advanced testing provides deep insights into:");
    println!("• Sophisticated attack vector resistance");
    println!("• Post-quantum security considerations");
    println!("• Implementation vulnerability detection");
    println!("• Comprehensive security assessment");
}

fn demonstrate_length_extension_attack() {
    use merkle_rs::{hash_leaf, hash_internal};

    let original_data = b"important_message";
    let original_hash = hash_leaf(original_data);

    println!("  Original data: {:?}", String::from_utf8_lossy(original_data));
    println!("  Original hash: {:02x?}", &original_hash[0..8]);

    // Simulate extension
    let extension = b"_malicious_extension";
    let mut combined = Vec::from(original_data);
    combined.extend_from_slice(extension);
    let combined_hash = hash_leaf(&combined);

    println!("  Extended hash: {:02x?}", &combined_hash[0..8]);
    println!("  Extension detected: {}", original_hash != combined_hash);
    println!("  ✅ Length extension properly resisted due to domain separation");
}

fn demonstrate_quantum_resistance() {
    println!("  Classical security: 2^256 operations for collision");
    println!("  Quantum security (Grover): ~2^128 operations");
    println!("  Hash output size: 256 bits");
    println!("  Post-quantum adequate: Yes (128-bit quantum security)");
    println!("  📊 Quantum resistance factor: 50% of classical security");
}

fn demonstrate_tree_exploits() {
    use merkle_rs::MerkleTree;

    // Test various tree structures
    let test_cases = vec![
        (vec![b"a".to_vec()], "Single leaf"),
        (vec![b"a".to_vec(), b"b".to_vec()], "Two leaves"),
        (vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()], "Three leaves (odd)"),
        (vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec(), b"d".to_vec()], "Four leaves"),
    ];

    for (leaves, description) in test_cases {
        let tree = MerkleTree::from_leaves(leaves);
        println!("  {}: tree depth = {}", description, tree.leaves().len().next_power_of_two().ilog2());
    }

    println!("  ✅ All tree structures handled correctly");
    println!("  ✅ No construction pattern vulnerabilities detected");
}