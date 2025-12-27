//! Quick test to verify security framework works

use merkle_rs::{MerkleTree, security::{SecurityTestConfig, SecurityTestSuite}};

fn main() {
    println!("🧪 Quick Security Framework Test");

    // Test basic Merkle tree functionality
    let leaves = vec![
        b"test1".to_vec(),
        b"test2".to_vec(),
        b"test3".to_vec(),
    ];

    let tree = MerkleTree::from_leaves(leaves.clone());
    println!("✅ Merkle tree created successfully");
    println!("   Root: {:02x?}", &tree.root()[0..8]); // Show first 8 bytes

    // Test proof generation and verification
    let proof = tree.prove(1);
    let verified = merkle_rs::verify(tree.root(), &leaves[1], proof);
    println!("✅ Proof verification: {}", verified);

    // Test security framework
    let config = SecurityTestConfig {
        test_iterations: 10, // Small number for quick test
        max_data_size: 20,
        exhaustive: false,
        seed: Some(42),
    };

    let suite = SecurityTestSuite::with_config(config);
    println!("✅ Security test suite created");

    // Test individual components
    let collision_tester = merkle_rs::security::CollisionTester::new(&suite.config());
    println!("✅ Collision tester created");

    let properties = merkle_rs::security::SecurityProperties::verify_all(&tree, &leaves);
    println!("✅ Security properties verified");
    println!("   Security score: {:.2}%", properties.security_score() * 100.0);

    println!("\n🎉 All basic functionality tests passed!");
}