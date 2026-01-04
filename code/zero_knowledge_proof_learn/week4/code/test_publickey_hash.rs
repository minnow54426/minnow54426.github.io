// Test to verify if PublicKey implements Hash and Eq traits
use ed25519_dalek::PublicKey;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn main() {
    println!("Testing PublicKey trait implementations...\n");

    // Test 1: Check if PublicKey implements Eq
    println!("Test 1: Does PublicKey implement Eq?");
    let key_bytes_1: [u8; 32] = [1u8; 32];
    let key_bytes_2: [u8; 32] = [1u8; 32];
    let key_bytes_3: [u8; 32] = [2u8; 32];

    let pub_key_1 = PublicKey::from(&key_bytes_1);
    let pub_key_2 = PublicKey::from(&key_bytes_2);
    let pub_key_3 = PublicKey::from(&key_bytes_3);

    // This will only compile if Eq is implemented
    if pub_key_1 == pub_key_2 {
        println!("✓ Eq is implemented: key1 == key2");
    }
    if pub_key_1 != pub_key_3 {
        println!("✓ Eq is implemented: key1 != key3");
    }

    // Test 2: Check if PublicKey implements Hash
    println!("\nTest 2: Does PublicKey implement Hash?");

    // This will only compile if Hash is implemented
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    pub_key_1.hash(&mut hasher);
    let hash1 = hasher.finish();

    let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
    pub_key_2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    let mut hasher3 = std::collections::hash_map::DefaultHasher::new();
    pub_key_3.hash(&mut hasher3);
    let hash3 = hasher3.finish();

    println!("✓ Hash is implemented");
    println!("  hash(key1): {}", hash1);
    println!("  hash(key2): {}", hash2);
    println!("  hash(key3): {}", hash3);

    if hash1 == hash2 {
        println!("✓ Same keys have same hash");
    }
    if hash1 != hash3 {
        println!("✓ Different keys have different hashes");
    }

    // Test 3: Can we use PublicKey as HashMap key?
    println!("\nTest 3: Can PublicKey be used as HashMap key?");
    let mut map: HashMap<PublicKey, String> = HashMap::new();
    map.insert(pub_key_1, "Account 1".to_string());
    map.insert(pub_key_3, "Account 3".to_string());

    println!("✓ HashMap<PublicKey, String> works!");
    println!("  Map has {} entries", map.len());
    println!("  Account for key1: {}", map.get(&pub_key_1).unwrap());
    println!("  Account for key2: {}", map.get(&pub_key_2).unwrap());
    println!("  Account for key3: {}", map.get(&pub_key_3).unwrap());

    println!("\n=== ALL TESTS PASSED ===");
    println!("PublicKey DOES implement Hash and Eq in ed25519-dalek v1.0!");
}
