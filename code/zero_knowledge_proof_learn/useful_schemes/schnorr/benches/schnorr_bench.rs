use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::OsRng;
use schnorr::{verify_batch, KeyPair, PublicKey};

fn bench_sign(c: &mut Criterion) {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let message = b"benchmark message";

    c.bench_function("sign", |b| b.iter(|| keypair.sign(black_box(message))));
}

fn bench_verify(c: &mut Criterion) {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let message = b"benchmark message";
    let signature = keypair.sign(message);

    c.bench_function("verify", |b| {
        b.iter(|| {
            keypair
                .public_key()
                .verify(black_box(message), black_box(&signature))
        })
    });
}

fn bench_batch_verify(c: &mut Criterion) {
    for size in [10, 50, 100] {
        let items: Vec<_> = (0..size)
            .map(|_| {
                let mut rng = OsRng;
                let kp = KeyPair::new(&mut rng);
                let msg = b"batch";
                let pub_key_bytes = kp.public_key().to_bytes();
                let pub_key = PublicKey::from_bytes(&pub_key_bytes).expect("valid public key");
                (msg.to_vec(), pub_key, kp.sign(msg))
            })
            .collect();

        c.bench_function(&format!("batch_verify_{}", size), |b| {
            b.iter(|| verify_batch(black_box(&items)))
        });
    }
}

criterion_group!(benches, bench_sign, bench_verify, bench_batch_verify);
criterion_main!(benches);
