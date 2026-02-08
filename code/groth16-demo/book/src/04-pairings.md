# Elliptic Curves and Pairings

We've transformed our computation into polynomial divisibility. Now we need a way to check this divisibility **without revealing the polynomials themselves**. Enter elliptic curve pairings!

## Learning Objectives

After this chapter, you will understand:
- What elliptic curves are and why they're useful in cryptography
- Group operations on elliptic curves
- Bilinear pairings and their magical properties
- How pairings enable efficient ZK verification
- The BN254 curve used in Groth16

## Motivating Example: The Pairing Check

In QAP, we want to verify:
```text
P(x) = H(x) · T(x)
```

**Problem**: If we reveal P(x) and H(x), we leak information about the witness!

**Solution with pairings**: Check the equation **in the exponent**:
```text
e(P₁, G₂) = e(H₁, T₁)
```

Where:
- P₁ = P(x) · G₁ (P encrypted in G₁)
- H₁ = H(x) · G₁ (H encrypted in G₁)
- T₁ = T(x) · G₁ (T encrypted in G₁)
- e(·, ·) is the bilinear pairing

The verifier can check the equation without learning P(x), H(x), or the witness!

## Theory Deep Dive: Elliptic Curves

### What is an Elliptic Curve?

An **elliptic curve** over a finite field is the set of solutions (x, y) to:
```text
y² = x³ + ax + b
```

With a special "point at infinity" denoted ∞ (the identity element).

**Example**: The BN254 curve (used in Groth16):
```text
y² = x³ + 3
```

Over a field with ~2²⁵⁴ elements.

### The Group Law

Elliptic curves form an **abelian group** under the "chord-and-tangent" addition:

**Addition (P + Q = R)**:
1. Draw a line through P and Q
2. Find the third intersection with the curve
3. Reflect across the x-axis

**Doubling (P + P = 2P)**:
1. Draw the tangent line at P
2. Find the second intersection with the curve
3. Reflect across the x-axis

**Scalar multiplication (k·P)**:
```text
k·P = P + P + ... + P (k times)
```

This can be computed efficiently using the "double-and-add" algorithm (O(log k) time).

### Two Groups: G₁ and G₂

In pairing-based cryptography, we use **two groups**:

**G₁**: Points on the curve over the base field 𝔽ₚ
- Smaller elements (more efficient)
- Used for elements that need to be compared/combined

**G₂**: Points on the curve over an extension field 𝔽ₚ²
- Larger elements (less efficient)
- Used for pairing targets

Both groups have the same order (number of elements): a large prime r.

## Theory Deep Dive: Bilinear Pairings

### What is a Pairing?

A **bilinear pairing** is a function:
```text
e: G₁ × G₂ → Gₜ
```

Where Gₜ is a target group (usually 𝔽ₚⁿ).

**Key properties**:

1. **Bilinearity**:
   ```text
   e(a·P, b·Q) = e(P, Q)ᵃ·ᵇ = e(a·P, Q)ᵇ = e(P, b·Q)ᵃ
   ```

2. **Non-degeneracy**:
   ```text
   If e(P, Q) = 1 for all Q, then P = ∞
   ```

3. **Computability**:
   ```text
   e(P, Q) can be computed efficiently
   ```

### The Pairing Equation

The pairing enables us to check equations **in the exponent**:

**Integer equation**:
```text
a = b
```

**Pairing check**:
```text
e(a·G₁, G₂) = e(b·G₁, G₂)
```

This works because:
```text
e(a·G₁, G₂) = e(G₁, G₂)ᵃ
e(b·G₁, G₂) = e(G₁, G₂)ᵇ

If a = b, then e(G₁, G₂)ᵃ = e(G₁, G₂)ᵇ
```

### Product Equations

Pairings also work for products:

**Integer equation**:
```text
a · b = c · d
```

**Pairing check**:
```text
e(a·G₁, b·G₂) = e(c·G₁, d·G₂)
```

This works because:
```text
e(a·G₁, b·G₂) = e(G₁, G₂)ᵃ·ᵇ
e(c·G₁, d·G₂) = e(G₁, G₂)ᶜ·ᵈ
```

## Implementation: Pairings in Rust

Now let's see how pairings are implemented in our codebase.

### The BN254 Curve

We use the BN254 curve from the `arkworks` library:

```rust,ignore
use ark_bn254::{Bn254, G1Affine, G2Affine};
use ark_ec::pairing::Pairing;

// Bn254 is a pairing-friendly curve
type G1 = <Bn254 as Pairing>::G1;  // G₁ group
type G2 = <Bn254 as Pairing>::G2;  // G₂ group
type Gt = <Bn254 as Pairing>::TargetField;  // Gₜ (target field)
```

### Computing Pairings

From `crates/math/src/pairing.rs:6-17`:

```rust,ignore
use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;

pub struct PairingGroup;

impl PairingGroup {
    pub fn verify_pairing_equation(
        a: &<Bn254 as Pairing>::G1,
        b: &<Bn254 as Pairing>::G2,
        c: &<Bn254 as Pairing>::G1,
        d: &<Bn254 as Pairing>::G2,
    ) -> bool {
        // e(a, b) == e(c, d)
        let left = Bn254::pairing(*a, *b);
        let right = Bn254::pairing(*c, *d);
        left == right
    }
}
```

This checks if a·b = c·d in the exponent:
```text
e(a, b) = e(G₁, G₂)ᵃ·ᵇ
e(c, d) = e(G₁, G₂)ᶜ·ᵈ

e(a, b) == e(c, d) ⟺ a·b = c·d
```

### Group Operations

```rust,ignore
use ark_bn254::{G1Projective as G1, G2Projective as G2};
use ark_ec::{AffineRepr, CurveGroup};

// Point addition
let p = G1::from(G1Affine::generator());
let q = G1::from(G1Affine::generator());
let r = p + q;  // 2·G₁

// Scalar multiplication
let scalar = 5u64;
let s = G1::from(G1Affine::generator()) * scalar;  // 5·G₁

// Convert between affine and projective representations
let affine_point = s.into_affine();
let projective_point = G1::from(affine_point);
```

### The Verification Equation

In Groth16, the verification equation is:
```text
e(A, B) = e(α, β) · e(Σ publicᵢ · ICᵢ, γ) · e(C, δ)
```

This checks that:
1. A and B were constructed using the toxic waste (α, β, γ, δ)
2. The witness matches the public inputs
3. The QAP division polynomial exists (H(x) is valid)

We'll see this in detail in Chapter 7!

## Running the Code

### Example: Basic Pairing

```rust,ignore
use ark_bn254::{Bn254, G1Affine, G2Affine};
use ark_ec::pairing::Pairing;

// Get generators
let g1 = G1Affine::generator();
let g2 = G2Affine::generator();

// Compute e(5·G₁, 3·G₂)
let five_g1 = g1 * 5u64;
let three_g2 = g2 * 3u64;
let left = Bn254::pairing(five_g1, three_g2);

// Compute e(15·G₁, G₂)
let fifteen_g1 = g1 * 15u64;
let right = Bn254::pairing(fifteen_g1, g2);

// Check: e(5·G₁, 3·G₂) = e(15·G₁, G₂) because 5·3 = 15
assert_eq!(left, right);
```

### Example: Product Check

```rust,ignore
// Check if a·b = c·d
let a = 5u64;
let b = 3u64;
let c = 15u64;
let d = 1u64;

// e(a·G₁, b·G₂) = e(c·G₁, d·G₂)?
let left = Bn254::pairing(g1 * a, g2 * b);
let right = Bn254::pairing(g1 * c, g2 * d);

// 5·3 = 15·1, so this should be true
assert_eq!(left, right);
```

## Connection to Groth16

Pairings are the **final piece** that makes Groth16 work:

```text
Computation
    ↓
R1CS (matrix constraints)
    ↓
QAP (polynomial divisibility)
    ↓
Elliptic Curve Pairings ← You are here
    ↓
Zero-Knowledge Proof!
```

**Key insight**: Pairings allow us to check polynomial equations without revealing the polynomials themselves!

### The Encryption Trick

We can "encrypt" field elements by multiplying by a generator:
```text
plaintext: x
ciphertext: x·G₁
```

The pairing allows us to check equations on ciphertexts:
```text
Check: a + b = c
Pairing: e(a·G₁, G₂) · e(b·G₁, G₂) = e(c·G₁, G₂)

Check: a · b = c
Pairing: e(a·G₁, b·G₂) = e(c·G₁, G₂)
```

## Security Properties

### The Discrete Logarithm Problem

Given P = x·G, it's computationally infeasible to find x.

This is the **Elliptic Curve Discrete Logarithm Problem (ECDLP)** and is the foundation of elliptic curve cryptography.

### Pairing-Friendly Curves

Not all elliptic curves support efficient pairings. The BN254 curve is specifically designed for:
- Efficient pairing computation
- Security level of ~128 bits
- Compatibility with the BN254 pairing

**Warning**: Curves not designed for pairings may have vulnerabilities!

## Exercises

1. **Scalar multiplication**:
   ```rust
   // Compute 7·G₁ using double-and-add
   // Hint: 7 = 4 + 2 + 1 (binary: 111)
   ```

2. **Pairing check**:
   ```text
   Is e(3·G₁, 4·G₂) = e(6·G₁, 2·G₂)?
   Why or why not?
   ```

3. **Verification equation**:
   ```text
   Given: e(A, B) = e(α, β) · e(public·IC, γ) · e(C, δ)

   If we know public, IC, α, β, γ, δ,
   what can we verify about A, B, C?
   ```

4. **Challenge question**:
   ```text
   Why do we need two groups (G₁ and G₂)?
   What would break if we only used G₁?
   ```

## Further Reading

- **Pairings for Beginners**: [Vitalik Buterin's Blog](https://vitalik.ca/general/2017/01/14/exploring_ecp.html)
- **BN254 Curve**: [Barreto-Naehrig Curves](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html)
- **Pairing-Based Crypto**: [The Pairing-Based Crypto Lounge](https://crypto.stanford.edu/pbc/)

---

**Ready to generate the proving keys? Continue to [Chapter 5: Trusted Setup](./05-trusted-setup.md)**
