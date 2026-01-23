//! Challenge computation for Schnorr signatures
//!
//! Challenge e = SHA256("BIP340/challenge" || r || P || m)
//!
//! Domain separation prevents cross-protocol attacks.

use k256::elliptic_curve::PrimeField;
use k256::Scalar;
use sha2::{Digest, Sha256};

/// Compute the Schnorr challenge scalar
///
/// # Algorithm
///
/// ```text
/// e = SHA256("BIP340/challenge" || r || P || m)
/// ```
///
/// Where:
/// - r is the x-coordinate of commitment point R
/// - P is the public key (33 bytes compressed)
/// - m is the message
///
/// # Arguments
///
/// * `r_bytes` - 32-byte x-coordinate of R
/// * `public_key` - 33-byte compressed public key
/// * `message` - Message to sign
pub fn compute(r_bytes: &[u8; 32], public_key: &[u8; 33], message: &[u8]) -> Scalar {
    let mut hasher = Sha256::new();

    // Domain separation tag
    hasher.update(b"BIP340/challenge");

    // r: x-coordinate of R
    hasher.update(r_bytes);

    // P: public key (compressed)
    hasher.update(public_key);

    // m: message
    hasher.update(message);

    let hash = hasher.finalize();

    // Convert to scalar
    Scalar::from_repr(hash).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_deterministic() {
        let r = [1u8; 32];
        let pk = [2u8; 33];
        let msg = b"test";

        let e1 = compute(&r, &pk, msg);
        let e2 = compute(&r, &pk, msg);

        assert_eq!(e1.to_bytes(), e2.to_bytes());
    }

    #[test]
    fn test_challenge_different_messages() {
        let r = [1u8; 32];
        let pk = [2u8; 33];

        let e1 = compute(&r, &pk, b"message1");
        let e2 = compute(&r, &pk, b"message2");

        assert_ne!(e1.to_bytes(), e2.to_bytes());
    }
}
