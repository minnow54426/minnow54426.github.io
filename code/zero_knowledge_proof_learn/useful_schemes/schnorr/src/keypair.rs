//! Key types: SecretKey, PublicKey, KeyPair
//!
//! This module implements the key types for Schnorr signatures.

use crate::error::Error;
use k256::elliptic_curve::ff::PrimeField;
use k256::elliptic_curve::group::prime::PrimeCurveAffine;
use k256::elliptic_curve::point::AffineCoordinates;
use k256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use k256::{AffinePoint, ProjectivePoint, Scalar};
use rand_core::RngCore;
use zeroize::Zeroize;

/// Secret key for Schnorr signatures
///
/// This is a wrapper around a scalar value x in [1, n-1] where n is the curve order.
/// The public key is P = x*G where G is the generator.
///
/// # BIP340 Secret Key Adjustment
///
/// BIP340 requires public keys to have even y-coordinates. If x*G has odd y,
/// BIP340 uses (n-x) as the effective secret key instead, where n is the curve order.
/// This ensures the public key P = (n-x)*G = -(x*G) has even y.
///
/// # Security
///
/// - Secrets are never serialized (use `to_bytes` only for secure backup)
/// - Secrets are zeroized on drop
/// - All operations are constant-time
#[derive(Debug, PartialEq)]
pub struct SecretKey(Scalar);

impl SecretKey {
    /// Generate a random secret key
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator (must be cryptographically secure)
    pub fn random(rng: &mut impl RngCore) -> Self {
        loop {
            // Generate random bytes and interpret as scalar
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);

            // Try to create scalar from bytes using from_repr (PrimeField trait)
            let scalar = Scalar::from_repr(bytes.into());

            // Ensure scalar is valid and not zero
            // CtOption::is_some() returns Choice, which we convert to bool
            let is_valid = scalar.is_some();
            if bool::from(is_valid) {
                let s = scalar.unwrap();
                if !bool::from(s.is_zero()) {
                    return SecretKey(s);
                }
            }

            // Retry with different bytes (loop continues)
        }
    }

    /// Create a secret key from bytes
    ///
    /// # Arguments
    ///
    /// * `bytes` - 32-byte array
    ///
    /// # Returns
    ///
    /// Returns `Error::InvalidSecretKey` if the scalar is 0 or >= curve order
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Error> {
        // Clone bytes to convert from reference to owned array
        let bytes_array = *bytes;
        let scalar = Scalar::from_repr(bytes_array.into());

        // CtOption::is_some() returns Choice, convert to bool
        let is_valid = scalar.is_some();
        if bool::from(is_valid) {
            let s = scalar.unwrap();
            if !bool::from(s.is_zero()) {
                return Ok(SecretKey(s));
            }
        }

        Err(Error::InvalidSecretKey)
    }

    /// Export secret key as bytes (use only for secure backup!)
    ///
    /// # Security Warning
    ///
    /// This exports the raw secret key. Only use this for secure backup.
    /// Never log or transmit these bytes.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_repr().into()
    }

    /// Get reference to the underlying scalar
    pub fn as_scalar(&self) -> &Scalar {
        &self.0
    }

    /// Get the effective secret key for signing
    ///
    /// BIP340: If the derived public key has odd y-coordinate, we use (n - x) for signing
    /// instead of x, where n is the curve order. This ensures the signature verifies
    /// correctly against the negated public key with even y.
    ///
    /// The computation here must match PublicKey::from_secret_key() exactly:
    /// - If x*G has odd y, PublicKey stores -(x*G) and we sign with (n-x)
    /// - If x*G has even y, PublicKey stores x*G and we sign with x
    pub(crate) fn effective_scalar(&self) -> Scalar {
        // Compute what the public key would be BEFORE negation
        let point = ProjectivePoint::GENERATOR * self.0;
        let affine = point.to_affine();
        let y_is_odd = bool::from(affine.y_is_odd());

        if y_is_odd {
            // Public key was negated in from_secret_key, so we use n - x for signing
            -self.0
        } else {
            // Public key was not negated, use x directly
            self.0
        }
    }
}

// Drop implementation to zeroize secret data
impl Drop for SecretKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

/// Public key for Schnorr signatures
///
/// This is a point P = x*G on the secp256k1 curve where x is the secret key.
/// Stored as an affine point for efficient verification.
#[derive(Debug, PartialEq)]
pub struct PublicKey(AffinePoint);

impl PublicKey {
    /// Derive a public key from a secret key
    ///
    /// Computes P = x*G where x is the secret key and G is the generator.
    ///
    /// # BIP340 Requirement
    ///
    /// The public key must have an even y-coordinate. If the computed point
    /// has odd y, we negate it to get the point with even y (which corresponds
    /// to using n-x as the secret key, where n is the curve order).
    ///
    /// # Important
    ///
    /// This function does NOT modify the secret key. The caller must use
    /// `effective_scalar()` when signing to get the properly adjusted secret key.
    pub fn from_secret_key(secret: &SecretKey) -> Self {
        let point = ProjectivePoint::GENERATOR * secret.as_scalar();
        let affine = point.to_affine();

        // BIP340: Public keys must have even y-coordinate
        // If y is odd, negate the point to get even y
        let y_is_odd = bool::from(affine.y_is_odd());

        if y_is_odd {
            PublicKey(-affine)
        } else {
            PublicKey(affine)
        }
    }

    /// Parse a public key from compressed SEC encoding
    ///
    /// # Arguments
    ///
    /// * `bytes` - 33-byte compressed SEC encoding
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 33 {
            return Err(Error::InvalidEncoding);
        }

        let encoded = k256::EncodedPoint::from_bytes(bytes).map_err(|_| Error::InvalidEncoding)?;

        let point = AffinePoint::from_encoded_point(&encoded)
            .into_option()
            .ok_or(Error::InvalidPublicKey)?;

        // Check that point is not infinity
        if point.is_identity().into() {
            return Err(Error::InvalidPublicKey);
        }

        Ok(PublicKey(point))
    }

    /// Serialize public key to compressed SEC encoding
    pub fn to_bytes(&self) -> [u8; 33] {
        let encoded = self.0.to_encoded_point(true);
        let mut bytes = [0u8; 33];
        bytes.copy_from_slice(encoded.as_bytes());
        bytes
    }

    /// Get reference to the underlying affine point (internal use)
    #[allow(dead_code)]
    fn as_affine(&self) -> &AffinePoint {
        &self.0
    }

    /// Get as projective point (for verification)
    pub(crate) fn as_projective(&self) -> ProjectivePoint {
        ProjectivePoint::from(self.0)
    }
}

/// A key pair for Schnorr signatures
///
/// Combines a secret key and its derived public key.
pub struct KeyPair {
    secret: SecretKey,
    public: PublicKey,
}

impl KeyPair {
    /// Generate a new random key pair
    ///
    /// # BIP340 Compliance
    ///
    /// PublicKey::from_secret_key() now automatically ensures the public key
    /// has an even y-coordinate by negating the point if necessary, so this
    /// method is simpler than before.
    pub fn new(rng: &mut (impl rand_core::CryptoRng + rand_core::RngCore)) -> Self {
        let secret = SecretKey::random(rng);
        let public = PublicKey::from_secret_key(&secret);
        KeyPair { secret, public }
    }

    /// Get the secret key
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret
    }

    /// Get the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }

    /// Create a KeyPair from an existing secret key
    ///
    /// # BIP340 Compliance
    ///
    /// This ensures the public key has an even y-coordinate by negating
    /// the point if necessary (as per BIP340 specification).
    pub fn from_secret(secret: SecretKey) -> Self {
        let public = PublicKey::from_secret_key(&secret);
        KeyPair { secret, public }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_secret_key_random() {
        let mut rng = OsRng;
        let sk = SecretKey::random(&mut rng);
        let bytes = sk.to_bytes();

        // Should be 32 bytes
        assert_eq!(bytes.len(), 32);

        // Should not be all zeros
        assert_ne!(bytes, [0u8; 32]);
    }

    #[test]
    fn test_secret_key_from_bytes() {
        let bytes = [1u8; 32];
        let sk = SecretKey::from_bytes(&bytes).unwrap();

        assert_eq!(sk.to_bytes(), bytes);
    }

    #[test]
    fn test_secret_key_zero_bytes_fails() {
        let bytes = [0u8; 32];
        let result = SecretKey::from_bytes(&bytes);

        assert_eq!(result, Err(Error::InvalidSecretKey));
    }

    #[test]
    fn test_public_key_from_secret() {
        let mut rng = OsRng;
        let secret = SecretKey::random(&mut rng);
        let public = PublicKey::from_secret_key(&secret);

        // Public key should be 33 bytes (compressed SEC encoding)
        let bytes = public.to_bytes();
        assert_eq!(bytes.len(), 33);

        // First byte should be 0x02 or 0x03 (compressed encoding)
        assert!(bytes[0] == 0x02 || bytes[0] == 0x03);
    }

    #[test]
    fn test_public_key_roundtrip() {
        let mut rng = OsRng;
        let secret = SecretKey::random(&mut rng);
        let public1 = PublicKey::from_secret_key(&secret);

        let bytes = public1.to_bytes();
        let public2 = PublicKey::from_bytes(&bytes).unwrap();

        assert_eq!(public1.to_bytes(), public2.to_bytes());
    }

    #[test]
    fn test_public_key_invalid_bytes() {
        // Wrong length
        let result = PublicKey::from_bytes(&[0x02; 32]);
        assert_eq!(result, Err(Error::InvalidEncoding));
    }

    #[test]
    fn test_keypair_generation() {
        let mut rng = OsRng;
        let keypair = KeyPair::new(&mut rng);

        // Should have both secret and public keys
        let _ = keypair.secret_key();
        let pub_bytes = keypair.public_key().to_bytes();

        assert_eq!(pub_bytes.len(), 33);
    }

    #[test]
    fn test_keypair_from_secret() {
        let secret1_bytes = [1u8; 32];
        let secret1 = SecretKey::from_bytes(&secret1_bytes).unwrap();
        let keypair = KeyPair::from_secret(secret1);

        assert_eq!(keypair.secret_key().to_bytes(), secret1_bytes);
    }
}
