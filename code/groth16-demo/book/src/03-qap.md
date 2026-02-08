# Quadratic Arithmetic Programs (QAP)

Now that we understand R1CS, let's transform those matrix constraints into polynomial equations. This is where the real magic happens!

## Learning Objectives

After this chapter, you will understand:
- What QAP is and why it's more powerful than R1CS
- How to transform R1CS constraints into polynomial divisibility
- The target polynomial and why it matters
- Lagrange interpolation for constructing QAP polynomials
- How polynomial divisibility proves constraint satisfaction

## Motivating Example: From R1CS to QAP

Recall from Chapter 2 that our multiplier circuit has one R1CS constraint:
```text
A = [0, 0, 1, 0]  ← selects 'a'
B = [0, 0, 0, 1]  ← selects 'b'
C = [0, 1, 0, 0]  ← selects 'c'
```

**The problem**: R1CS requires checking each constraint individually. For large circuits with thousands of constraints, this is slow!

**The QAP solution**: Transform the constraints into a single polynomial divisibility check:
```text
P(x) = A(x) · B(x) - C(x)
Is P(x) divisible by T(x)?
```

If yes, the witness satisfies ALL constraints at once!

## Theory Deep Dive: What is QAP?

### The Big Picture

**QAP (Quadratic Arithmetic Program)** is a transformation that converts R1CS constraints into polynomial equations. Instead of checking n constraints separately, we check if one polynomial divides another.

**Key insight**: Polynomial divisibility at specific points (1, 2, ..., n) is equivalent to satisfying n R1CS constraints!

### From R1CS Matrices to Polynomials

For each variable j (0 to m), we create three polynomials:
- **Aⱼ(x)**: interpolates the A-coefficients of variable j across all constraints
- **Bⱼ(x)**: interpolates the B-coefficients of variable j across all constraints
- **Cⱼ(x)**: interpolates the C-coefficients of variable j across all constraints

**Example**: Suppose we have 2 constraints and variable j appears as:
- Constraint 1: A₁ⱼ = 1, B₁ⱼ = 0, C₁ⱼ = 2
- Constraint 2: A₂ⱼ = 3, B₂ⱼ = 1, C₂ⱼ = 0

Then we construct:
```text
Aⱼ(x): passes through points (1, 1) and (2, 3)
Bⱼ(x): passes through points (1, 0) and (2, 1)
Cⱼ(x): passes through points (1, 2) and (2, 0)
```

### The Target Polynomial T(x)

The **target polynomial** is:
```text
T(x) = (x - 1)(x - 2)...(x - n)
```

Where n is the number of constraints.

**Key property**: T(x) has roots exactly at x = 1, 2, ..., n (the constraint indices).

### The QAP Satisfaction Check

Given a witness z = [z₀, z₁, ..., zₘ], we:
1. Compute A(x) = Σⱼ zⱼ · Aⱼ(x)
2. Compute B(x) = Σⱼ zⱼ · Bⱼ(x)
3. Compute C(x) = Σⱼ zⱼ · Cⱼ(x)
4. Compute P(x) = A(x) · B(x) - C(x)
5. Check if P(x) is divisible by T(x)

**Why this works**:
- P(i) = A(i) · B(i) - C(i) for i = 1, ..., n
- If the witness satisfies constraint i, then P(i) = 0
- If P(i) = 0 for all i = 1, ..., n, then T(x) divides P(x)
- Checking one polynomial division is faster than checking n constraints!

### Lagrange Interpolation

To construct Aⱼ(x), Bⱼ(x), Cⱼ(x), we use **Lagrange interpolation**:

Given points (x₁, y₁), (x₂, y₂), ..., (xₙ, yₙ), the unique polynomial passing through them is:

```text
P(x) = Σᵢ yᵢ · Lᵢ(x)
```

Where Lᵢ(x) is the i-th Lagrange basis polynomial:
```text
Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ) / (xᵢ - xⱼ)
```

**Example**: Points (1, 2), (2, 4)

```text
L₁(x) = (x - 2) / (1 - 2) = 2 - x
L₂(x) = (x - 1) / (2 - 1) = x - 1

P(x) = 2·L₁(x) + 4·L₂(x)
     = 2(2 - x) + 4(x - 1)
     = 4 - 2x + 4x - 4
     = 2x
```

Verify: P(1) = 2 ✓, P(2) = 4 ✓

## Implementation: QAP in Rust

Now let's see how QAP is implemented in our codebase.

### R1CS to QAP Transformation

From `crates/qap/src/polynomials.rs:60-129`:

```rust,ignore
/// Transforms an R1CS constraint system into a Quadratic Arithmetic Program.
///
/// For each variable j (0..m), this creates three polynomials Aⱼ(x), Bⱼ(x), Cⱼ(x)
/// such that for any constraint i (1..n):
///   Aⱼ(i) = coefficient of variable j in A vector of constraint i
pub fn r1cs_to_qap<F>(
    constraints: &[R1CSConstraint<F>],
    num_variables: usize,
) -> Result<QapPolynomials<F>, QapError>
where
    F: PrimeField,
{
    let n = constraints.len();
    let mut a_polys = Vec::with_capacity(num_variables);
    let mut b_polys = Vec::with_capacity(num_variables);
    let mut c_polys = Vec::with_capacity(num_variables);

    for j in 0..num_variables {
        // Collect points for variable j across all constraints
        let mut a_points = Vec::with_capacity(n);
        let mut b_points = Vec::with_capacity(n);
        let mut c_points = Vec::with_capacity(n);

        for (i, constraint) in constraints.iter().enumerate() {
            let x = F::from((i + 1) as u64); // 1-based constraint index

            let a_coeff = constraint.a.get(&j)
                .cloned().unwrap_or_else(FieldWrapper::zero);
            let b_coeff = constraint.b.get(&j)
                .cloned().unwrap_or_else(FieldWrapper::zero);
            let c_coeff = constraint.c.get(&j)
                .cloned().unwrap_or_else(FieldWrapper::zero);

            a_points.push((x, a_coeff));
            b_points.push((x, b_coeff));
            c_points.push((x, c_coeff));
        }

        // Interpolate polynomials for this variable
        let a_poly = lagrange_interpolate(&a_points)?;
        let b_poly = lagrange_interpolate(&b_points)?;
        let c_poly = lagrange_interpolate(&c_points)?;

        a_polys.push(a_poly);
        b_polys.push(b_poly);
        c_polys.push(c_poly);
    }

    Ok((a_polys, b_polys, c_polys))
}
```

**Key observations**:
- For each variable, we collect its coefficients across all constraints
- We use 1-based indexing for constraint positions (x = 1, 2, ..., n)
- Missing coefficients default to 0 (sparse representation)

### Lagrange Interpolation Implementation

From `crates/qap/src/polynomials.rs:152-238`:

```rust,ignore
pub fn lagrange_interpolate<F>(
    points: &[(F, FieldWrapper<F>)],
) -> Result<Polynomial<F>, QapError>
where
    F: PrimeField,
{
    let n = points.len();

    // Start with zero polynomial
    let mut result_coeffs = vec![FieldWrapper::<F>::zero(); n];

    for i in 0..n {
        let (xi, yi) = &points[i];

        // Compute Lagrange basis polynomial Lᵢ(x)
        let mut li_coeffs = vec![FieldWrapper::<F>::one()];
        let mut denominator = FieldWrapper::<F>::one();

        for (j, xj) in points.iter().enumerate() {
            if i == j { continue; }

            // Multiply by (x - xj)
            let mut new_coeffs = vec![FieldWrapper::<F>::zero(); li_coeffs.len() + 1];
            for (k, coeff) in li_coeffs.iter().enumerate() {
                new_coeffs[k + 1] += coeff.clone();  // x * coeff
                new_coeffs[k] -= coeff.clone() * FieldWrapper::<F>::from(xj.0);  // -xj * coeff
            }
            li_coeffs = new_coeffs;

            // Multiply denominator by (xi - xj)
            denominator = denominator * (FieldWrapper::<F>::from(*xi) - FieldWrapper::<F>::from(xj.0));
        }

        // Scale by yi / denominator
        let inv_denominator = FieldWrapper::<F>::from(denominator.value.inverse().unwrap());
        let scalar = yi.clone() * inv_denominator;

        // Add scaled Lᵢ(x) to result
        for (k, coeff) in li_coeffs.iter().enumerate() {
            if k < result_coeffs.len() {
                result_coeffs[k] += coeff.clone() * scalar.clone();
            }
        }
    }

    Ok(Polynomial::new(result_coeffs))
}
```

### Polynomial Divisibility Check

From `crates/qap/src/divisibility.rs:46-99`:

```rust,ignore
/// Checks if a witness polynomial is divisible by the target polynomial.
pub fn check_divisibility<F>(
    witness: &[FieldWrapper<F>],
    a_polynomials: &[Polynomial<F>],
    b_polynomials: &[Polynomial<F>],
    c_polynomials: &[Polynomial<F>],
    target: &Polynomial<F>,
) -> Result<bool, QapError>
where
    F: PrimeField,
{
    // Compute a(x) = Σⱼ witness[j] · Aⱼ(x)
    let mut a = Polynomial::<F>::new(vec![FieldWrapper::zero()]);
    for (j, poly) in a_polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        a = a + scaled;
    }

    // Compute b(x) = Σⱼ witness[j] · Bⱼ(x)
    let mut b = Polynomial::<F>::new(vec![FieldWrapper::zero()]);
    for (j, poly) in b_polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        b = b + scaled;
    }

    // Compute c(x) = Σⱼ witness[j] · Cⱼ(x)
    let mut c = Polynomial::<F>::new(vec![FieldWrapper::zero()]);
    for (j, poly) in c_polynomials.iter().enumerate() {
        let scaled = scale_polynomial(poly, &witness[j]);
        c = c + scaled;
    }

    // Compute p(x) = a(x) · b(x) - c(x)
    let ab = a.clone() * b.clone();
    let p = ab - c;

    // Check if p(x) is divisible by t(x)
    let (_quotient, remainder) = polynomial_long_division(&p, target)?;

    // Witness is valid iff remainder is zero
    Ok(remainder.is_zero())
}
```

### Target Polynomial

From `crates/qap/src/divisibility.rs:218-243`:

```rust,ignore
/// Constructs the target polynomial t(x) = ∏ᵢ₌₁ⁿ (x - i).
pub fn target_polynomial<F>(num_constraints: usize) -> Polynomial<F>
where
    F: PrimeField,
{
    if num_constraints == 0 {
        return Polynomial::<F>::new(vec![]);
    }

    // Start with (x - 1)
    let mut result = Polynomial::<F>::new(vec![
        FieldWrapper::<F>::zero() - FieldWrapper::<F>::one(),  // -1
        FieldWrapper::<F>::one(),                              // x
    ]);

    // Multiply by (x - i) for i = 2..num_constraints
    for i in 2..=num_constraints {
        let i_field = FieldWrapper::<F>::from(i as u64);
        let factor = Polynomial::<F>::new(vec![
            FieldWrapper::<F>::zero() - i_field,  // -i
            FieldWrapper::<F>::one(),              // x
        ]);
        result = result * factor;
    }

    result
}
```

## Running the Code

Let's see QAP transformation in action.

### Example: Multiplier Circuit

For the multiplier `a × b = c` with witness `z = [1, 12, 3, 4]`:

```text
Constraint 1: a × b = c
A = [0, 0, 1, 0]  (selects a)
B = [0, 0, 0, 1]  (selects b)
C = [0, 1, 0, 0]  (selects c)
```

**Single constraint QAP**:
- T(x) = (x - 1) = x - 1
- A₀(x) = 0, A₁(x) = 0, A₂(x) = 1, A₃(x) = 0
- B₀(x) = 0, B₁(x) = 0, B₂(x) = 0, B₃(x) = 1
- C₀(x) = 0, C₁(x) = 1, C₂(x) = 0, C₃(x) = 0

**Check divisibility**:
```text
A(x) = Σⱼ zⱼ · Aⱼ(x) = 1·0 + 12·0 + 3·1 + 4·0 = 3
B(x) = Σⱼ zⱼ · Bⱼ(x) = 1·0 + 12·0 + 3·0 + 4·1 = 4
C(x) = Σⱼ zⱼ · Cⱼ(x) = 1·0 + 12·1 + 3·0 + 4·0 = 12

P(x) = A(x) · B(x) - C(x) = 3·4 - 12 = 0

Is 0 divisible by T(x)? Yes! (0 = 0 · T(x))
```

### Two-Constraint Example

Let's create a more interesting example with 2 constraints:

```text
Constraint 1: a × b = c
Constraint 2: c × c = d

A vectors:
  C1: [0, 0, 1, 0, 0]  (selects a)
  C2: [0, 0, 0, 1, 0]  (selects c)

B vectors:
  C1: [0, 0, 0, 1, 0]  (selects b)
  C2: [0, 0, 0, 1, 0]  (selects c)

C vectors:
  C1: [0, 1, 0, 0, 0]  (selects c)
  C2: [0, 0, 0, 0, 1]  (selects d)

Target: T(x) = (x - 1)(x - 2)
```

For witness `z = [1, 4, 2, 2, 16]`:
```text
Constraint 1: 2 × 2 = 4 ✓
Constraint 2: 4 × 4 = 16 ✓
```

## Connection to Groth16

QAP is the **bridge** between R1CS and elliptic curve pairings:

```text
Computation
    ↓
R1CS (matrix constraints)
    ↓
QAP (polynomial divisibility) ← You are here
    ↓
Elliptic Curve Pairings (Chapter 4)
    ↓
Zero-Knowledge Proof!
```

**Key insight**: Polynomial equations can be "checked in the exponent" using pairings, enabling efficient verification!

## Exercises

1. **Lagrange interpolation**:
   ```text
   Given points (1, 3), (2, 5), (3, 9), find the polynomial P(x).
   Verify: P(1) = 3, P(2) = 5, P(3) = 9
   ```

2. **QAP construction**:
   ```text
   Given constraints:
   C1: x × y = z
   C2: x × x = w

   Construct A₂(x) (for variable x).
   Points: (1, 1), (2, 1)
   ```

3. **Divisibility check**:
   ```text
   P(x) = x² - 3x + 2
   T(x) = (x - 1)(x - 2)

   Is P(x) divisible by T(x)?
   Hint: Check if P(1) = 0 and P(2) = 0
   ```

4. **Challenge question**:
   ```text
   Why do we need at least 2 constraints for QAP?
   What happens with only 1 constraint?
   ```

## Further Reading

- **QAP Paper**: [GGPR13 (Bitansky et al.)](https://eprint.iacr.org/2013/718)
- **Lagrange Interpolation**: [Brilliant Wiki](https://brilliant.org/wiki/lagrange-interpolation/)
- **Polynomial Division**: [Khan Academy: Dividing Polynomials](https://www.khanacademy.org/math/algebra2/x2ec2f6f830c9fb89:poly-div)

---

**Ready for elliptic curves? Continue to [Chapter 4: Elliptic Curves and Pairings](./04-pairings.md)**
