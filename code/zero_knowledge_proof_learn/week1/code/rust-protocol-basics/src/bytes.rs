/// Bytes module - provides hex encoding/decoding and serialization utilities
///
/// This module demonstrates:
/// - Working with byte arrays and slices
/// - Hex encoding/decoding using the hex crate
/// - Serialization with bincode
/// - Error handling with anyhow
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Hex encode a byte slice to a string
///
/// # Arguments
/// * `data` - The byte slice to encode
///
/// # Returns
/// A hex-encoded string representation of the input bytes
///
/// # Examples
/// ```
/// use rust_protocol_basics::hex_encode;
///
/// let data = b"hello";
/// let hex_str = hex_encode(data).unwrap();
/// assert_eq!(hex_str, "68656c6c6f");
/// ```
pub fn hex_encode(data: &[u8]) -> Result<String> {
    Ok(hex::encode(data))
}

/// Hex decode a string back to bytes
///
/// # Arguments
/// * `hex_str` - The hex string to decode
///
/// # Returns
/// A vector of bytes decoded from the hex string
///
/// # Examples
/// ```
/// use rust_protocol_basics::hex_decode;
///
/// let hex_str = "68656c6c6f";
/// let data = hex_decode(hex_str).unwrap();
/// assert_eq!(data, b"hello");
/// ```
pub fn hex_decode(hex_str: &str) -> Result<Vec<u8>> {
    hex::decode(hex_str)
        .context("Failed to decode hex string - ensure it contains valid hex characters")
}

/// Trait to convert any serializable struct to bytes using bincode
///
/// This trait provides a convenient method to serialize any type that implements
/// serde::Serialize into a byte vector using bincode for deterministic binary encoding.
pub trait ToBytes {
    /// Convert the implementing type to a byte vector
    ///
    /// # Returns
    /// A serialized byte representation of the object
    ///
    /// # Errors
    /// Returns an error if serialization fails
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

/// Blanket implementation for all types that implement Serialize
impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).context("Failed to serialize object to bytes using bincode")
    }
}

/// Example struct demonstrating serialization capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleStruct {
    pub id: u32,
    pub name: String,
    pub data: Vec<u8>,
}

impl ExampleStruct {
    /// Create a new example struct
    pub fn new(id: u32, name: String, data: Vec<u8>) -> Self {
        Self { id, name, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_encode() {
        // Test basic hex encoding
        let data = b"hello world";
        let result = hex_encode(data).unwrap();
        assert_eq!(result, "68656c6c6f20776f726c64");

        // Test empty input
        let empty = b"";
        let result = hex_encode(empty).unwrap();
        assert_eq!(result, "");

        // Test single byte
        let single = b"\xff";
        let result = hex_encode(single).unwrap();
        assert_eq!(result, "ff");
    }

    #[test]
    fn test_hex_decode() {
        // Test basic hex decoding
        let hex_str = "68656c6c6f20776f726c64";
        let result = hex_decode(hex_str).unwrap();
        assert_eq!(result, b"hello world");

        // Test empty string
        let empty = "";
        let result = hex_decode(empty).unwrap();
        assert_eq!(result, b"");

        // Test odd length hex string (should fail)
        let odd = "abc";
        assert!(hex_decode(odd).is_err());

        // Test invalid hex characters (should fail)
        let invalid = "xyz";
        assert!(hex_decode(invalid).is_err());
    }

    #[test]
    fn test_hex_roundtrip() {
        // Test that encode -> decode preserves original data
        let original = b"The quick brown fox jumps over the lazy dog";
        let hex_str = hex_encode(original).unwrap();
        let decoded = hex_decode(&hex_str).unwrap();
        assert_eq!(original, decoded.as_slice());
    }

    #[test]
    fn test_to_bytes_trait() {
        // Test struct serialization
        let example = ExampleStruct::new(42, "test".to_string(), vec![1, 2, 3, 4, 5]);

        let bytes = example.to_bytes().unwrap();
        assert!(!bytes.is_empty());

        // Verify the bytes are deterministic (same input produces same output)
        let bytes2 = example.to_bytes().unwrap();
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn test_serialization_with_different_types() {
        // Test different primitive types
        let num: u32 = 12345;
        let bytes = num.to_bytes().unwrap();
        assert_eq!(bytes.len(), 4); // u32 should be 4 bytes

        let string = "hello".to_string();
        let bytes = string.to_bytes().unwrap();
        assert!(!bytes.is_empty());

        let vec_data = vec![1u8, 2u8, 3u8];
        let bytes = vec_data.to_bytes().unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_example_struct_creation() {
        let data = vec![0x01, 0x02, 0x03];
        let example = ExampleStruct::new(100, "test_struct".to_string(), data.clone());

        assert_eq!(example.id, 100);
        assert_eq!(example.name, "test_struct");
        assert_eq!(example.data, data);
    }
}
