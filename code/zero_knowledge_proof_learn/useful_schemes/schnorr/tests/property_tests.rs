//! Property-based tests for Schnorr signatures

use proptest::prelude::*;
use schnorr::KeyPair;

proptest! {
    #[test]
    fn prop_sign_verify_roundtrip(msg in prop::collection::vec(any::<u8>(), 0..1000)) {
        let mut rng = rand::thread_rng();
        let keypair = KeyPair::new(&mut rng);
        let signature = keypair.sign(&msg);

        // Should verify successfully
        prop_assert!(keypair.public_key().verify(&msg, &signature).is_ok());

        // Tampered message should fail
        if !msg.is_empty() {
            let mut tampered = msg.clone();
            tampered[0] ^= 0xFF;
            prop_assert!(keypair.public_key().verify(&tampered, &signature).is_err());
        }
    }
}
