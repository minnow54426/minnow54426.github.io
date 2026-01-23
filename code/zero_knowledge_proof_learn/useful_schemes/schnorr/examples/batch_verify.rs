use rand::rngs::OsRng;
use schnorr::{verify_batch, KeyPair, PublicKey};
use std::time::Instant;

fn main() {
    let mut rng = OsRng;
    let n = 100;
    let items: Vec<_> = (0..n)
        .map(|_| {
            let kp = KeyPair::new(&mut rng);
            let msg = b"batch verification test message";
            let pub_key_bytes = kp.public_key().to_bytes();
            let pub_key = PublicKey::from_bytes(&pub_key_bytes).expect("valid public key");
            (msg.to_vec(), pub_key, kp.sign(msg))
        })
        .collect();

    let start = Instant::now();
    for (msg, pub_key, sig) in &items {
        pub_key.verify(msg, sig).expect("valid");
    }
    let individual_time = start.elapsed();

    let start = Instant::now();
    verify_batch(&items).expect("batch valid");
    let batch_time = start.elapsed();

    let speedup = individual_time.as_nanos() as f64 / batch_time.as_nanos() as f64;
    println!("Batch verification is {:.2}x faster", speedup);
}
