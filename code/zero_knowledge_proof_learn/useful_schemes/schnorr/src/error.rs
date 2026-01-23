//! Error types for Schnorr operations
//!
//! This module defines the error types used throughout the library.

use std::fmt;

/// Errors that can occur during Schnorr operations
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Secret key is invalid (scalar is 0 or >= curve order)
    InvalidSecretKey,

    /// Public key is invalid (point not on curve or at infinity)
    InvalidPublicKey,

    /// Signature verification failed
    InvalidSignature,

    /// Nonce generation failed (nonce is 0)
    InvalidNonce,

    /// Invalid byte encoding (wrong length, parse error)
    InvalidEncoding,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSecretKey => write!(f, "Invalid secret key: scalar must be in [1, n-1]"),
            Error::InvalidPublicKey => write!(f, "Invalid public key: point not on curve"),
            Error::InvalidSignature => write!(f, "Invalid signature: verification failed"),
            Error::InvalidNonce => write!(f, "Invalid nonce: nonce cannot be zero"),
            Error::InvalidEncoding => write!(f, "Invalid encoding: byte sequence malformed"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            Error::InvalidSecretKey.to_string(),
            "Invalid secret key: scalar must be in [1, n-1]"
        );
        assert_eq!(
            Error::InvalidPublicKey.to_string(),
            "Invalid public key: point not on curve"
        );
        assert_eq!(
            Error::InvalidSignature.to_string(),
            "Invalid signature: verification failed"
        );
        assert_eq!(
            Error::InvalidNonce.to_string(),
            "Invalid nonce: nonce cannot be zero"
        );
        assert_eq!(
            Error::InvalidEncoding.to_string(),
            "Invalid encoding: byte sequence malformed"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(Error::InvalidSignature, Error::InvalidSignature);
        assert_ne!(Error::InvalidSignature, Error::InvalidNonce);
    }
}
