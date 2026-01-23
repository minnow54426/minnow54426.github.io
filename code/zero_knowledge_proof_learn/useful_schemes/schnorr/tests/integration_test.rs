//! Integration tests for Schnorr signatures

use rand::rngs::OsRng;
use schnorr::{KeyPair, PublicKey, Signature};

#[test]
fn test_end_to_end_sign_verify() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let message = b"Hello, Schnorr!";

    let signature = keypair.sign(message);
    assert!(keypair.public_key().verify(message, &signature).is_ok());
}

#[test]
fn test_multiple_messages_same_key() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);

    let messages = vec![b"msg1", b"msg2", b"msg3"];

    for msg in messages {
        let sig = keypair.sign(msg);
        assert!(keypair.public_key().verify(msg, &sig).is_ok());
    }
}

#[test]
fn test_key_serialization_roundtrip() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);

    let pub_bytes = keypair.public_key().to_bytes();
    let pub_restored = PublicKey::from_bytes(&pub_bytes).unwrap();

    assert_eq!(keypair.public_key().to_bytes(), pub_restored.to_bytes());

    // Verify that restored key works
    let msg = b"test";
    let sig = keypair.sign(msg);
    assert!(pub_restored.verify(msg, &sig).is_ok());
}

#[test]
fn test_signature_serialization_roundtrip() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let msg = b"test";

    let sig1 = keypair.sign(msg);
    let bytes = sig1.to_bytes();
    let sig2 = Signature::from_bytes(&bytes).unwrap();

    assert_eq!(sig1, sig2);
    assert!(keypair.public_key().verify(msg, &sig2).is_ok());
}

#[test]
fn test_empty_message() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);

    let sig = keypair.sign(&[]);
    assert!(keypair.public_key().verify(&[], &sig).is_ok());
}

#[test]
fn test_large_message() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let large_msg = vec![0u8; 1_000_000];

    let sig = keypair.sign(&large_msg);
    assert!(keypair.public_key().verify(&large_msg, &sig).is_ok());
}
