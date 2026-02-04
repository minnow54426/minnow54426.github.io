# Chapter 4: Elliptic Curves and Bilinear Pairings

## Introduction

Chapters 1-3 built the algebraic machinery for zk-SNARKs: constraint systems, polynomial transformations, and divisibility checks. But there's a missing piece: how do we evaluate polynomials "in the exponent" to enable zero-knowledge?

The answer is **bilinear pairings** on elliptic curves. Pairings are special functions that let us compare polynomials in two different groups - this is the cryptographic engine that powers Groth16.

**Chapter Goals:**
- Understand elliptic curve basics
- Learn how pairings work (intuitively and mathematically)
- See why pairings enable zero-knowledge proofs
- Understand pairing-friendly curves (BN254, BLS12-381)
- Implement pairing operations in Rust using arkworks

## Why We Need Pairings

### The Zero-Knowledge Problem

In Chapter 3, we learned to verify computations by checking:

```
A(r) · B(r) - C(r) = t(r) · h(r)
```

For a random point r. But this reveals A(r), B(r), C(r), h(r) - not zero-knowledge!

**Solution:** Evaluate everything "in the exponent" using pairings:

Instead of revealing A(r), reveal g₁^A(r). Instead of revealing B(r), reveal g₂^B(r).

The pairing function e(·, ·) has a magical property:

```
e(g₁^A, g₂^B) = e(g₁, g2)^(A·B)
```

This lets us check the polynomial equation without revealing the values!

### What Pairings Give Us

1. **Encryption in the exponent:** g^a reveals nothing about a (discrete log hardness)
2. **Homomorphic comparison:** e(g^a, h^b) = e(g, h)^(ab) lets us compare encrypted values
3. **Cross-group operations:** g₁ and g₂ are in different groups, preventing certain attacks

This is the cryptographic foundation of Groth16.

## Elliptic Curve Fundamentals

### Elliptic Curves Over Finite Fields

An elliptic curve is defined by the Weierstrass equation:

```
y² = x³ + ax + b  (mod p)
```

Where a, b are coefficients and p is a large prime.

**Example:** BN254 curve uses:
```
y² = x³ + 3  (mod p)
where p = 218882428718392752222464057452572750886963111572978236626890378946452626785
```

### Group Operations

**Point Addition (P + Q):**
- Draw line through P and Q
- Find third intersection with curve
- Reflect across x-axis

**Point Doubling (P + P):**
- Draw tangent line at P
- Find second intersection
- Reflect across x-axis

**Scalar Multiplication (k·P):**
- Add P to itself k times
- Equivalent to repeated point addition

**The Point at Infinity (𝒪):**
- Identity element of the group
- Result of P + (-P) = 𝒪

### Group Properties

Elliptic curve points form an abelian group:
1. **Closure:** P + Q is on the curve
2. **Associativity:** (P + Q) + R = P + (Q + R)
3. **Identity:** P + 𝒪 = P
4. **Inverse:** P + (-P) = 𝒪
5. **Commutativity:** P + Q = Q + P

**Discrete Logarithm Hardness:**
Given P and k·P, finding k is computationally infeasible for large groups.
This is the security foundation!

### Subgroup Checks

Not all points on the curve are in the prime-order subgroup we use.

**Point Validity Check:**
1. Point is on the curve: y² = x³ + ax + b
2. Point is in the correct subgroup: [r]P = 𝒪 (where r is group order)

Failure to check leads to **small-subgroup attacks**!

## Bilinear Pairings

### Definition

A bilinear pairing is a function:

```
e: G₁ × G₂ → G_T
```

Where G₁, G₂, G_T are cyclic groups of prime order r, with generators g₁, g₂, g_T.

**Bilinearity Properties:**

1. **Bilinear in first argument:**
   ```
   e(g₁^a, g₂^b) = e(g₁, g₂)^(a·b)
   ```

2. **Bilinear in second argument:**
   ```
   e(g₁^a, g₂^b) = e(g₁^a, g₂^b)  (same as above)
   ```

3. **Non-degenerate:** e(g₁, g₂) ≠ 1 (the identity)

**Intuition:** The pairing "transfers" multiplication from exponents to the group operation.

### The Pairing Equation

This is the **heart of Groth16 verification:**

```
e(A, B) = e(α, β) · e(public_input, γ) · e(C, δ)
```

Where:
- A, B, C are proof elements (group elements)
- α, β, γ, δ are setup parameters
- public_input is the statement being proven

Expanded using bilinearity:

```
e(g₁^(α·s + ...), g₂^(β·s + ...)) = e(g₁^α, g₂^β) · e(g₁^(public), g₂^γ) · e(g₁^(...), g₂^δ)
```

If this equation holds, the proof is valid!

**Connection to QAP:** This pairing equation encodes the QAP divisibility check from Chapter 3. If e(A, B) = e(α, β) · e(input·IC, γ) · e(C, δ), then the prover knows a valid assignment to the QAP polynomials.

### Tate Pairing (Construction)

The most common pairing construction uses **Weil pairing** or **Tate pairing**.

**Tate Pairing Construction:**
1. Choose elliptic curve with "embedding degree" k
2. Define groups:
   - G₁: Points on E(F_p)
   - G₂: Points on E(F_{p^k}) (extension field)
   - G_T: Subgroup of F_{p^k}^* (multiplicative group)

3. Compute:
   ```
   e(P, Q) = f_P(Q)^((p^k - 1) / r)
   ```
   Where f_P is a function derived from point P

### Pairing-Friendly Curves

Not all elliptic curves support efficient pairings. We need:

1. **Low embedding degree:** k is small (typically 12 or 24)
2. **Large prime-order subgroup:** r is ~256 bits for security
3. **Efficient field arithmetic:** p has special form

**BN254 (Barreto-Naehrig):**
- p ≈ 254 bits (fits in 4 × 64-bit limbs)
- Embedding degree k = 12
- Security level: ~128 bits
- Used in: Ethereum (pre-merge), early ZK systems

**BLS12-381:**
- p ≈ 381 bits
- Embedding degree k = 12
- Security level: ~128 bits
- Used in: Zcash, zkSync, modern ZK systems

**Why BLS12-381 is preferred:**
- More secure against certain attacks
- Better performance for multi-scalar multiplication
- Wider ecosystem support

## Implementation

**Implementation Note:** The code examples in this section illustrate the key concepts of pairing operations. The actual implementation in `../week11/crates/groth16/src/verify.rs` uses arkworks 0.4 APIs which may differ from the simplified examples shown here. For production code, refer to the actual implementation files.

Our pairing operations use arkworks' BN254 implementation.

### Data Structures

```rust
use ark_bn254::{Bn254, G1Projective, G2Projective, Fq, Fq2};
use ark_ec::PairingEngine;

// G1: Points on base field E(F_p)
type G1 = G1Projective;

// G2: Points on extension field E(F_{p^2})
type G2 = G2Projective;

// GT: Target group (subgroup of F_{p^12}^*)
type GT = <Bn254 as PairingEngine>::Fqk;

// Pairing engine
type Pairing = Bn254;

// Type alias for clarity
type Scalar = <Bn254 as PairingEngine>::Fr;
```

### Point Operations

```rust
/// Generate a random point in G1
pub fn random_g1() -> G1 {
    let mut rng = rand::thread_rng();
    G1::rand(&mut rng)
}

/// Scalar multiplication
pub fn scalar_mul_g1(point: &G1, scalar: &Scalar) -> G1 {
    point.mul(scalar)
}

/// Point addition
pub fn add_g1(a: &G1, b: &G1) -> G1 {
    a + b
}

/// Check if point is in correct subgroup
pub fn check_subgroup(point: &G1) -> bool {
    // Verify point is in the correct subgroup by checking
    // that multiplying by the group order yields the identity
    let order = <Bn254 as PairingEngine>::ScalarField::MODULUS;
    point.mul(order) == G1::zero()
}
```

### Pairing Computation

```rust
/// Compute pairing e(P, Q)
pub fn pairing(p: G1, q: G2) -> GT {
    <Bn254 as PairingEngine>::pairing(p, q)
}

/// Verify pairing equation: e(A, B) = e(C, D)
pub fn verify_pairing_equation(
    a: G1,
    b: G2,
    c: G1,
    d: G2,
) -> bool {
    let left = pairing(a, b);
    let right = pairing(c, d);
    left == right
}

/// Product of pairings (for multi-pairing checks)
pub fn product_of_pairings(
    terms: &[(G1, G2)],  // [(A₁, B₁), (A₂, B₂), ...]
) -> GT {
    <Bn254 as PairingEngine>::product_of_pairings(terms)
}
```

### Groth16 Verification Equation

```rust
/// Verify Groth16 proof
pub fn verify_groth16(
    proof: &Proof,
    vk: &VerifyingKey,
    public_input: &[Scalar],
) -> bool {
    // Compute pairing equation:
    // e(A, B) = e(α, β) · e(public_input·IC, γ) · e(C, δ)

    // Left side: e(A, B)
    let left = pairing(proof.a, proof.b);

    // Right side: three pairings
    let g1_alpha = vk.alpha_g1;
    let g2_beta = vk.beta_g2;

    // public_input · IC = Σ public_input[i] · IC[i]
    let mut public_ic = vk.ic[0].clone();  // IC[0] is the constant
    for (i, input) in public_input.iter().enumerate() {
        public_ic += vk.ic[i + 1].mul(*input);
    }

    let right = product_of_pairings(&[
        (g1_alpha, g2_beta),                    // e(α, β)
        (public_ic, vk.gamma_g2),               // e(public·IC, γ)
        (proof.c, vk.delta_g2),                 // e(C, δ)
    ]);

    left == right
}
```

### Batch Verification

```rust
/// Verify multiple proofs efficiently
pub fn batch_verify(
    proofs: &[Proof],
    vks: &[VerifyingKey],
    public_inputs: &[Vec<Scalar>],
) -> bool {
    // Combine all pairing checks into one:
    // Σ rᵢ · [e(Aᵢ, Bᵢ)] = Σ rᵢ · [e(α, β) · e(publicᵢ·IC, γ) · e(Cᵢ, δ)]

    use ark_std::rand::Rng;

    let mut rng = ark_std::test_rng();
    let mut left_pairs = Vec::new();
    let mut right_pairs = Vec::new();

    for ((proof, vk), pub_input) in proofs.iter().zip(vks.iter()).zip(public_inputs.iter()) {
        // Random scalar for this proof
        let r = Scalar::rand(&mut rng);

        // Left: r · e(A, B) = e(r·A, B)
        left_pairs.push((proof.a.mul(r), proof.b));

        // Right: r · [e(α, β) · e(public·IC, γ) · e(C, δ)]
        let mut public_ic = vk.ic[0].clone();
        for (j, input) in pub_input.iter().enumerate() {
            public_ic += vk.ic[j + 1].mul(*input);
        }

        right_pairs.extend(&[
            (vk.alpha_g1.mul(r), vk.beta_g2),
            (public_ic.mul(r), vk.gamma_g2),
            (proof.c.mul(r), vk.delta_g2),
        ]);
    }

    let left = product_of_pairings(&left_pairs);
    let right = product_of_pairings(&right_pairs);

    left == right
}
```

## Worked Example: Pairing Equation

Let's verify a simple Groth16 proof with pairing operations.

**Setup Parameters:**
```
α = 5, β = 7, γ = 11, δ = 13
α·G₁ = [138, 292]  (point on curve)
β·G₂ = [[54 + 97i], [12 + 3i]]  (point on G₂)
```

**Proof:**
```
A = [101, 203]
B = [[44 + 23i], [88 + 12i]]
C = [67, 145]
```

**Public Input:**
```
public = 15
IC = [[5, 10], [8, 14], ...]  // verification key
```

**Step 1: Compute left side**
```
left = e(A, B) = e([101, 203], [[44+23i], [88+12i]])
      = some element in GT (finite field element)
```

**Step 2: Compute right side**
```
public·IC = 15 · IC[1] + IC[0]
          = 15 · [8, 14] + [5, 10]
          = [125, 220]

right = e(α, β) · e(public·IC, γ) · e(C, δ)
      = e([138, 292], [[54+97i], [12+3i]]) ·
        e([125, 220], [99, 45]) ·
        e([67, 145], [76, 199])
```

**Step 3: Compare**
```
left == right?  →  Yes!
Proof is valid.
```

## Security Considerations

### Small-Subgroup Attacks

**Attack:** Attacker provides point P in small subgroup (order r' < r).

**Defense:** Always check P · r = 𝒪 (point at infinity).

```rust
fn validate_point(p: &G1) -> bool {
    let r = <Pairing as PairingEngine>::Fr::MODULUS;
    p.mul(r) == G1::zero()  // Must be identity
}
```

### Invalid Curve Attacks

**Attack:** Attacker provides point not on the curve.

**Defense:** Check point satisfies curve equation y² = x³ + ax + b.

```rust
fn is_on_curve(p: &G1Affine) -> bool {
    let x = p.x;
    let y = p.y;
    y.square() == x.cube() + 3  // For BN254: y² = x³ + 3
}
```

### Pairing Fuselage Attacks

**Attack:** Attacker crafts A, B such that e(A, B) = 1 but A ≠ 𝒪, B ≠ 𝒪.

**Defense:** Use validated generators and proper subgroup checks.

## Summary

**Key Takeaways:**
1. Elliptic curves provide groups with discrete log hardness
2. Pairings enable cross-group comparisons: e(g₁^a, g₂^b) = e(g₁, g₂)^(ab)
3. This property lets us check polynomial equations "in the exponent"
4. Pairing-friendly curves (BN254, BLS12-381) enable efficient pairings
5. Security requires subgroup and validity checks

**Next Chapter:** With QAPs and pairings, we can now perform the trusted setup ceremony - generating the proving and verification keys.

## Further Reading

- [Pairings for Beginners](https://www.cryptologie.net/article/311/pairings-for-beginners/) - Intuitive explanation
- [BN254 Curve Specification](https://eprint.iacr.org/2005/133) - Original curve paper
- [BLS12-381 Standard](https://www.ietf.org/archive/id/draft-irtf-cfrg-pairing-friendly-curves-04.html) - Modern standard
- [arkworks Documentation](https://docs.rs/ark-ec/) - Implementation reference
