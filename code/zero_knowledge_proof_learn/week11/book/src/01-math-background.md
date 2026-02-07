# Mathematical Background

Before diving into Groth16, we need to build some mathematical foundations. Don't worry - we'll connect everything to practical code!

## Learning Objectives

After this chapter, you will understand:
- Finite fields and modular arithmetic
- Polynomials and polynomial evaluation
- Lagrange interpolation
- Why polynomials are useful in zero-knowledge proofs

## Finite Fields

### What is a Field?

A **field** is a mathematical structure where you can add, subtract, multiply, and divide (except by zero). Examples you know:
- **Rational numbers** (ℚ): fractions like 3/4, -5/7
- **Real numbers** (ℝ): decimals like 3.14, √2
- **Complex numbers** (ℂ): a + bi

### Finite Fields (Galois Fields)

In cryptography, we use **finite fields** - fields with a finite number of elements. We denote a field of prime order p as **𝔽ₚ**.

**Example**: 𝔽₇ = {0, 1, 2, 3, 4, 5, 6} (integers modulo 7)

### Modular Arithmetic

Finite fields use **modular arithmetic** - arithmetic that wraps around.

```rust,ignore
// In 𝔽₇:
(5 + 4) % 7 = 9 % 7 = 2      // Addition wraps around
(3 × 6) % 7 = 18 % 7 = 4     // Multiplication wraps around
```

**Key intuition**: Think of a clock (modular arithmetic mod 12):
- 10:00 + 5 hours = 3:00 (not 15:00!)
- 9:00 - 12 hours = 9:00 (going backwards wraps)

### In Practice: arkworks Finite Fields

In our Rust code, we use `ark_bn254::Fr` - the scalar field for BN254 elliptic curve:

```rust,ignore
use ark_bn254::Fr as ScalarField;
use ark_ff::Field;

// Field elements
let a = ScalarField::from(5u64);
let b = ScalarField::from(4u64);

let sum = a + b;                          // Addition
let product = a * b;                      // Multiplication
let inverse = a.inverse().unwrap();       // Multiplicative inverse
let square = a.square();                  // Exponentiation (a²)

// All operations are modulo the field prime (automatically!)
```

**Run this example**:

```bash
cargo run --example field_operations
```

## Polynomials

### What is a Polynomial?

A **polynomial** is an expression of variables and coefficients:

```text
P(x) = a₀ + a₁x + a₂x² + a₃x³ + ... + aₙxⁿ
```

Where:
- `aᵢ` are coefficients (from a field)
- `x` is the variable
- `n` is the degree

**Example**: `P(x) = 2 + 3x + x²` is a degree-2 polynomial

### Why Polynomials?

Polynomials are incredibly useful in zero-knowledge proofs because of this **fundamental theorem**:

> **Theorem**: A degree-d polynomial is uniquely determined by d+1 points.

**Implication**: If two degree-d polynomials agree on d+1 points, they're identical!

This is the foundation of QAP (Quadratic Arithmetic Programs).

### Polynomial Evaluation

Evaluating a polynomial means computing its value at a specific point:

```rust,ignore
// Evaluate P(x) = 2 + 3x + x² at x = 5:
P(5) = 2 + 3(5) + 5² = 2 + 15 + 25 = 42
```

**In code**:

```rust,ignore
use ark_ff::Field;
use ark_bn254::Fr as ScalarField;

// P(x) = 2 + 3x + x²
fn evaluate_polynomial(x: ScalarField) -> ScalarField {
    ScalarField::from(2u64)
        + ScalarField::from(3u64) * x
        + x * x
}

let x = ScalarField::from(5u64);
let result = evaluate_polynomial(x);
// result = 42
```

### Polynomial Interpolation

**Interpolation** is the reverse of evaluation: given points, find the polynomial.

**Lagrange Interpolation** finds the unique polynomial passing through given points:

```text
Given: (1, 3), (2, 5), (3, 9)
Find: P(x) such that P(1)=3, P(2)=5, P(3)=9
```

**Formula**: For points (x₁, y₁), ..., (xₙ, yₙ):

```text
P(x) = Σ yᵢ · Lᵢ(x)
```

Where `Lᵢ(x)` is the i-th Lagrange basis polynomial:

```text
Lᵢ(x) = Π (x - xⱼ) / (xᵢ - xⱼ)  for all j ≠ i
```

**Example** (simplified):

Given points (1, 3), (2, 5):

```text
L₁(x) = (x - 2) / (1 - 2) = (x - 2) / -1 = 2 - x
L₂(x) = (x - 1) / (2 - 1) = (x - 1) / 1 = x - 1

P(x) = 3·L₁(x) + 5·L₂(x)
     = 3(2 - x) + 5(x - 1)
     = 6 - 3x + 5x - 5
     = 1 + 2x

Verify: P(1) = 1 + 2(1) = 3 ✓
         P(2) = 1 + 2(2) = 5 ✓
```

### In Practice: Using arkworks

```rust,ignore
use ark_poly::{
    polynomial::univariate::DensePolynomial,
    EvaluationDomain,
    GeneralEvaluationDomain,
};
use ark_bn254::Fr as ScalarField;

// Create polynomial: P(x) = 1 + 2x
let coefficients = vec![
    ScalarField::from(1u64),  // constant term
    ScalarField::from(2u64),  // x term
];
let poly = DensePolynomial::from_coefficients_slice(&coefficients);

// Evaluate at x = 5
let x = ScalarField::from(5u64);
let result = poly.evaluate(&x);
// result = 1 + 2(5) = 11

// Interpolate polynomial from points
let domain = GeneralEvaluationDomain::new(8).unwrap();
// ... (see Chapter 3 for full QAP interpolation)
```

## Polynomial Division

### The Division Test

A key insight in Groth16: **A polynomial is divisible by another if and only if it evaluates to zero at all roots of the divisor**.

**Example**: Let `T(x) = (x - 1)(x - 2)(x - 3)` (a "target polynomial")

If `P(x) = H(x) · T(x)`, then:
- `P(1) = H(1) · T(1) = H(1) · 0 = 0`
- `P(2) = H(2) · T(2) = H(2) · 0 = 0`
- `P(3) = H(3) · T(3) = H(3) · 0 = 0`

**Converse**: If `P(1) = P(2) = P(3) = 0`, then `P(x)` is divisible by `T(x)`!

This is the **QAP satisfaction check** in Groth16!

### In Practice: Polynomial Division

```rust,ignore
use ark_poly::polynomial::UVPolynomial;

// P(x) = x³ - 6x² + 11x - 6
// T(x) = (x - 1)(x - 2)(x - 3) = x³ - 6x² + 11x - 6
// H(x) = P(x) / T(x) = 1

let p = DensePolynomial::from_coefficients_slice(&[
    ScalarField::from(6u64),   // -6
    ScalarField::from(11u64),  // 11x
    ScalarField::from(6u64),   // -6x²
    ScalarField::from(1u64),   // x³
]);

let t = DensePolynomial::from_coefficients_slice(&[
    ScalarField::from(6u64),
    ScalarField::from(11u64),
    ScalarField::from(6u64),
    ScalarField::from(1u64),
]);

let h = &p / &t;
// h = 1 (the quotient polynomial)
```

## Connecting to Groth16

### The Big Picture

1. **R1CS** (Chapter 2): Encodes computation as matrix constraints
2. **QAP** (Chapter 3): Transforms R1CS to polynomial divisibility check
3. **Pairings** (Chapter 4): Allow us to check polynomial equations in the exponent

**Key insight**: Checking if a polynomial division works is equivalent to checking if the computation is correct!

### Example: Multiplier Circuit

For `a × b = c`:
1. **R1CS form**: `Az ∘ Bz = Cz` where `z = [1, a, b, c]`
2. **QAP form**: `P(x) = H(x) · T(x)` where `P(x) = A(x) · B(x) - C(x)`
3. **Verification**: Check if `P(x)` is divisible by `T(x)` using pairings

We'll see this in detail in Chapters 2 and 3!

## Exercises

1. **Field arithmetic**:
   ```rust
   // In 𝔽₇, compute:
   // (3 + 5) × (2 + 4)
   ```
   What's the result? Check with Rust code.

2. **Polynomial evaluation**:
   ```rust
   // P(x) = 3 + 2x - x²
   // Compute P(4) in 𝔽₇
   ```
   Hint: Be careful with negative numbers!

3. **Interpolation intuition**:
   - Given 2 points, how many degree-1 polynomials pass through them?
   - Given 3 points, how many degree-2 polynomials pass through them?
   - Given 3 points, how many degree-3 polynomials pass through them?

4. **Polynomial divisibility**:
   ```text
   P(x) = x³ - 6x² + 11x - 6
   T(x) = (x - 1)(x - 2)(x - 3)
   ```
   Is P(x) divisible by T(x)? Verify by evaluating P(x) at x=1, 2, 3.

## Further Reading

- **Finite Fields**: [Wikipedia: Finite Field](https://en.wikipedia.org/wiki/Finite_field)
- **Polynomials**: [Khan Academy: Polynomials](https://www.khanacademy.org/math/algebra2/x2ec2f6f830c9fb89:poly)
- **Interpolation**: [Brilliant: Lagrange Interpolation](https://brilliant.org/wiki/lagrange-interpolation/)

---

**Ready to encode computations? Continue to [Chapter 2: Rank-1 Constraint Systems](./02-r1cs.md)**
