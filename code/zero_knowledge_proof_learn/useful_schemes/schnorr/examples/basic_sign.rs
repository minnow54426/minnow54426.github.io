use rand::rngs::OsRng;
use schnorr::KeyPair;

fn main() {
    let mut rng = OsRng;
    let keypair = KeyPair::new(&mut rng);
    let message = b"Hello, Schnorr signatures!";
    let signature = keypair.sign(message);

    println!(
        "Public key: {}",
        hex::encode(keypair.public_key().to_bytes())
    );
    println!("Signature r: {}", hex::encode(signature.r));
    println!("Signature s: {}", hex::encode(signature.s));

    match keypair.public_key().verify(message, &signature) {
        Ok(()) => println!("\n✓ Signature valid!"),
        Err(e) => println!("\n✗ Signature invalid: {}", e),
    }
}
