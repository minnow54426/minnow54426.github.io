use anyhow::{anyhow, Result};
/// Types module - defines custom types for cryptographic operations
///
/// This module demonstrates:
/// - Creating newtype wrappers for type safety
/// - Implementing custom Display and Debug traits
/// - Working with fixed-size arrays
/// - Type conversions and validation
use std::fmt;
use std::str::FromStr;

/// A newtype wrapper for 32-byte hashes
///
/// This provides type safety to distinguish between regular byte arrays
/// and cryptographic hashes. It includes methods for hex display and
/// conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Hash32([u8; 32]);

impl Hash32 {
    /// Create a new Hash32 from a byte array
    ///
    /// # Arguments
    /// * `bytes` - A 32-byte array
    ///
    /// # Returns
    /// A new Hash32 instance
    ///
    /// # Examples
    /// ```
    /// use rust_protocol_basics::Hash32;
    ///
    /// let bytes = [0u8; 32];
    /// let hash = Hash32::new(bytes);
    /// ```
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create a Hash32 from a slice (must be exactly 32 bytes)
    ///
    /// # Arguments
    /// * `slice` - A byte slice of exactly 32 bytes
    ///
    /// # Returns
    /// Result containing the Hash32 or an error if slice length is wrong
    ///
    /// # Examples
    /// ```
    /// use rust_protocol_basics::Hash32;
    ///
    /// let slice = &[0u8; 32];
    /// let hash = Hash32::from_slice(slice).unwrap();
    /// ```
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() != 32 {
            return Err(anyhow!(
                "Hash32 requires exactly 32 bytes, got {}",
                slice.len()
            ));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Get the underlying byte array
    ///
    /// # Returns
    /// A reference to the 32-byte array
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Get the underlying byte array as mutable reference
    ///
    /// # Returns
    /// A mutable reference to the 32-byte array
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }

    /// Consume the Hash32 and return the inner byte array
    ///
    /// # Returns
    /// The 32-byte array
    pub fn into_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Create a Hash32 from a hex string
    ///
    /// # Arguments
    /// * `hex_str` - A hex string of exactly 64 characters
    ///
    /// # Returns
    /// Result containing the Hash32 or an error if hex is invalid
    ///
    /// # Examples
    /// ```
    /// use rust_protocol_basics::Hash32;
    ///
    /// let hex = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    /// let hash = Hash32::from_hex(hex).unwrap();
    /// ```
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str).map_err(|e| anyhow!("Invalid hex string: {}", e))?;

        Self::from_slice(&bytes)
    }

    /// Create a Hash32 by hashing data directly
    ///
    /// This is a convenience method that hashes the input data and returns
    /// a Hash32 without needing intermediate allocations.
    ///
    /// # Arguments
    /// * `data` - The data to hash
    ///
    /// # Returns
    /// A new Hash32 containing the SHA-256 hash of the data
    ///
    /// # Examples
    /// ```
    /// use rust_protocol_basics::Hash32;
    ///
    /// let hash = Hash32::hash_data("hello world");
    /// println!("Hash: {}", hash);
    /// ```
    pub fn hash_data<T: AsRef<[u8]>>(data: T) -> Self {
        Self(crate::hash::sha256(data))
    }

    /// Check if the hash is all zeros
    ///
    /// # Returns
    /// true if all 32 bytes are zero, false otherwise
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    /// Count the number of leading zero bytes
    ///
    /// This is useful for proof-of-work calculations where you might
    /// want to count leading zeros as a measure of difficulty.
    ///
    /// # Returns
    /// The number of consecutive zero bytes from the start
    pub fn leading_zero_bytes(&self) -> usize {
        self.0.iter().take_while(|&&b| b == 0).count()
    }

    /// Count the number of leading zero bits
    ///
    /// More precise than leading_zero_bytes as it counts individual bits.
    ///
    /// # Returns
    /// The number of consecutive zero bits from the start
    pub fn leading_zero_bits(&self) -> usize {
        // Fast path for all zeros
        if self.0 == [0u8; 32] {
            return 256;
        }

        // Use iterator::position to find first non-zero byte more efficiently
        if let Some(first_non_zero) = self.0.iter().position(|&b| b != 0) {
            let zero_bytes = first_non_zero;
            let zero_bits_in_byte = self.0[first_non_zero].leading_zeros() as usize;
            zero_bytes * 8 + zero_bits_in_byte
        } else {
            256 // All bytes are zero
        }
    }
}

impl fmt::Display for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use hex::encode which is optimized for this exact use case
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::LowerHex for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::UpperHex for Hash32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0).to_uppercase())
    }
}

impl FromStr for Hash32 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_hex(s)
    }
}

impl From<[u8; 32]> for Hash32 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<Hash32> for [u8; 32] {
    fn from(hash: Hash32) -> Self {
        hash.0
    }
}

impl AsRef<[u8]> for Hash32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for Hash32 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

/// A trait for types that can be converted to Hash32
///
/// This is useful for creating hashes from various data types.
pub trait ToHash32 {
    /// Convert the type to a Hash32
    ///
    /// # Returns
    /// A Hash32 representation of the data
    fn to_hash32(&self) -> Hash32;
}

/// Implement ToHash32 for any byte slice
impl<T: AsRef<[u8]>> ToHash32 for T {
    fn to_hash32(&self) -> Hash32 {
        Hash32::hash_data(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash32_creation() {
        // Test creating from byte array
        let bytes = [1u8; 32];
        let hash = Hash32::new(bytes);
        assert_eq!(hash.as_bytes(), &[1u8; 32]);

        // Test creating from slice
        let slice = &[2u8; 32];
        let hash2 = Hash32::from_slice(slice).unwrap();
        assert_eq!(hash2.as_bytes(), &[2u8; 32]);
    }

    #[test]
    fn test_hash32_from_invalid_slice() {
        // Test with too short slice
        let short = &[1u8; 31];
        assert!(Hash32::from_slice(short).is_err());

        // Test with too long slice
        let long = &[1u8; 33];
        assert!(Hash32::from_slice(long).is_err());
    }

    #[test]
    fn test_hash32_display() {
        // Test display format (lowercase hex)
        let bytes = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];
        let hash = Hash32::new(bytes);

        let display_str = format!("{}", hash);
        assert_eq!(
            display_str,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hash32_hex_formatting() {
        let hash = Hash32::new([
            0xff, 0x00, 0xab, 0xcd, 0xff, 0x00, 0xab, 0xcd, 0xff, 0x00, 0xab, 0xcd, 0xff, 0x00,
            0xab, 0xcd, 0xff, 0x00, 0xab, 0xcd, 0xff, 0x00, 0xab, 0xcd, 0xff, 0x00, 0xab, 0xcd,
            0xff, 0x00, 0xab, 0xcd,
        ]);

        // Test lowercase hex
        let lower = format!("{:x}", hash);
        assert_eq!(lower, lower.to_lowercase());

        // Test uppercase hex
        let upper = format!("{:X}", hash);
        assert_eq!(upper, upper.to_uppercase());
    }

    #[test]
    fn test_hash32_from_hex() {
        // Test valid hex string
        let hex_str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hash = Hash32::from_hex(hex_str).unwrap();

        // Verify the bytes are correct
        let expected = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(hash.as_bytes(), &expected);

        // Test invalid hex string
        assert!(Hash32::from_hex("invalid").is_err());
        assert!(Hash32::from_hex("abc").is_err()); // Too short
    }

    #[test]
    fn test_hash32_from_str() {
        // Test FromStr trait implementation
        let hex_str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hash: Hash32 = hex_str.parse().unwrap();

        let expected = Hash32::from_hex(hex_str).unwrap();
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_hash32_conversions() {
        // Test From<[u8; 32]>
        let bytes = [5u8; 32];
        let hash1: Hash32 = bytes.into();
        assert_eq!(hash1.as_bytes(), &bytes);

        // Test Into<[u8; 32]>
        let hash2 = Hash32::new([6u8; 32]);
        let bytes2: [u8; 32] = hash2.into();
        assert_eq!(bytes2, [6u8; 32]);
    }

    #[test]
    fn test_hash32_zero_check() {
        // Test zero hash
        let zero_hash = Hash32::default();
        assert!(zero_hash.is_zero());
        assert_eq!(zero_hash.leading_zero_bytes(), 32);
        assert_eq!(zero_hash.leading_zero_bits(), 256);

        // Test non-zero hash
        let mut bytes = [0u8; 32];
        bytes[31] = 1; // Set last byte to 1
        let non_zero = Hash32::new(bytes);
        assert!(!non_zero.is_zero());
        assert_eq!(non_zero.leading_zero_bytes(), 31);
        assert_eq!(non_zero.leading_zero_bits(), 31 * 8 + 7); // 248 + 7 = 255

        // Test hash with first byte non-zero
        let mut bytes = [0u8; 32];
        bytes[0] = 0x80; // Set highest bit of first byte
        let first_bit = Hash32::new(bytes);
        assert_eq!(first_bit.leading_zero_bytes(), 0);
        assert_eq!(first_bit.leading_zero_bits(), 0);
    }

    #[test]
    fn test_hash32_equality() {
        // Test equality and hashing
        let bytes1 = [1u8; 32];
        let bytes2 = [1u8; 32];
        let bytes3 = [2u8; 32];

        let hash1 = Hash32::new(bytes1);
        let hash2 = Hash32::new(bytes2);
        let hash3 = Hash32::new(bytes3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);

        // Test that equal hashes have equal hash values (for HashMap usage)
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(hash1, "value1");
        assert_eq!(map.get(&hash2), Some(&"value1"));
        assert_eq!(map.get(&hash3), None);
    }

    #[test]
    fn test_hash32_to_hash32_trait() {
        // Test ToHash32 trait for different types
        let data = b"hello world";
        let hash = data.to_hash32();

        // Should be SHA-256 of "hello world"
        assert_eq!(hash.as_bytes(), &crate::hash::sha256(data));

        // Test with String
        let string = "test string".to_string();
        let hash2 = string.to_hash32();
        assert_eq!(hash2.as_bytes(), &crate::hash::sha256(string));
    }

    #[test]
    fn test_hash32_mutability() {
        // Test mutable access to bytes
        let mut hash = Hash32::default();
        hash.as_bytes_mut()[0] = 0xff;
        assert_eq!(hash.as_bytes()[0], 0xff);
    }

    #[test]
    fn test_hash32_hash_data() {
        // Test the new hash_data convenience method
        let hash1 = Hash32::hash_data("test string");
        let hash2 = crate::hash::sha256(b"test string").into();
        assert_eq!(hash1, hash2);

        // Test with different types
        let bytes = vec![1, 2, 3, 4, 5];
        let hash3 = Hash32::hash_data(&bytes);
        let hash4 = Hash32::hash_data(bytes.as_slice());
        assert_eq!(hash3, hash4);
    }
}
