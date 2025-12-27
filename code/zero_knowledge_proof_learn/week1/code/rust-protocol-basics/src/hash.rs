/// Hash module - provides SHA-256 hashing functionality
///
/// This module demonstrates:
/// - Working with cryptographic hash functions
/// - Using the sha2 crate for SHA-256 operations
/// - Converting between different byte representations
/// - Cryptographic best practices
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::convert::AsRef;

/// Compute SHA-256 hash of the given data
///
/// This function takes any type that can be referenced as a byte slice
/// and returns its SHA-256 hash as a 32-byte array.
///
/// # Arguments
/// * `data` - The data to hash, anything that implements AsRef<[u8]>
///
/// # Returns
/// A 32-byte array containing the SHA-256 hash
///
/// # Examples
/// ```
/// use rust_protocol_basics::sha256;
///
/// let data = b"hello world";
/// let hash = sha256(data);
/// // hash is now [u8; 32] containing the SHA-256 of "hello world"
/// ```
///
/// # Security Notes
/// - SHA-256 is considered secure for collision resistance as of current knowledge
/// - This function does not add any salt - use HMAC for authenticated hashing
/// - The output is deterministic: same input always produces same output
pub fn sha256<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    // Convert GenericArray to fixed-size array
    result.into()
}

/// Compute SHA-256 hash and return as hex string
///
/// This is a convenience function that combines sha256() with hex encoding
/// for easier display and debugging.
///
/// # Arguments
/// * `data` - The data to hash
///
/// # Returns
/// A hex-encoded string representation of the SHA-256 hash
///
/// # Examples
/// ```
/// use rust_protocol_basics::sha256_hex;
///
/// let data = b"hello";
/// let hex_hash = sha256_hex(data).unwrap();
/// assert_eq!(hex_hash, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
/// ```
pub fn sha256_hex<T: AsRef<[u8]>>(data: T) -> Result<String> {
    let hash_bytes = sha256(data);
    Ok(hex::encode(hash_bytes))
}

/// Compute double SHA-256 (hash of hash)
///
/// This is commonly used in Bitcoin and other cryptocurrencies
/// for additional security against length-extension attacks.
///
/// # Arguments
/// * `data` - The data to hash twice
///
/// # Returns
/// A 32-byte array containing the double SHA-256 hash
pub fn sha256d<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let first_hash = sha256(data);
    sha256(first_hash)
}

/// Merkle tree implementation for efficient hash verification
///
/// A Merkle tree allows efficient verification of large data sets
/// by computing a single root hash from multiple leaf hashes.
pub struct MerkleTree {
    root: [u8; 32],
    leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    /// Create a new Merkle tree from the given data items
    ///
    /// # Arguments
    /// * `items` - A slice of byte slices to include in the tree
    ///
    /// # Returns
    /// A new MerkleTree instance
    pub fn new<T: AsRef<[u8]>>(items: &[T]) -> Self {
        let leaves: Vec<[u8; 32]> = items.iter().map(sha256).collect();

        let root = Self::compute_merkle_root(&leaves);

        Self { root, leaves }
    }

    /// Get the Merkle root hash
    pub fn root(&self) -> [u8; 32] {
        self.root
    }

    /// Get the number of leaves in the tree
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    /// Validate the tree integrity by recomputing the root
    ///
    /// This method verifies that the stored root hash matches the computed
    /// root hash from the current leaves. Useful for detecting corruption.
    ///
    /// # Returns
    /// true if the tree is valid, false otherwise
    pub fn validate(&self) -> bool {
        let computed_root = Self::compute_merkle_root(&self.leaves);
        computed_root == self.root
    }

    /// Compute the Merkle root from a set of leaf hashes
    ///
    /// This recursively pairs up hashes and hashes them together
    /// until only a single root hash remains.
    fn compute_merkle_root(hashes: &[[u8; 32]]) -> [u8; 32] {
        if hashes.len() == 1 {
            return hashes[0];
        }

        if hashes.is_empty() {
            return [0u8; 32]; // Edge case: empty tree
        }

        // Pair up hashes and hash each pair
        // Pre-allocate with exact capacity needed to avoid reallocations
        let mut next_level = Vec::with_capacity((hashes.len() + 1) / 2);
        for chunk in hashes.chunks(2) {
            let combined = match chunk {
                [hash1] => {
                    // Odd number of hashes - duplicate the last one
                    // Use stack allocation to avoid heap allocation
                    let mut temp = [0u8; 64];
                    temp[..32].copy_from_slice(hash1);
                    temp[32..].copy_from_slice(hash1);
                    sha256(temp)
                }
                [hash1, hash2] => {
                    // Use stack allocation for better performance
                    let mut temp = [0u8; 64];
                    temp[..32].copy_from_slice(hash1);
                    temp[32..].copy_from_slice(hash2);
                    sha256(temp)
                }
                _ => unreachable!(), // chunks(2) never yields more than 2 items
            };
            next_level.push(combined);
        }

        Self::compute_merkle_root(&next_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_basic() {
        // Test known SHA-256 vectors
        let data = b"hello world";
        let hash = sha256(data);

        // Known SHA-256 of "hello world"
        let expected = [
            0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08, 0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d,
            0xab, 0xfa, 0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee, 0x90, 0x88, 0xf7, 0xac,
            0xe2, 0xef, 0xcd, 0xe9,
        ];

        assert_eq!(hash, expected);
    }

    #[test]
    fn test_sha256_empty() {
        // Test SHA-256 of empty string
        let data = b"";
        let hash = sha256(data);

        // Known SHA-256 of empty string
        let expected = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];

        assert_eq!(hash, expected);
    }

    #[test]
    fn test_sha256_different_inputs() {
        // Test that different inputs produce different hashes
        let data1 = b"hello";
        let data2 = b"hello ";
        let data3 = b"Hello";

        let hash1 = sha256(data1);
        let hash2 = sha256(data2);
        let hash3 = sha256(data3);

        assert_ne!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_ne!(hash2, hash3);
    }

    #[test]
    fn test_sha256_deterministic() {
        // Test that same input always produces same output
        let data = b"deterministic test";
        let hash1 = sha256(data);
        let hash2 = sha256(data);
        let hash3 = sha256(data.to_vec()); // Different type, same content

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
    }

    #[test]
    fn test_sha256_hex() {
        // Test hex encoding functionality
        let data = b"hello";
        let hex_hash = sha256_hex(data).unwrap();

        // Known hex SHA-256 of "hello"
        assert_eq!(
            hex_hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha256d() {
        // Test double SHA-256
        let data = b"double hash test";
        let single = sha256(data);
        let double = sha256d(data);
        let expected = sha256(single);

        assert_eq!(double, expected);
    }

    #[test]
    fn test_merkle_tree_basic() {
        // Test basic Merkle tree construction
        let items = vec![b"item1", b"item2", b"item3", b"item4"];
        let tree = MerkleTree::new(&items);

        assert_eq!(tree.leaf_count(), 4);

        // Verify root is not just a simple hash of all items
        let concatenated = b"item1item2item3item4";
        let simple_hash = sha256(concatenated);
        assert_ne!(tree.root(), simple_hash);
    }

    #[test]
    fn test_merkle_tree_single_item() {
        // Test Merkle tree with single item
        let items = vec![b"single"];
        let tree = MerkleTree::new(&items);

        assert_eq!(tree.leaf_count(), 1);
        assert_eq!(tree.root(), sha256(b"single"));
    }

    #[test]
    fn test_merkle_tree_empty() {
        // Test Merkle tree with no items
        let items: Vec<&[u8]> = vec![];
        let tree = MerkleTree::new(&items);

        assert_eq!(tree.leaf_count(), 0);
        assert_eq!(tree.root(), [0u8; 32]);
    }

    #[test]
    fn test_merkle_tree_odd_number() {
        // Test Merkle tree with odd number of items
        let items = vec![b"item1", b"item2", b"item3"];
        let tree = MerkleTree::new(&items);

        assert_eq!(tree.leaf_count(), 3);
        assert_ne!(tree.root(), [0u8; 32]); // Should not be empty
    }

    #[test]
    fn test_hash_large_data() {
        // Test hashing larger data
        let large_data = vec![0u8; 10000]; // 10KB of zeros
        let hash = sha256(&large_data);

        // Verify it's not all zeros (hash of zeros is not zeros)
        assert_ne!(hash, [0u8; 32]);

        // Verify deterministic
        let hash2 = sha256(&large_data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_merkle_tree_validation() {
        // Test tree validation
        let items = vec![b"item1", b"item2", b"item3", b"item4"];
        let tree = MerkleTree::new(&items);

        // Valid tree should pass validation
        assert!(tree.validate());

        // Test with different data (should still be valid)
        let items2 = vec![b"a", b"b", b"c"];
        let tree2 = MerkleTree::new(&items2);
        assert!(tree2.validate());

        // Empty tree validation
        let empty_items: Vec<&[u8]> = vec![];
        let empty_tree = MerkleTree::new(&empty_items);
        assert!(empty_tree.validate());
    }

    #[test]
    fn test_hash_with_string_input() {
        // Test that String input works (not just &[u8])
        let string_data = "test string".to_string();
        let hash1 = sha256(&string_data);
        let hash2 = sha256(b"test string");

        assert_eq!(hash1, hash2);
    }
}
