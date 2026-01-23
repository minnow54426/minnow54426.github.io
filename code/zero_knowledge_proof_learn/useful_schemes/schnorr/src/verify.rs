//! Schnorr signature verification
//!
//! Verification equation: s*G = R + e*P
//!
//! If this holds, the signature is valid.

use crate::challenge;
use crate::{Error, PublicKey, Signature};
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::elliptic_curve::PrimeField;
use k256::{AffinePoint, ProjectivePoint, Scalar};
use rand_core::RngCore;

impl PublicKey {
    /// Verify a signature
    ///
    /// # Algorithm
    ///
    /// 1. Validate signature format
    /// 2. Parse s as scalar
    /// 3. Compute challenge e = H(R||P||m)
    /// 4. Verify: s*G = R + e*P
    ///
    /// # Arguments
    ///
    /// * `message` - Message that was signed
    /// * `signature` - Signature to verify
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        // Step 1: Validate signature
        if !signature.is_valid() {
            return Err(Error::InvalidSignature);
        }

        // Step 2: Parse s as scalar
        let s_opt = Scalar::from_repr(signature.s.into());
        let s = if bool::from(s_opt.is_some()) {
            s_opt.unwrap()
        } else {
            return Err(Error::InvalidSignature);
        };

        // Check s is not zero
        if bool::from(s.is_zero()) {
            return Err(Error::InvalidSignature);
        }

        // Step 3: Compute challenge
        let e = challenge::compute(&signature.r, &self.to_bytes(), message);

        // Step 4: Verify s*G = R + e*P
        // Left side: s*G
        let s_g = ProjectivePoint::GENERATOR * s;

        // Right side: R + e*P
        let r = Self::recover_point_from_x(&signature.r)?;
        let e_p = self.as_projective() * e;
        let rhs = r + e_p;

        // Check equality
        if s_g.eq(&rhs) {
            Ok(())
        } else {
            Err(Error::InvalidSignature)
        }
    }

    /// Recover point from x-coordinate (with even y)
    ///
    /// This reconstructs R from just the x-coordinate r.
    /// Uses the point with even y-coordinate (BIP340 convention).
    pub fn recover_point_from_x(x_bytes: &[u8; 32]) -> Result<ProjectivePoint, Error> {
        use k256::elliptic_curve::sec1::EncodedPoint;

        // Create compressed point encoding manually
        // Format: [0x02 or 0x03][32-byte x]
        // 0x02 = even y, 0x03 = odd y
        let mut encoded = [0u8; 33];
        encoded[0] = 0x02; // Even y
        encoded[1..33].copy_from_slice(x_bytes);

        // Decode the point
        let encoded_point = EncodedPoint::<k256::Secp256k1>::from_bytes(&encoded[..])
            .map_err(|_| Error::InvalidPublicKey)?;

        AffinePoint::from_encoded_point(&encoded_point)
            .into_option()
            .map(ProjectivePoint::from)
            .ok_or(Error::InvalidPublicKey)
    }
}

/// Verify multiple signatures efficiently
///
/// # Algorithm
///
/// Σ(aᵢ*sᵢ)*G = Σ(aᵢ*Rᵢ) + Σ(aᵢ*eᵢ*Pᵢ)
///
/// Where aᵢ are random coefficients to prevent fraud proofs.
///
/// # Arguments
///
/// * `batch` - Slice of (message, public_key, signature) tuples
pub fn verify_batch(batch: &[(Vec<u8>, PublicKey, Signature)]) -> Result<(), Error> {
    if batch.is_empty() {
        return Ok(());
    }

    use rand_core::OsRng;
    let mut rng = OsRng;

    // Generate random coefficients
    let coefficients: Vec<Scalar> = batch
        .iter()
        .map(|_| {
            loop {
                let mut bytes = [0u8; 32];
                rng.fill_bytes(&mut bytes);
                let scalar_opt = Scalar::from_repr(bytes.into());
                if bool::from(scalar_opt.is_some()) {
                    break scalar_opt.unwrap();
                }
                // Try again if random bytes don't form a valid scalar
            }
        })
        .collect();

    // Compute both sides of the equation
    let mut lhs = ProjectivePoint::IDENTITY;
    let mut rhs = ProjectivePoint::IDENTITY;

    for (i, (msg, pub_key, sig)) in batch.iter().enumerate() {
        let a = &coefficients[i];

        // Parse signature
        let s_opt = Scalar::from_repr(sig.s.into());
        let s = if bool::from(s_opt.is_some()) {
            s_opt.unwrap()
        } else {
            return Err(Error::InvalidSignature);
        };

        // Compute challenge
        let e = challenge::compute(&sig.r, &pub_key.to_bytes(), msg);

        // Left side: Σ(aᵢ * sᵢ) * G
        lhs += ProjectivePoint::GENERATOR * (*a * s);

        // Right side: Σ(aᵢ * Rᵢ) + Σ(aᵢ * eᵢ * Pᵢ)
        let r = PublicKey::recover_point_from_x(&sig.r)?;
        rhs += r * *a;
        rhs += pub_key.as_projective() * (*a * e);
    }

    if lhs.eq(&rhs) {
        Ok(())
    } else {
        println!("Batch verification equation failed:");
        println!("  LHS (Σaᵢsᵢ)G: {:?}", lhs);
        println!("  RHS (ΣaᵢRᵢ + ΣaᵢeᵢPᵢ): {:?}", rhs);
        Err(Error::InvalidSignature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeyPair;
    use rand::rngs::OsRng;

    #[test]
    fn test_verify_valid_signature() {
        let mut rng = OsRng;
        let keypair = KeyPair::new(&mut rng);
        let message = b"test message";

        let signature = keypair.sign(message);

        let pub_key_bytes = keypair.public_key().to_bytes();
        println!("Public key: {:02x?}", pub_key_bytes);
        println!("Signature r: {:02x?}", signature.r);
        println!("Signature s: {:02x?}", signature.s);

        let result = keypair.public_key().verify(message, &signature);
        if let Err(e) = &result {
            println!("Verification failed: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_wrong_message_fails() {
        let mut rng = OsRng;
        let keypair = KeyPair::new(&mut rng);
        let signature = keypair.sign(b"message1");

        assert!(keypair
            .public_key()
            .verify(b"message2", &signature)
            .is_err());
    }

    #[test]
    fn test_verify_wrong_key_fails() {
        let mut rng = OsRng;
        let keypair1 = KeyPair::new(&mut rng);
        let keypair2 = KeyPair::new(&mut rng);

        let signature = keypair1.sign(b"message");

        assert!(keypair2
            .public_key()
            .verify(b"message", &signature)
            .is_err());
    }

    #[test]
    fn test_verify_tampered_signature_fails() {
        let mut rng = OsRng;
        let keypair = KeyPair::new(&mut rng);
        let mut signature = keypair.sign(b"message");

        // Tamper with s
        signature.s[0] ^= 0x01;

        assert!(keypair.public_key().verify(b"message", &signature).is_err());
    }

    #[test]
    fn test_batch_verify_all_valid() {
        // Test batch with multiple signatures
        let mut items = Vec::new();
        for _ in 0..10 {
            let mut rng = OsRng;
            let kp = KeyPair::new(&mut rng);
            let msg = b"batch test";
            let sig = kp.sign(msg);
            // Clone public key via serialization
            let pub_key = PublicKey::from_bytes(&kp.public_key().to_bytes()).unwrap();
            items.push((msg.to_vec(), pub_key, sig));
        }

        let result = verify_batch(&items);
        if let Err(e) = &result {
            println!("Batch verification failed: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_verify_one_invalid_fails() {
        let mut items: Vec<_> = (0..9)
            .map(|_| {
                let mut rng = OsRng;
                let kp = KeyPair::new(&mut rng);
                let msg = b"batch test";
                let pub_key = PublicKey::from_bytes(&kp.public_key().to_bytes()).unwrap();
                (msg.to_vec(), pub_key, kp.sign(msg))
            })
            .collect();

        // Add one invalid signature
        let mut rng = OsRng;
        let kp = KeyPair::new(&mut rng);
        let mut sig = kp.sign(b"different message");
        sig.s[0] ^= 0x01; // Tamper
        let pub_key = PublicKey::from_bytes(&kp.public_key().to_bytes()).unwrap();
        items.push((b"batch test".to_vec(), pub_key, sig));

        assert!(verify_batch(&items).is_err());
    }
}
