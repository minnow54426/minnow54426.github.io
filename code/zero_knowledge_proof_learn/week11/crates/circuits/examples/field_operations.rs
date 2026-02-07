//! Field Operations Demo
//!
//! Demonstrates finite field arithmetic operations using arkworks
//! This example complements Chapter 1: Mathematical Background

use ark_bn254::Fr as ScalarField;
use ark_ff::Field;

fn main() {
    println!("Finite Field Operations Demo");
    println!("=============================");
    println!();

    // Create field elements
    println!("Creating field elements in 𝔽ₚ (BN254 scalar field):");
    println!("---------------------------------------------------");
    let a = ScalarField::from(5u64);
    let b = ScalarField::from(4u64);

    println!("a = {}", a);
    println!("b = {}", b);
    println!();

    // Addition
    println!("Addition:");
    println!("----------");
    let sum = a + b;
    println!("a + b = {}", sum);
    println!("Note: All operations are modulo the field prime");
    println!("      (BN254 field prime is very large, so no wrap-around here)");
    println!();

    // Multiplication
    println!("Multiplication:");
    println!("---------------");
    let product = a * b;
    println!("a × b = {}", product);
    println!();

    // Multiplicative inverse
    println!("Multiplicative Inverse:");
    println!("-----------------------");
    let inverse = a.inverse().unwrap();
    println!("a⁻¹ = {}", inverse);
    println!("Verification: a × a⁻¹ = {}", a * inverse);
    println!();

    // Squaring
    println!("Exponentiation (Squaring):");
    println!("--------------------------");
    let square = a.square();
    println!("a² = {}", square);
    println!();

    // Demonstrate modular arithmetic with a small field
    println!("Modular Arithmetic Intuition:");
    println!("------------------------------");
    println!("In a small field like 𝔽₇:");
    println!("  (5 + 4) mod 7 = 9 mod 7 = 2");
    println!("  (3 × 6) mod 7 = 18 mod 7 = 4");
    println!();
    println!("Think of a clock (mod 12):");
    println!("  10:00 + 5 hours = 3:00 (not 15:00!)");
    println!("  9:00 - 12 hours = 9:00 (wraps around)");
    println!();

    // Field properties
    println!("Field Properties:");
    println!("-----------------");
    let zero = ScalarField::from(0u64);
    let one = ScalarField::ONE;

    println!(
        "Additive identity: a + 0 = {} (equals a? {})",
        a + zero,
        (a + zero) == a
    );
    println!(
        "Multiplicative identity: a × 1 = {} (equals a? {})",
        a * one,
        (a * one) == a
    );
    println!("Additive inverse: a + (-a) = {}", a + (-a));
    println!();

    // Working with larger values
    println!("Larger Values:");
    println!("--------------");
    let big_a = ScalarField::from(123456789u64);
    let big_b = ScalarField::from(987654321u64);

    println!("big_a = {}", big_a);
    println!("big_b = {}", big_b);
    println!("big_a + big_b = {}", big_a + big_b);
    println!("big_a × big_b = {}", big_a * big_b);
    println!();

    println!("Demo completed successfully!");
}
