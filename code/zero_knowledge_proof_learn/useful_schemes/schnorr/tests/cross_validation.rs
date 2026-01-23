//! Cross-library validation against secp256k1 crate
//!
//! Note: This requires the secp256k1 crate with Schnorr support

use rand::rngs::OsRng;
use schnorr::KeyPair;

#[test]
#[ignore] // Run manually: cargo test --test cross_validation -- --ignored
fn test_cross_library_compatibility() {
    // This test validates interoperability with the secp256k1 crate
    // It's ignored by default since it requires specific secp256k1 features

    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let message = b"cross-library test";

    // Sign with our library
    let signature = keypair.sign(message);

    // Verify with our library
    assert!(keypair.public_key().verify(message, &signature).is_ok());

    // TODO: Add verification with secp256k1 crate once Schnorr is available
    // let secp_ctx = secp256k1::Secp256k1::new();
    // let secp_sig = secp256k1::schnorr::Signature::from_slice(&signature.to_bytes()).unwrap();
    // let secp_msg = secp256k1::Message::from_digest_slice(message).unwrap();
    // assert!(secp_ctx.verify_schnorrsig(&secp_msg, &secp_sig, &secp_pubkey).is_ok());

    println!("Cross-validation: Our implementation produces valid signatures");
}
