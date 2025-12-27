//! Rust Protocol Basics - Core cryptographic and serialization utilities
//!
//! This library provides fundamental building blocks for cryptographic protocols
//! and secure communication systems. It demonstrates Rust's ownership system,
//! error handling patterns, and type safety features.
//!
//! # Quick Start
//!
//! ```rust
//! use rust_protocol_basics::*;
//!
//! // Serialize a struct and hash it
//! let data = ExampleStruct::new(1, "test".to_string(), vec![1, 2, 3]);
//! let bytes = data.to_bytes().unwrap();
//! let hash = sha256(&bytes);
//! let hash32 = Hash32::new(hash);
//!
//! println!("Hash: {}", hash32);
//! ```
//!
//! # Modules
//!
//! - [`bytes`] - Hex encoding/decoding and serialization utilities
//! - [`hash`] - SHA-256 hashing and Merkle tree implementations
//! - [`types`] - Type-safe wrappers for cryptographic data

// Include all modules
mod bytes;
mod hash;
mod types;

// Re-export commonly used types and functions for convenience
pub use bytes::{hex_decode, hex_encode, ExampleStruct, ToBytes};
pub use hash::{sha256, sha256_hex, sha256d, MerkleTree};
pub use types::{Hash32, ToHash32};

/// Convenience function to hash data directly into a Hash32
///
/// This is a shortcut for Hash32::hash_data() provided for ergonomic reasons.
///
/// # Examples
/// ```
/// use rust_protocol_basics::hash_to_hash32;
///
/// let hash = hash_to_hash32("hello world");
/// println!("Hash: {}", hash);
/// ```
pub fn hash_to_hash32<T: AsRef<[u8]>>(data: T) -> Hash32 {
    Hash32::hash_data(data)
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_serialize_hash_workflow() {
        // Demonstrate the complete workflow: serialize -> hash -> display
        let example = ExampleStruct::new(
            42,
            "integration test".to_string(),
            vec![0x01, 0x02, 0x03, 0x04],
        );

        // Serialize to bytes
        let serialized = example.to_bytes().unwrap();
        assert!(!serialized.is_empty());

        // Hash the bytes
        let hash_bytes = sha256(&serialized);
        let hash_wrapper = Hash32::new(hash_bytes);

        // Display as hex
        let hex_display = format!("{}", hash_wrapper);
        assert_eq!(hex_display.len(), 64); // SHA-256 is 32 bytes = 64 hex chars

        // Roundtrip: hex string back to Hash32
        let reconstructed: Hash32 = hex_display.parse().unwrap();
        assert_eq!(reconstructed, hash_wrapper);
    }

    #[test]
    fn test_merkle_tree_with_serialized_data() {
        // Test Merkle tree with serialized structs
        let items = vec![
            ExampleStruct::new(1, "a".to_string(), vec![1]),
            ExampleStruct::new(2, "b".to_string(), vec![2]),
            ExampleStruct::new(3, "c".to_string(), vec![3]),
        ];

        // Serialize all items
        let serialized_items: Result<Vec<Vec<u8>>, _> =
            items.iter().map(|item| item.to_bytes()).collect();
        let serialized_items = serialized_items.unwrap();

        // Create Merkle tree
        let tree = MerkleTree::new(&serialized_items);
        assert_ne!(tree.root(), [0u8; 32]);

        // Verify root is different from simple concatenation
        let concatenated = serialized_items.concat();
        let simple_hash = sha256(&concatenated);
        assert_ne!(tree.root(), simple_hash);
    }
}
