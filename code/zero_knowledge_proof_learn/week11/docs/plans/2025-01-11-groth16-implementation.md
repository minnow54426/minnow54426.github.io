# Groth16 Demo Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a comprehensive Rust project demonstrating the Groth16 zero-knowledge proof system with rigorous mathematical detail and practical implementation, serving as an educational tutorial, implementation reference, and paper companion guide.

**Architecture:**
- Modular workspace with 6 crates: math, r1cs, qap, groth16, circuits
- Hybrid approach: Use arkworks for crypto primitives (fields, curves, pairings), implement Groth16 protocol logic ourselves
- Progressive examples: 5 circuits of increasing complexity (multiplier → cubic → hash → merkle → range proof)
- Tutorial book (mdbook) with 14 chapters following concept-first approach

**Tech Stack:**
- Rust 2021 edition
- arkworks-rs v0.4 (ark-ff, ark-ec, ark-bn254, ark-poly, ark-groth16, ark-relations, ark-r1cs-std, ark-crypto-primitives)
- serde/bincode for serialization
- anyhow/thiserror for error handling
- proptest for property-based testing
- mdbook for tutorial documentation

---

## Phase 1: Mathematical Foundations (math crate)

### Task 1: Set up field operations wrapper

**Files:**
- Modify: `crates/math/src/fields.rs`
- Test: `crates/math/src/fields_tests.rs`

**Step 1: Write failing test for field wrapper**

```rust
// crates/math/src/fields_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::PrimeField;
    use ark_bn254::Fq;

    #[test]
    fn test_field_wrapper_creation() {
        let field = FieldWrapper::<Fq>::from(5u64);
        assert_eq!(field.value, Fq::from(5u64));
    }

    #[test]
    fn test_field_wrapper_arithmetic() {
        let a = FieldWrapper::<Fq>::from(5u64);
        let b = FieldWrapper::<Fq>::from(3u64);
        let sum = a + b;
        assert_eq!(sum.value, Fq::from(8u64));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-math --lib`
Expected: FAIL with "field_wrapper not found"

**Step 3: Write minimal field wrapper implementation**

```rust
// crates/math/src/fields.rs

use ark_ff::PrimeField;
use serde::{Serialize, Deserialize};

/// Wrapper around arkworks field elements for type safety
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldWrapper<F: PrimeField> {
    pub value: F,
}

impl<F: PrimeField> FieldWrapper<F> {
    pub fn from(value: impl Into<F>) -> Self {
        Self { value: value.into() }
    }

    pub fn zero() -> Self {
        Self { value: F::zero() }
    }

    pub fn one() -> Self {
        Self { value: F::one() }
    }
}

impl<F: PrimeField> std::ops::Add for FieldWrapper<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<F: PrimeField> std::ops::Mul for FieldWrapper<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}
```

**Step 4: Add test module to lib.rs**

```rust
// crates/math/src/lib.rs

pub mod fields;

#[cfg(test)]
mod fields_tests;
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p groth16-math test_field_wrapper`
Expected: PASS (2 tests)

**Step 6: Commit**

```bash
cd /Users/boycrypt/code/python/website/.worktrees/groth16-demo/code/groth16-demo
git add crates/math/src/
git commit -m "feat(math): add field wrapper with tests

Implement FieldWrapper type for type-safe field operations.
Tests cover creation and basic arithmetic."
```

---

### Task 2: Add polynomial operations

**Files:**
- Modify: `crates/math/src/polynomial.rs`
- Test: `crates/math/src/polynomial_tests.rs`

**Step 1: Write failing test for polynomial evaluation**

```rust
// crates/math/src/polynomial_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use crate::fields::FieldWrapper;

    #[test]
    fn test_polynomial_evaluation() {
        // p(x) = 2x + 3
        let coeffs = vec![
            FieldWrapper::<Fq>::from(3u64),  // constant term
            FieldWrapper::<Fq>::from(2u64),  // x term
        ];
        let poly = Polynomial::new(coeffs);

        // p(5) = 2*5 + 3 = 13
        let x = FieldWrapper::<Fq>::from(5u64);
        let result = poly.evaluate(&x);
        assert_eq!(result.value, Fq::from(13u64));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-math test_polynomial`
Expected: FAIL with "polynomial not found"

**Step 3: Implement polynomial type**

```rust
// crates/math/src/polynomial.rs

use crate::fields::FieldWrapper;
use ark_ff::PrimeField;

#[derive(Clone, Debug)]
pub struct Polynomial<F: PrimeField> {
    pub coeffs: Vec<FieldWrapper<F>>,
}

impl<F: PrimeField> Polynomial<F> {
    pub fn new(coeffs: Vec<FieldWrapper<F>>) -> Self {
        Self { coeffs }
    }

    pub fn evaluate(&self, x: &FieldWrapper<F>) -> FieldWrapper<F> {
        let mut result = FieldWrapper::zero();
        let mut x_pow = FieldWrapper::one();

        for coeff in &self.coeffs {
            result = result + coeff.clone() * x_pow.clone();
            x_pow = x_pow.clone() * x.clone();
        }

        result
    }

    pub fn degree(&self) -> usize {
        if self.coeffs.is_empty() {
            return 0;
        }
        self.coeffs.len() - 1
    }
}
```

**Step 4: Update lib.rs**

```rust
// crates/math/src/lib.rs

pub mod fields;
pub mod polynomial;

#[cfg(test)]
mod polynomial_tests;
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p groth16-math test_polynomial_evaluation`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/math/src/
git commit -m "feat(math): add polynomial evaluation

Implement Polynomial type with coefficient-based evaluation.
Tests cover basic polynomial evaluation."
```

---

### Task 3: Add pairing operations wrapper

**Files:**
- Modify: `crates/math/src/pairing.rs`
- Test: `crates/math/src/pairing_tests.rs`

**Step 1: Write failing test for pairing check**

```rust
// crates/math/src/pairing_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{Bn254, G1Projective as G1, G2Projective as G2};
    use ark_ec::{PairingEngine, ProjectiveCurve};

    #[test]
    fn test_pairing_bilinearity() {
        let rng = &mut rand::thread_rng();
        let a = Fq::rand(rng);
        let b = Fq2::rand(rng);

        // e(g1^a, g2^b) = e(g1, g2)^(ab)
        let g1 = G1::prime_subgroup_generator();
        let g2 = G2::prime_subgroup_generator();

        let g1_a = g1.mul(a);
        let g2_b = g2.mul(b.into_repr());

        let left = Bn254::pairing(g1_a, g2_b);
        let right = Bn254::final_exponentiation(
            &Bn254::miller_loop(&[(g1.into(), g2.into())])
                .unwrap()
        ).unwrap();

        // They should be equal (up to final exponentiation)
        assert_eq!(left, right);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-math test_pairing`
Expected: FAIL with module not found

**Step 3: Implement pairing wrapper**

```rust
// crates/math/src/pairing.rs

use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_bn254::Bn254;
use ark_std::rand;

pub struct PairingGroup;

impl PairingGroup {
    pub fn verify_pairing_equation(
        a: &<Bn254 as PairingEngine>::G1Projective,
        b: &<Bn254 as PairingEngine>::G2Projective,
        c: &<Bn254 as PairingEngine>::G1Projective,
        d: &<Bn254 as PairingEngine>::G2Projective,
    ) -> bool {
        // e(a, b) == e(c, d)
        let left = Bn254::pairing(*a, *b);
        let right = Bn254::pairing(*c, *d);
        left == right
    }
}
```

**Step 4: Update lib.rs and add tests**

```rust
// crates/math/src/lib.rs
pub mod pairing;

#[cfg(test)]
mod pairing_tests;
```

**Step 5: Run tests**

Run: `cargo test -p groth16-math test_pairing`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/math/src/
git commit -m "feat(math): add pairing wrapper

Implement basic pairing verification wrapper.
Tests check bilinearity property."
```

---

## Phase 2: R1CS Implementation (r1cs crate)

### Task 4: Implement R1CS constraint representation

**Files:**
- Create: `crates/r1cs/src/constraint.rs`
- Modify: `crates/r1cs/src/lib.rs`
- Test: `crates/r1cs/src/constraint_tests.rs`

**Step 1: Write failing test for R1CS constraint**

```rust
// crates/r1cs/src/constraint_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::fields::FieldWrapper;

    #[test]
    fn test_r1cs_constraint_creation() {
        // Simple constraint: a * b = c
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64)); // a
        constraint.add_b_variable(0, FieldWrapper::<Fq>::from(1u64)); // b
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64)); // c

        assert_eq!(constraint.num_variables(), 2);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-r1cs test_r1cs`
Expected: FAIL

**Step 3: Implement R1CS constraint**

```rust
// crates/r1cs/src/constraint.rs

use ark_ff::PrimeField;
use groth16_math::fields::FieldWrapper;
use std::collections::HashMap;

pub struct R1CSConstraint<F: PrimeField> {
    pub a: HashMap<usize, FieldWrapper<F>>,
    pub b: HashMap<usize, FieldWrapper<F>>,
    pub c: HashMap<usize, FieldWrapper<F>>,
}

impl<F: PrimeField> R1CSConstraint<F> {
    pub fn new() -> Self {
        Self {
            a: HashMap::new(),
            b: HashMap::new(),
            c: HashMap::new(),
        }
    }

    pub fn add_a_variable(&mut self, index: usize, coeff: FieldWrapper<F>) {
        self.a.insert(index, coeff);
    }

    pub fn add_b_variable(&mut self, index: usize, coeff: FieldWrapper<F>) {
        self.b.insert(index, coeff);
    }

    pub fn add_c_variable(&mut self, index: usize, coeff: FieldWrapper<F>) {
        self.c.insert(index, coeff);
    }

    pub fn num_variables(&self) -> usize {
        *[self.a.len(), self.b.len(), self.c.len()]
            .iter()
            .max()
            .unwrap_or(&0)
    }
}
```

**Step 4: Update lib.rs**

```rust
// crates/r1cs/src/lib.rs
pub mod constraint;

#[cfg(test)]
mod constraint_tests;
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p groth16-r1cs test_r1cs_constraint_creation`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/r1cs/src/
git commit -m "feat(r1cs): add constraint representation

Implement R1CSConstraint with a, b, c coefficient maps.
Tests cover constraint creation."
```

---

### Task 5: Implement witness satisfaction checking

**Files:**
- Modify: `crates/r1cs/src/witness.rs`
- Test: `crates/r1cs/src/witness_tests.rs`

**Step 1: Write failing test for witness satisfaction**

```rust
// crates/r1cs/src/witness_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::fields::FieldWrapper;

    #[test]
    fn test_witness_satisfaction() {
        // Constraint: a * b = c
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
        constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));

        let witness = vec![
            FieldWrapper::<Fq>::from(3u64),  // a = 3
            FieldWrapper::<Fq>::from(4u64),  // b = 4
            FieldWrapper::<Fq>::from(12u64), // c = 12
        ];

        // Should satisfy: 3 * 4 = 12 ✓
        assert!(constraint.is_satisfied(&witness));
    }

    #[test]
    fn test_witness_violation() {
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
        constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));

        let witness = vec![
            FieldWrapper::<Fq>::from(3u64),  // a = 3
            FieldWrapper::<Fq>::from(4u64),  // b = 4
            FieldWrapper::<Fq>::from(13u64), // c = 13 (wrong!)
        ];

        // Should not satisfy: 3 * 4 != 13 ✗
        assert!(!constraint.is_satisfied(&witness));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-r1cs test_witness`
Expected: FAIL with is_satisfied not found

**Step 3: Implement witness checking**

```rust
// crates/r1cs/src/witness.rs

use crate::constraint::R1CSConstraint;
use ark_ff::PrimeField;
use groth16_math::fields::FieldWrapper;

impl<F: PrimeField> R1CSConstraint<F> {
    pub fn is_satisfied(&self, witness: &[FieldWrapper<F>]) -> bool {
        let a_value = self.evaluate_linear_combination(&self.a, witness);
        let b_value = self.evaluate_linear_combination(&self.b, witness);
        let c_value = self.evaluate_linear_combination(&self.c, witness);

        // Check: a · b = c
        let product = a_value.clone() * b_value;
        product.value == c_value.value
    }

    fn evaluate_linear_combination(
        &self,
        coeffs: &std::collections::HashMap<usize, FieldWrapper<F>>,
        witness: &[FieldWrapper<F>],
    ) -> FieldWrapper<F> {
        let mut result = FieldWrapper::zero();
        for (idx, coeff) in coeffs {
            if let Some(w) = witness.get(*idx) {
                result = result + coeff.clone() * w.clone();
            }
        }
        result
    }
}
```

**Step 4: Update lib.rs**

```rust
// crates/r1cs/src/lib.rs
pub mod constraint;
pub mod witness;

#[cfg(test)]
mod witness_tests;
```

**Step 5: Run tests**

Run: `cargo test -p groth16-r1cs test_witness`
Expected: PASS (2 tests)

**Step 6: Commit**

```bash
git add crates/r1cs/src/
git commit -m "feat(r1cs): add witness satisfaction checking

Implement is_satisfied to verify witnesses satisfy constraints.
Tests cover valid and invalid witnesses."
```

---

## Phase 3: QAP Implementation (qap crate)

### Task 6: Implement R1CS to QAP transformation

**Files:**
- Create: `crates/qap/src/polynomials.rs`
- Test: `crates/qap/src/polynomials_tests.rs`

**Step 1: Write failing test for Lagrange interpolation**

```rust
// crates/qap/src/polynomials_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::{fields::FieldWrapper, polynomial::Polynomial};

    #[test]
    fn test_lagrange_interpolation() {
        // Points: (1, 2), (2, 4), (3, 6)
        // Should get polynomial p(x) = 2x
        let points = vec![
            (Fq::from(1u64), Fq::from(2u64)),
            (Fq::from(2u64), Fq::from(4u64)),
            (Fq::from(3u64), Fq::from(6u64)),
        ];

        let poly = lagrange_interpolate(&points);

        // Test at x=5: p(5) should be 10
        let x = FieldWrapper::from(5u64);
        let result = poly.evaluate(&x);
        assert_eq!(result.value, Fq::from(10u64));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-qap test_lagrange`
Expected: FAIL

**Step 3: Implement Lagrange interpolation**

```rust
// crates/qap/src/polynomials.rs

use ark_ff::{Field, PrimeField};
use groth16_math::{fields::FieldWrapper, polynomial::Polynomial};

pub fn lagrange_interpolate<F: PrimeField>(points: &[(F, F)]) -> Polynomial<FieldWrapper<F>> {
    let n = points.len();
    let mut coeffs = vec![FieldWrapper::<F>::zero(); n];

    for i in 0..n {
        let mut l_i = vec![FieldWrapper::<F>::zero(); n];
        l_i[0] = FieldWrapper::from(points[i].1);

        for j in 0..n {
            if j != i {
                // L_i(x) = (x - x_j) / (x_i - x_j)
                let denom = points[i].0 - points[j].0;
                let denom_inv = denom.inverse().unwrap();

                // Update coefficients for (x - x_j)
                let neg_x_j = FieldWrapper::from(-points[j].0);
                for k in (1..n).rev() {
                    l_i[k] = l_i[k-1].clone() + l_i[k].clone();
                    l_i[k-1] = l_i[k-1].clone() * neg_x_j.clone();
                }
                l_i[0] = l_i[0].clone() * FieldWrapper::from(denom_inv);
            }
        }

        for k in 0..n {
            coeffs[k] = coeffs[k].clone() + l_i[k].clone();
        }
    }

    Polynomial::new(coeffs)
}
```

**Step 4: Update lib.rs and add tests**

```rust
// crates/qap/src/lib.rs
pub mod polynomials;

#[cfg(test)]
mod polynomials_tests;
```

**Step 5: Run test**

Run: `cargo test -p groth16-qap test_lagrange_interpolation`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/qap/src/
git commit -m "feat(qap): add Lagrange interpolation

Implement Lagrange interpolation for QAP polynomial construction.
Tests cover basic interpolation."
```

---

### Task 7: Implement divisibility checking

**Files:**
- Create: `crates/qap/src/divisibility.rs`
- Test: `crates/qap/src/divisibility_tests.rs`

**Step 1: Write failing test for polynomial divisibility**

```rust
// crates/qap/src/divisibility_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::polynomial::Polynomial;
    use groth16_math::fields::FieldWrapper;

    #[test]
    fn test_polynomial_divisibility() {
        // p(x) = x^2 - 1 = (x-1)(x+1)
        // H(x) = x + 1
        // Z(x) = x - 1
        let p = Polynomial::new(vec![
            FieldWrapper::<Fq>::from((-1i64) as u64), // -1
            FieldWrapper::<Fq>::from(0u64),           // 0x
            FieldWrapper::<Fq>::from(1u64),           // 1x^2
        ]);

        let z = Polynomial::new(vec![
            FieldWrapper::<Fq>::from((-1i64) as u64), // -1
            FieldWrapper::<Fq>::from(1u64),           // 1x
        ]);

        assert!(is_divisible(&p, &z));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-qap test_divisibility`
Expected: FAIL

**Step 3: Implement polynomial division**

```rust
// crates/qap/src/divisibility.rs

use ark_ff::PrimeField;
use groth16_math::polynomial::Polynomial;
use groth16_math::fields::FieldWrapper;

pub fn is_divisible<F: PrimeField>(
    p: &Polynomial<FieldWrapper<F>>,
    divisor: &Polynomial<FieldWrapper<F>>,
) -> bool {
    if divisor.degree() > p.degree() {
        return false;
    }

    match polynomial_division(p, divisor) {
        Some((quotient, remainder)) => {
            // Check if remainder is zero polynomial
            remainder.coeffs.iter().all(|c| c.value.is_zero())
        }
        None => false,
    }
}

fn polynomial_division<F: PrimeField>(
    dividend: &Polynomial<FieldWrapper<F>>,
    divisor: &Polynomial<FieldWrapper<F>>,
) -> Option<(Polynomial<FieldWrapper<F>>, Polynomial<FieldWrapper<F>>)> {
    if divisor.degree() > dividend.degree() {
        return None;
    }

    let mut remainder = dividend.coeffs.clone();
    let divisor_leading = divisor.coeffs.last()?;
    let divisor_degree = divisor.degree();
    let dividend_degree = dividend.degree();

    let mut quotient_coeffs = vec![FieldWrapper::zero(); dividend_degree - divisor_degree + 1];

    for i in (0..=(dividend_degree - divisor_degree)).rev() {
        let remainder_leading = remainder.get(divisor_degree + i)?;
        if remainder_leading.value.is_zero() {
            continue;
        }

        let q_coeff = remainder_leading.clone() * divisor_leading.clone();
        quotient_coeffs[i] = q_coeff.clone();

        // Subtract q_coeff * divisor * x^i from remainder
        for j in 0..=divisor_degree {
            if let Some(r) = remainder.get_mut(i + j) {
                let term = divisor.coeffs.get(j)?.clone() * q_coeff.clone();
                *r = r.clone() - term;
            }
        }
    }

    let quotient = Polynomial::new(quotient_coeffs);
    let remainder_poly = Polynomial::new(
        remainder.into_iter()
            .take(divisor_degree)
            .collect()
    );

    Some((quotient, remainder_poly))
}
```

**Step 4: Add subtraction to FieldWrapper**

```rust
// crates/math/src/fields.rs

impl<F: PrimeField> std::ops::Sub for FieldWrapper<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}
```

**Step 5: Update lib.rs**

```rust
// crates/qap/src/lib.rs
pub mod polynomials;
pub mod divisibility;

#[cfg(test)]
mod divisibility_tests;
```

**Step 6: Run tests**

Run: `cargo test -p groth16-qap test_divisibility`
Expected: PASS

**Step 7: Commit**

```bash
git add crates/
git commit -m "feat(qap): add polynomial divisibility checking

Implement polynomial division for QAP divisibility checks.
Tests cover simple polynomial division."
```

---

## Phase 4: Example 1 - Simple Multiplier (Complete End-to-End)

### Task 8: Build simple multiplier R1CS

**Files:**
- Create: `crates/circuits/src/multiplier.rs`
- Test: `crates/circuits/src/multiplier_tests.rs`

**Step 1: Write failing test for multiplier circuit**

```rust
// crates/circuits/src/multiplier_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use groth16_math::fields::FieldWrapper;

    #[test]
    fn test_multiplier_circuit() {
        let circuit = MultiplierCircuit::new();

        // Witness: [1, a, b, c] where a*b=c
        let a = FieldWrapper::<Fq>::from(3u64);
        let b = FieldWrapper::<Fq>::from(4u64);
        let c = FieldWrapper::<Fq>::from(12u64);

        let witness = circuit.generate_witness(a, b, c);
        assert!(circuit.is_satisfied(&witness));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p groth16-circuits test_multiplier`
Expected: FAIL

**Step 3: Implement multiplier circuit**

```rust
// crates/circuits/src/multiplier.rs

use groth16_r1cs::constraint::R1CSConstraint;
use ark_ff::PrimeField;
use groth16_math::fields::FieldWrapper;

pub struct MultiplierCircuit<F: PrimeField> {
    pub constraints: Vec<R1CSConstraint<F>>,
}

impl<F: PrimeField> MultiplierCircuit<F> {
    pub fn new() -> Self {
        let mut constraints = Vec::new();

        // Constraint 0: a * 1 = a (copy to ensure wire assignment)
        let mut c0 = R1CSConstraint::new();
        c0.add_a_variable(1, FieldWrapper::one());  // a
        c0.add_b_variable(0, FieldWrapper::one());  // 1
        c0.add_c_variable(1, FieldWrapper::one());  // a
        constraints.push(c0);

        // Constraint 1: b * 1 = b (copy)
        let mut c1 = R1CSConstraint::new();
        c1.add_a_variable(2, FieldWrapper::one());  // b
        c1.add_b_variable(0, FieldWrapper::one());  // 1
        c1.add_c_variable(2, FieldWrapper::one());  // b
        constraints.push(c1);

        // Constraint 2: a * b = c
        let mut c2 = R1CSConstraint::new();
        c2.add_a_variable(1, FieldWrapper::one());  // a
        c2.add_b_variable(2, FieldWrapper::one());  // b
        c2.add_c_variable(3, FieldWrapper::one());  // c
        constraints.push(c2);

        Self { constraints }
    }

    pub fn generate_witness(
        &self,
        a: FieldWrapper<F>,
        b: FieldWrapper<F>,
        c: FieldWrapper<F>,
    ) -> Vec<FieldWrapper<F>> {
        vec![
            FieldWrapper::one(),  // 0: constant 1
            a,                    // 1: a
            b,                    // 2: b
            c,                    // 3: c = a*b
        ]
    }

    pub fn is_satisfied(&self, witness: &[FieldWrapper<F>]) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(witness))
    }
}
```

**Step 4: Update lib.rs**

```rust
// crates/circuits/src/lib.rs
pub mod multiplier;

#[cfg(test)]
mod multiplier_tests;
```

**Step 5: Run test**

Run: `cargo test -p groth16-circuits test_multiplier_circuit`
Expected: PASS

**Step 6: Commit**

```bash
git add crates/circuits/src/
git commit -m "feat(circuits): add multiplier circuit

Implement simple multiplier circuit (a * b = c).
Tests cover witness generation and satisfaction."
```

---

### Task 9: Update multiplier demo with real implementation

**Files:**
- Modify: `crates/circuits/examples/multiplier_demo.rs`

**Step 1: Implement demo**

```rust
// crates/circuits/examples/multiplier_demo.rs

use groth16_circuits::multiplier::MultiplierCircuit;
use groth16_math::fields::FieldWrapper;
use ark_bn254::Fq;

fn main() {
    println!("Groth16 Multiplier Circuit Demo");
    println!("===============================");
    println!();

    let circuit = MultiplierCircuit::<Fq>::new();

    let a = FieldWrapper::<Fq>::from(5u64);
    let b = FieldWrapper::<Fq>::from(7u64);
    let c = FieldWrapper::<Fq>::from(35u64);

    println!("Computing {} × {} = {}", a.value, b.value, c.value);
    println!();

    let witness = circuit.generate_witness(a.clone(), b.clone(), c.clone());

    println!("Witness: [1, {}, {}, {}]",
        witness[1].value, witness[2].value, witness[3].value);
    println!();

    if circuit.is_satisfied(&witness) {
        println!("✓ Constraint satisfied!");
    } else {
        println!("✗ Constraint NOT satisfied!");
    }

    println!();
    println!("Next steps:");
    println!("- Convert R1CS to QAP");
    println!("- Implement trusted setup");
    println!("- Generate and verify proofs");
}
```

**Step 2: Run demo**

Run: `cargo run --bin multiplier-demo`
Expected: Output showing constraint satisfaction

**Step 3: Commit**

```bash
git add crates/circuits/examples/
git commit -m "demo(multiplier): update with real implementation

Show working R1CS constraint satisfaction."
```

---

## Phase 5: Tutorial Book Chapters

### Task 10: Write Chapter 0 - Introduction

**Files:**
- Create: `book/src/00-introduction.md`

**Step 1: Create chapter content**

```markdown
# Introduction

## What is Groth16?

Groth16 is a zero-knowledge succinct non-interactive argument of knowledge (zk-SNARK) protocol. It allows a prover to convince a verifier that they know a witness satisfying a constraint system, without revealing the witness itself.

## Why Groth16?

- **Succinct**: Proofs are small (only 3 group elements)
- **Non-interactive**: Single message from prover to verifier
- **Zero-knowledge**: Verifier learns nothing beyond the truth of the statement
- **Efficient verification**: Constant-time pairing checks

## The Groth16 Pipeline

```
Computation → R1CS → QAP → Setup → Prove → Verify
```

1. **Computation**: Express your problem as a circuit
2. **R1CS**: Convert to Rank-1 Constraint System
3. **QAP**: Transform to Quadratic Arithmetic Program
4. **Setup**: Generate proving key (pk) and verification key (vk)
5. **Prove**: Create zero-knowledge proof using pk and witness
6. **Verify**: Check proof using vk and public inputs

## What You'll Learn

This tutorial covers:
- Mathematical foundations (finite fields, polynomials, pairings)
- R1CS and QAP transformations
- The complete Groth16 protocol
- Five working example circuits

Let's get started!
```

**Step 2: Build book**

Run: `mdbook build book`
Expected: Book builds successfully

**Step 3: Commit**

```bash
git add book/src/
git commit -m "docs(book): add Chapter 0 - Introduction

Provide overview of Groth16 and tutorial structure."
```

---

### Task 11: Write Chapter 1 - Mathematical Background

**Files:**
- Create: `book/src/01-math-background.md`

**Step 1: Create chapter content**

```markdown
# Mathematical Background

## Finite Fields

A finite field (or Galois field) is a set with finite number of elements on which addition, subtraction, multiplication, and division are defined.

### Example: Field modulo 5

```
Elements: {0, 1, 2, 3, 4}
3 + 4 = 2 (mod 5)
3 × 4 = 2 (mod 5)
```

### In Rust

```rust
use ark_bn254::Fq;

let a = Fq::from(5u64);
let b = Fq::from(3u64);
let sum = a + b;  // Fq::from(8u64)
```

## Polynomials

A polynomial is an expression of variables and coefficients:

```
p(x) = 3x² + 2x + 1
```

### Evaluation

To evaluate p(x) at x = 5:

```
p(5) = 3(25) + 2(5) + 1 = 75 + 10 + 1 = 86
```

### In Rust

```rust
use groth16_math::polynomial::Polynomial;
use groth16_math::fields::FieldWrapper;

let coeffs = vec![
    FieldWrapper::from(1u64),  // constant
    FieldWrapper::from(2u64),  // x term
    FieldWrapper::from(3u64),  // x² term
];
let p = Polynomial::new(coeffs);
let x = FieldWrapper::from(5u64);
let result = p.evaluate(&x);
```

## Lagrange Interpolation

Given n points, there is a unique polynomial of degree n-1 passing through them.

### Example

Points: (1, 2), (2, 3), (3, 4)

Polynomial: p(x) = x + 1

```
p(1) = 2 ✓
p(2) = 3 ✓
p(3) = 4 ✓
```

### In Rust

```rust
use groth16_qap::polynomials::lagrange_interpolate;
use ark_bn254::Fq;

let points = vec![
    (Fq::from(1u64), Fq::from(2u64)),
    (Fq::from(2u64), Fq::from(3u64)),
    (Fq::from(3u64), Fq::from(4u64)),
];
let poly = lagrange_interpolate(&points);
```

## What's Next

Now that we understand the math foundations, let's learn about Rank-1 Constraint Systems!
```

**Step 2: Build and verify book**

Run: `mdbook build book`
Expected: Success

**Step 3: Commit**

```bash
git add book/src/
git commit -m "docs(book): add Chapter 1 - Mathematical Background

Cover finite fields, polynomials, and Lagrange interpolation."
```

---

## Phase 6: Continue Implementation (Remaining Tasks)

### Remaining High-Level Tasks

**Tasks 12-20**: Continue with remaining mathematical primitives
- Task 12: R1CS system with multiple constraints
- Task 13: Complete R1CS → QAP transformation
- Task 14: Implement QAP divisibility check
- Task 15: Implement trusted setup
- Task 16: Implement proof generation
- Task 17: Implement proof verification
- Task 18: Write Chapters 2-7 of tutorial
- Task 19: Implement remaining example circuits
- Task 20: Add comprehensive documentation and benchmarks

### Implementation Notes

**For each task**:
1. Write failing test first (TDD)
2. Run test to verify failure
3. Implement minimal code to pass
4. Run test to verify success
5. Commit with descriptive message

**File conventions**:
- Test files: `src/<module>_tests.rs`
- Example files: `examples/<name>_demo.rs`
- Book chapters: `book/src/XX-<name>.md`

**Git workflow**:
- Frequent commits (each completed task)
- Commit format: `feat(scope): description`
- Include test coverage in commit message

**Documentation requirements**:
- Rust doc comments on all public APIs
- Tutorial chapters explain concepts before code
- Examples show complete working code

---

## Appendix: Development Workflow

### Running Tests

```bash
# Test all crates
cargo test --workspace

# Test specific crate
cargo test -p groth16-math
cargo test -p groth16-r1cs
cargo test -p groth16-qap
cargo test -p groth16-groth16
cargo test -p groth16-circuits

# Test with output
cargo test -- --nocapture

# Run specific test
cargo test test_multiplier_circuit
```

### Building Documentation

```bash
# Build Rust docs
cargo doc --no-deps --open

# Build tutorial book
mdbook build book
mdbook open book
```

### Code Quality

```bash
# Format code
cargo fmt

# Check linting
cargo clippy -- -D warnings

# All checks
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

### Running Examples

```bash
cargo run --bin multiplier-demo
cargo run --bin cubic-demo
cargo run --bin hash-preimage-demo
cargo run --bin merkle-demo
cargo run --bin range-proof-demo
```

---

**End of Implementation Plan**

**Total Tasks**: 20 major tasks, each broken into 5-6 steps
**Estimated Timeline**: ~100-120 individual commits
**Testing**: Test-driven development throughout
**Documentation**: Rust docs + tutorial book + README

**Ready for execution!**
