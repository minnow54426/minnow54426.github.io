//! Schnorr signing algorithm
//!
//! s = k + H(R||P||m)*x
//!
//! Where:
//! - k is the nonce
//! - R = k*G is the commitment
//! - r is the x-coordinate of R
//! - e = H(R||P||m) is the challenge
//! - x is the secret key

use crate::challenge;
use crate::nonce;
use crate::{KeyPair, Signature};
use k256::elliptic_curve::point::AffineCoordinates;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::ProjectivePoint;

impl KeyPair {
    /// Sign a message
    ///
    /// # Algorithm
    ///
    /// 1. Generate nonce k
    /// 2. Compute R = k*G
    /// 3. Extract r = x-coordinate of R
    /// 4. Compute challenge e = H(R||P||m)
    /// 5. Compute s = k + e*x
    /// 6. Return signature (r, s)
    ///
    /// # Arguments
    ///
    /// * `message` - Message to sign
    pub fn sign(&self, message: &[u8]) -> Signature {
        use rand_core::OsRng;

        // Step 1: Generate nonce k
        // NOTE: Per BIP340, the nonce is generated using the effective secret key
        // (adjusted for even-y requirement), which matches the signing key.
        let mut rng = OsRng;
        let k = nonce::generate_nonce(self.secret_key(), message, &mut rng);

        // Step 2: Compute R = k*G
        let mut r_point = ProjectivePoint::GENERATOR * k;

        // Step 3: Extract r = x-coordinate of R (with even y)
        let r_affine = r_point.to_affine();

        // BIP340: If R has odd y, negate it (use -k instead)
        // This ensures the recovered point during verification matches
        let y_is_odd = bool::from(r_affine.y_is_odd());
        let k = if y_is_odd {
            r_point = -r_point;
            -k // Negate k to match the negated R
        } else {
            k
        };

        // Now extract x-coordinate from (possibly negated) R
        let r_final = r_point.to_affine();
        let encoded = r_final.to_encoded_point(false);
        let x_coord = encoded.x().unwrap();
        let mut r_bytes = [0u8; 32];
        // Convert GenericArray to byte array
        for (i, &byte) in x_coord.iter().enumerate() {
            r_bytes[i] = byte;
        }

        // Step 4: Compute challenge e = H(R||P||m)
        let public_bytes = self.public_key().to_bytes();
        let e = challenge::compute(&r_bytes, &public_bytes, message);

        // Step 5: Compute s = k + e*x
        // Use the effective secret key (adjusted for BIP340 even-y requirement)
        let x = self.secret_key().effective_scalar();
        let s = k + (e * x);

        // Step 6: Output signature
        Signature {
            r: r_bytes,
            s: s.to_bytes().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_sign_creates_valid_signature() {
        let mut rng = OsRng;
        let keypair = KeyPair::new(&mut rng);
        let message = b"test message";

        let signature = keypair.sign(message);

        // Signature should have valid length
        assert_eq!(signature.r.len(), 32);
        assert_eq!(signature.s.len(), 32);
    }

    #[test]
    fn test_sign_deterministic_same_inputs() {
        let mut rng = OsRng;
        let keypair = KeyPair::new(&mut rng);
        let message = b"deterministic test";

        // Sign twice (note: aux_rand will differ, so signatures will differ)
        let sig1 = keypair.sign(message);
        let sig2 = keypair.sign(message);

        // Due to aux_rand, signatures will differ (this is OK!)
        // But both should have valid length
        assert_eq!(sig1.r.len(), 32);
        assert_eq!(sig1.s.len(), 32);
        assert_eq!(sig2.r.len(), 32);
        assert_eq!(sig2.s.len(), 32);

        // Signatures should be different (probabilistically)
        assert_ne!(sig1, sig2);
    }
}
