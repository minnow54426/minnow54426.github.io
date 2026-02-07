use ark_bn254::{G1Projective as G1, Fr};
use ark_ff::UniformRand;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let scalar = Fr::rand(&mut rng);
    
    // Try different ways to get generator
    let g1_affine = ark_bn254::G1Affine::generator();
    println!("G1Affine::generator() works: {:?}", g1_affine);
    
    let g1_proj = G1::from(g1_affine);
    println!("G1::from(affine) works");
    
    let result = g1_proj * scalar;
    println!("Multiplication works");
}
