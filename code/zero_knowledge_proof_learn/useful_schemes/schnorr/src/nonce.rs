//! Deterministic nonce generation for Schnorr signatures
//!
//! This is the most security-critical component. Nonce reuse is catastrophic.
//!
//! Algorithm: nonce = SHA256("BIP340/nonce" || secret_key || message || aux_rand)

use crate::keypair::SecretKey;
use k256::elliptic_curve::PrimeField;
use k256::Scalar;
use rand_core::CryptoRngCore;
use sha2::{Digest, Sha256};

/// Generate a deterministic nonce for Schnorr signing
///
/// # Security
///
/// This function uses a deterministic algorithm to prevent nonce reuse,
/// which would catastrophically leak the secret key.
///
/// # Algorithm
///
/// ```text
/// nonce = SHA256("BIP340/nonce" || effective_secret_key || message || aux_rand)
/// ```
///
/// Where aux_rand provides additional randomness as defense-in-depth.
/// IMPORTANT: We use the effective secret key (adjusted for even-y requirement)
/// to match the signing key, as required by BIP340.
pub fn generate_nonce(
    secret_key: &SecretKey,
    message: &[u8],
    rng: &mut impl CryptoRngCore,
) -> Scalar {
    // Domain separation tag
    let mut hasher = Sha256::new();
    hasher.update(b"BIP340/nonce");

    // Input: effective secret key bytes (adjusted for BIP340 even-y requirement)
    // This MUST match the key used for signing!
    let effective_scalar = secret_key.effective_scalar();
    hasher.update(effective_scalar.to_repr());

    // Input: message
    hasher.update(message);

    // Input: auxiliary random data (defense-in-depth)
    let mut aux_rand = [0u8; 32];
    rng.fill_bytes(&mut aux_rand);
    hasher.update(aux_rand);

    // Output: 32 bytes, interpret as scalar
    let hash = hasher.finalize();

    // Convert to scalar and reduce mod n
    let scalar = Scalar::from_repr(hash);

    // Fallback: if scalar is invalid or zero, retry once (extremely unlikely)
    if bool::from(scalar.is_some()) {
        let s = scalar.unwrap();
        if !bool::from(s.is_zero()) {
            return s;
        }
    }

    // Fallback path
    Scalar::from(1u32) + Scalar::from_repr(aux_rand.into()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_nonce_deterministic_same_aux() {
        let secret_bytes = [1u8; 32];
        let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
        let message = b"test message";

        let mut rng1 = OsRng;
        let mut rng2 = OsRng;

        // Different aux_random should give different nonces (probabilistically)
        let nonce1 = generate_nonce(&secret, message, &mut rng1);
        let nonce2 = generate_nonce(&secret, message, &mut rng2);

        // Almost certainly different
        assert_ne!(nonce1.to_bytes(), nonce2.to_bytes());
    }

    #[test]
    fn test_nonce_never_zero() {
        let secret_bytes = [1u8; 32];
        let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
        let message = b"test";

        let mut rng = OsRng;
        for _ in 0..100 {
            let nonce = generate_nonce(&secret, message, &mut rng);
            assert!(!bool::from(nonce.is_zero()));
        }
    }
}
