use rand::rngs::OsRng;
use schnorr::{verify_batch, KeyPair, PublicKey};

fn main() {
    let mut rng = OsRng;
    let mut items = Vec::new();

    // Create just 2 signatures
    for i in 0..2 {
        let kp = KeyPair::new(&mut rng);
        let msg = format!("message {}", i);
        let sig = kp.sign(msg.as_bytes());
        let pub_key = PublicKey::from_bytes(&kp.public_key().to_bytes()).unwrap();

        // Verify individually first
        let result = pub_key.verify(msg.as_bytes(), &sig);
        println!("Individual verify {}: {:?}", i, result);

        items.push((msg.into_bytes(), pub_key, sig));
    }

    // Now batch verify
    let result = verify_batch(&items);
    println!("Batch verify: {:?}", result);
}
