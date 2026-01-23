use k256::elliptic_curve::PrimeField;
use rand::rngs::OsRng;
use schnorr::{KeyPair, SecretKey};

fn main() {
    let mut rng = OsRng;

    for i in 0..10 {
        let secret = SecretKey::random(&mut rng);
        let bytes = secret.to_bytes();

        // Compute public key manually
        use k256::{ProjectivePoint, Scalar};
        let scalar = Scalar::from_repr(bytes.into()).unwrap();
        let point = ProjectivePoint::GENERATOR * scalar;
        let affine = point.to_affine();

        use k256::elliptic_curve::point::AffineCoordinates;
        let y_is_odd = bool::from(affine.y_is_odd());

        let kp = KeyPair::from_secret(secret);
        let pub_bytes = kp.public_key().to_bytes();

        println!(
            "{}: y_is_odd={}, pub_prefix={:02x}",
            i, y_is_odd, pub_bytes[0]
        );

        // Sign and verify
        let msg = b"test";
        let sig = kp.sign(msg);
        let result = kp.public_key().verify(msg, &sig);
        println!("  verify: {:?}", result);
    }
}
