# Chapter 3: Quadratic Arithmetic Programs

## Introduction

In Chapter 2, we learned how to transform arbitrary computations into Rank-1 Constraint Systems (R1CS). R1CS gives us a way to verify computations by checking three matrices against a witness vector. However, R1CS has a problem: verification requires checking every constraint, which is O(n) in the number of constraints.

Quadratic Arithmetic Programs (QAPs) solve this by transforming the constraint system into a polynomial divisibility check. Instead of verifying n constraints separately, we verify a single polynomial equation. This is the key insight that makes zk-SNARKs practical.

**Chapter Goals:**
- Understand the R1CS → QAP transformation
- Learn Lagrange interpolation for constructing polynomials
- See how polynomial division replaces constraint checking
- Implement the transformation in Rust

## From Constraints to Polynomials

### The Problem with R1CS

Recall that R1CS consists of three matrices (A, B, C) and a witness w such that:

```
Aw ∘ Bw = Cw  (where ∘ is element-wise multiplication)
```

To verify, we must check this equation for all n constraints. For large circuits (thousands of constraints), this is expensive.

### The QAP Solution

A QAP transforms the constraint matrices into three polynomials (A(x), B(x), C(x)) and a target polynomial t(x). The verification becomes:

```
A(x) · B(x) - C(x) = t(x) · h(x)
```

Where:
- A(x), B(x), C(x) are constructed from the constraint matrices
- t(x) is the "target polynomial" that encodes constraint locations
- h(x) is the quotient polynomial (computed by division)

If this equation holds at a random point, the computation is valid with high probability.

**Key Insight:** Polynomial checking is O(1) regardless of circuit size!

### Why This Works

Polynomials have a crucial property: if two degree-d polynomials agree at d+1 points, they're identical everywhere. This means:
- We can check the polynomial equation at a single random point
- If it holds, the equation holds everywhere with overwhelming probability
- This reduces verification from O(n) to O(1)

This is the "algebraic simplification" at the heart of zk-SNARKs.

## Mathematical Foundation

### Lagrange Interpolation

Given k points (x₁, y₁), (x₂, y₂), ..., (xₖ, yₖ), Lagrange interpolation constructs a unique degree-(k-1) polynomial passing through all points.

**Lagrange Basis Polynomials:**

For each point i, define:

```
Lᵢ(x) = Π (x - xⱼ) / (xᵢ - xⱼ)  for all j ≠ i
```

Properties:
- Lᵢ(xᵢ) = 1
- Lᵢ(xⱼ) = 0 for all j ≠ i

**Interpolated Polynomial:**

```
P(x) = Σ yᵢ · Lᵢ(x)
```

This polynomial passes through all (xᵢ, yᵢ) points.

### From R1CS Matrices to Polynomials

Given R1CS matrices A, B, C (each n×m), we construct polynomials A(x), B(x), C(x).

For each column j (corresponding to witness variable wⱼ):

1. **Extract the column:** A[:, j] = [A[1,j], A[2,j], ..., A[n,j]]
2. **Interpolate:** Construct polynomial Aⱼ(x) passing through points (1, A[1,j]), (2, A[2,j]), ..., (n, A[n,j])
3. **Repeat for B and C:** Similarly construct Bⱼ(x) and Cⱼ(x)

The final polynomials are:
- A(x) = Σ wⱼ · Aⱼ(x)
- B(x) = Σ wⱼ · Bⱼ(x)
- C(x) = Σ wⱼ · Cⱼ(x)

### The Target Polynomial t(x)

The target polynomial encodes where constraints are located:

```
t(x) = Π (x - i)  for i = 1 to n
```

This polynomial is zero exactly at x = 1, 2, ..., n (the constraint indices).

### Polynomial Division and the Quotient

If the R1CS constraints are satisfied, then:

```
A(x) · B(x) - C(x) = 0  at x = 1, 2, ..., n
```

This means (A·B - C) is divisible by t(x):

```
(A(x) · B(x) - C(x)) / t(x) = h(x)
```

Where h(x) is the quotient polynomial. If division fails (has remainder), the computation is invalid.

**Verification Check:**

Instead of checking n constraints, we verify:
```
A(r) · B(r) - C(r) = t(r) · h(r)
```

For a random point r. This single check suffices!

## Implementation

Our QAP transformation lives in `../week11/crates/qap/src/polynomials.rs` (accessed via symlinks).

### Data Structures

```rust
use crate::error::QapError;
use ark_ff::PrimeField;
use groth16_math::fields::FieldWrapper;
use groth16_math::polynomial::Polynomial;

/// Quadratic Arithmetic Program
pub struct QAP {
    /// A polynomials: one per witness variable
    pub a_polys: Vec<DensePolynomial>,
    /// B polynomials: one per witness variable
    pub b_polys: Vec<DensePolynomial>,
    /// C polynomials: one per witness variable
    pub c_polys: Vec<DensePolynomial>,
    /// Target polynomial (vanishing at constraint points)
    pub target: DensePolynomial,
}

/// Dense polynomial representation
pub struct DensePolynomial {
    /// Coefficients from lowest-degree to highest
    pub coeffs: Vec<Scalar>,
}
```

### Lagrange Interpolation

```rust
pub fn lagrange_interpolate(points: &[(Scalar, Scalar)]) -> DensePolynomial {
    let n = points.len();

    // Compute Lagrange basis polynomials
    let mut coeffs = vec![Scalar::ZERO; n];

    for i in 0..n {
        let mut l_i = vec![Scalar::ONE; n];

        // Lᵢ(x) = Π (x - xⱼ) / (xᵢ - xⱼ)
        for j in 0..n {
            if j != i {
                let denom = points[i].0 - points[j].0;
                l_i[j] = -points[j].0 / denom;
            }
        }

        // Scale by yᵢ and accumulate
        for k in 0..n {
            coeffs[k] += points[i].1 * l_i[k];
        }
    }

    DensePolynomial { coeffs }
}
```

**Note:** This is a simplified presentation of Lagrange interpolation. The actual implementation in `crates/math/src/polynomial.rs` uses full finite field arithmetic with modular inverses. See the implementation for complete details.

### R1CS to QAP Transformation

```rust
use crate::r1cs::R1CS;

pub fn r1cs_to_qap(r1cs: &R1CS) -> QAP {
    let num_constraints = r1cs.a.num_rows();
    let num_vars = r1cs.a.num_cols();

    // Construct A, B, C polynomials for each variable
    let mut a_polys = Vec::with_capacity(num_vars);
    let mut b_polys = Vec::with_capacity(num_vars);
    let mut c_polys = Vec::with_capacity(num_vars);

    for var in 0..num_vars {
        // Extract column j from matrix A
        let points: Vec<_> = (1..=num_constraints)
            .map(|i| (Scalar::from(i as u64), r1cs.a[(i-1, var)]))
            .collect();

        a_polys.push(lagrange_interpolate(&points));

        // Repeat for B and C matrices
        // ... (similar code)
    }

    // Target polynomial: t(x) = Π (x - i)
    let target_coeffs: Vec<_> = (0..=num_constraints)
        .map(|i| {
            if i == 0 {
                Scalar::ONE
            } else {
                // Compute coefficients of expanded product
                // ... (implementation)
            }
        })
        .collect();

    QAP {
        a_polys,
        b_polys,
        c_polys,
        target: DensePolynomial { coeffs: target_coeffs },
    }
}
```

### Polynomial Division

```rust
use crate::error::{DivisionError, QapError};

pub fn poly_div(numerator: &DensePolynomial, denominator: &DensePolynomial)
    -> Result<DensePolynomial, DivisionError>
{
    // Long division algorithm
    let mut quotient = vec![Scalar::ZERO; numerator.coeffs.len()];
    let mut remainder = numerator.coeffs.clone();

    for i in (0..numerator.coeffs.len()).rev() {
        if remainder.len() <= i || denominator.coeffs.len() == 0 {
            break;
        }

        let factor = remainder[i] / denominator.coeffs.last().unwrap();
        quotient[i] = factor;

        // Subtract factor * denominator from remainder
        for (j, coeff) in denominator.coeffs.iter().enumerate() {
            remainder[j + i - denominator.coeffs.len() + 1] -= factor * coeff;
        }
    }

    // Check remainder is zero
    if remainder.iter().all(|c| c == &Scalar::ZERO) {
        Ok(DensePolynomial { coeffs: quotient })
    } else {
        Err(DivisionError::NonZeroRemainder)
    }
}
```

## Worked Example: Multiplier Circuit

Let's trace the R1CS → QAP transformation for our multiplier circuit: a × b = c.

### R1CS Representation

**Variables:** w = [1, a, b, c] (index 0 is the constant 1)

**Constraints:**
1. w[1] = a  (assignment)
2. w[2] = b  (assignment)
3. w[1] · w[2] = w[3]  (multiplication)

**Matrices (3×4):**

```
A = [[0, 1, 0, 0],     B = [[1, 0, 0, 0],     C = [[0, 1, 0, 0],
     [0, 0, 1, 0],          [1, 0, 0, 0],          [0, 0, 1, 0],
     [0, 1, 0, 0]]          [0, 0, 1, 0],          [0, 0, 0, 1]]
```

### Construct Polynomials

**For variable w[1] (column 1 of A, B, C):**
- A column: [1, 0, 1] at x = [1, 2, 3]
- Interpolate: A₁(x) passes through (1,1), (2,0), (3,1)
- Using Lagrange interpolation: A₁(x) = 0.5(x-2)(x-3) - 0 + 0.5(x-1)(x-2) = x² - 3x + 3

Similarly:
- B column: [0, 0, 0], so B₁(x) = 0
- C column: [0, 0, 0], so C₁(x) = 0

**For variable w[2] (column 2):**
- A column: [0, 1, 0]
- A₂(x) passes through (1,0), (2,1), (3,0)
- A₂(x) = -x² + 4x - 4

- B column: [0, 0, 1]
- B₂(x) passes through (1,0), (2,0), (3,1)
- B₂(x) = 0.5(x-1)(x-2) = 0.5x² - 1.5x + 1

- C column: [0, 0, 0], so C₂(x) = 0

**For variable w[3] (column 3):**
- A column: [0, 0, 0], so A₃(x) = 0
- B column: [0, 0, 0], so B₃(x) = 0
- C column: [0, 0, 1], so C₃(x) = 0.5x² - 1.5x + 1 (same as B₂)

**For constant w[0] = 1:**
- A₀(x) = 0, B₀(x) = 1, C₀(x) = 0

### Final Polynomials

A(x) = w[0]·A₀(x) + w[1]·A₁(x) + w[2]·A₂(x) + w[3]·A₃(x)
     = 1·0 + a·(x² - 3x + 3) + b·(-x² + 4x - 4) + c·0

B(x) = w[0]·B₀(x) + w[1]·B₁(x) + w[2]·B₂(x) + w[3]·B₃(x)
     = 1·1 + a·0 + b·(0.5x² - 1.5x + 1) + c·0

C(x) = w[0]·C₀(x) + w[1]·C₁(x) + w[2]·C₂(x) + w[3]·C₃(x)
     = 1·0 + a·0 + b·0 + c·(0.5x² - 1.5x + 1)

### Target Polynomial

t(x) = (x-1)(x-2)(x-3) = x³ - 6x² + 11x - 6

### Verification

For a = 3, b = 5, c = 15:
- A(x) = 3(x² - 3x + 3) + 5(-x² + 4x - 4) = -2x² + 11x - 11
- B(x) = 1 + 5(0.5x² - 1.5x + 1) = 2.5x² - 7.5x + 6
- C(x) = 15(0.5x² - 1.5x + 1) = 7.5x² - 22.5x + 15

Check at x = 1.5 (random point):
- A(1.5) = -2(2.25) + 11(1.5) - 11 = -4.5 + 16.5 - 11 = 1
- B(1.5) = 2.5(2.25) - 7.5(1.5) + 6 = 5.625 - 11.25 + 6 = 0.375
- C(1.5) = 7.5(2.25) - 22.5(1.5) + 15 = 16.875 - 33.75 + 15 = -1.875
- t(1.5) = 1.5³ - 6(1.5²) + 11(1.5) - 6 = 3.375 - 13.5 + 16.5 - 6 = 0.375

A·B - C = (1)(0.375) - (-1.875) = 0.375 + 1.875 = 2.25
t·h = 0.375 · h  →  h = 6

Check: (A·B - C)(x) = 2.25 at x=1.5
We can verify: A·B - C should be divisible by t(x)
(A·B - C) / t = h(x) where h(1.5) = 6

Division succeeds! Computation is valid.

**Note:** The calculations above use floating-point arithmetic for clarity. In the actual implementation, all operations use finite field arithmetic (mod p), which provides exact results.

## Summary

**Key Takeaways:**
1. R1CS constraint checking is O(n) - must verify every constraint
2. QAP transforms constraints into polynomials using Lagrange interpolation
3. Polynomial divisibility replaces constraint checking
4. Single-point verification suffices due to polynomial properties
5. This O(n) → O(1) reduction is what makes zk-SNARKs practical

**Next Chapter:** We need elliptic curves and pairings to evaluate these polynomials "in the exponent" - this enables the zero-knowledge property.

## Further Reading

- [Pinocchio Protocol Paper](https://eprint.iacr.org/2013/279) - Introduced QAP-based zk-SNARKs
- [Lagrange Interpolation](https://en.wikipedia.org/wiki/Lagrange_polynomial) - Mathematical background
- [ark-poly Documentation](https://docs.rs/ark-poly/) - Polynomial operations in Rust
