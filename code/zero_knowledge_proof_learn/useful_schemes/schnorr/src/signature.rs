//! Signature type
//!
//! Schnorr signature as (r, s) where r is the x-coordinate of R = k*G
//! and s = k + H(R||P||m)*x

use crate::error::Error;
use k256::elliptic_curve::PrimeField;
use k256::Scalar;

/// Schnorr signature
///
/// A Schnorr signature consists of two components:
/// - `r`: The x-coordinate of the commitment point R = k*G
/// - `s`: The response scalar s = k + H(R||P||m)*x
///
/// Both are 32-byte arrays.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
}

impl Signature {
    /// Create a signature from 64 bytes
    ///
    /// # Arguments
    ///
    /// * `bytes` - 64-byte array (32 bytes for r, 32 bytes for s)
    pub fn from_bytes(bytes: &[u8; 64]) -> Result<Self, Error> {
        let mut sig = Signature {
            r: [0u8; 32],
            s: [0u8; 32],
        };

        sig.r.copy_from_slice(&bytes[0..32]);
        sig.s.copy_from_slice(&bytes[32..64]);

        // Validate signature
        if !sig.is_valid() {
            return Err(Error::InvalidSignature);
        }

        Ok(sig)
    }

    /// Serialize signature to 64 bytes
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[0..32].copy_from_slice(&self.r);
        bytes[32..64].copy_from_slice(&self.s);
        bytes
    }

    /// Validate signature components
    ///
    /// Checks that:
    /// - s is not zero
    /// - s is less than curve order (implicitly checked by scalar reduction)
    pub(crate) fn is_valid(&self) -> bool {
        // Check that s is not zero
        let s_option = Scalar::from_repr(self.s.into());
        if s_option.is_none().into() {
            return false;
        }
        let s = s_option.unwrap();
        !bool::from(s.is_zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_roundtrip() {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r[0] = 0x01;
        s[0] = 0x02;

        let sig1 = Signature { r, s };
        let bytes = sig1.to_bytes();
        let sig2 = Signature::from_bytes(&bytes).unwrap();

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_signature_zero_s_fails() {
        let r_bytes = [1u8; 32];
        let mut sig_bytes = [0u8; 64];
        sig_bytes[0..32].copy_from_slice(&r_bytes);
        // s is all zeros - should fail

        let result = Signature::from_bytes(&sig_bytes);
        assert_eq!(result, Err(Error::InvalidSignature));
    }
}
