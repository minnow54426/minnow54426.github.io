use rand::rngs::OsRng;
use schnorr::{verify_batch, KeyPair, PublicKey};

fn main() {
    let mut rng = OsRng;

    // Create ONE signature and verify it individually and in batch
    let kp = KeyPair::new(&mut rng);
    let msg = b"test";
    let sig = kp.sign(msg);
    let pub_key = PublicKey::from_bytes(&kp.public_key().to_bytes()).unwrap();

    println!("Individual verify: {:?}", pub_key.verify(msg, &sig));

    let items = vec![(msg.to_vec(), pub_key, sig)];
    println!("Batch verify: {:?}", verify_batch(&items));
}
