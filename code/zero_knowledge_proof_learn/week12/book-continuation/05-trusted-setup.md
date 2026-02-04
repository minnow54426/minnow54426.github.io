# Chapter 5: Trusted Setup Ceremony

## Introduction

Chapters 1-4 built the mathematical and cryptographic foundations for zk-SNARKs. We have QAPs for polynomial verification and pairings for zero-knowledge. But before we can generate or verify proofs, we need one more thing: the proving and verification keys.

The **trusted setup** is a one-time ceremony that generates these keys. It's called "trusted" because it requires a source of randomness (called "toxic waste") that must be destroyed after the ceremony. If this randomness is leaked, an attacker can forge fake proofs.

This chapter explains how the trusted setup works, why it's necessary, and how modern implementations mitigate the trust requirement.

**Chapter Goals:**
- Understand the Powers of Tau computation
- Learn how proving and verification keys are structured
- See why the "toxic waste" must be kept secret
- Understand multi-party ceremonies as a trust mitigation
- Implement trusted setup in Rust using arkworks

## The Setup Problem

### Why We Need Setup

Recall from Chapter 4 that Groth16 verification checks:

```
e(A, B) = e(α, β) · e(public·IC, γ) · e(C, δ)
```

The parameters (α, β, γ, δ) and the input commitment (IC) must be generated somehow. They have special properties:
- **α, β, γ, δ**: Random field elements (the "toxic waste")
- **IC**: Computed from the QAP coefficients

These parameters are **circuit-specific** - different circuits need different parameters.

### The Trust Requirement

The security of Groth16 depends on α and β being uniformly random and **completely secret**.

**If α is leaked:**
```
Attacker can compute α·G₁ and create fake proofs
Soundness is broken - anyone can forge proofs
```

**If β is leaked:**
```
Similar attack via β·G₂
Soundness is broken
```

This is why the setup ceremony must be "trusted" - the participants must:
1. Generate high-quality randomness
2. Keep the randomness secret
3. Destroy it after the ceremony

## Powers of Tau

### The Core Computation

The trusted setup computes the "powers of tau" - sequential exponentiations of a random point:

```
τ¹, τ², τ³, ..., τⁿ
```

Where:
- **τ** is the random field element (the "toxic waste")
- **n** is the maximum degree of the QAP polynomials
- The computation happens in both G₁ and G₂

**Powers in G₁:**
```
G₁^τ, G₁^(τ²), G₁^(τ³), ..., G₁^(τⁿ)
```

**Powers in G₂:**
```
G₂^τ, G₂^(τ²), G₂^(τ³), ..., G₂^(τⁿ)
```

Additionally, we compute mixed powers for pairing verification.

### Why Powers of Tau?

The powers of tau correspond to the polynomial coefficients in the QAP. Recall that our QAP polynomials are:
- A(x) = Σ aᵢ · Aᵢ(x)
- B(x) = Σ bᵢ · Bᵢ(x)
- C(x) = Σ cᵢ · Cᵢ(x)

The powers of tau let us evaluate these polynomials "in the exponent":
```
G₁^A(τ) = G₁^(a₀·A₀(τ) + a₁·A₁(τ) + ...)
        = (G₁^A₀(τ))^a₀ · (G₁^A₁(τ))^a₁ · ...
```

This is the key to generating the proving key.

### Alpha, Beta, Gamma, Delta

These four field elements are the random parameters of the trusted setup:

- **α, β**: Used in proof verification equation (the main toxic waste)
- **γ, δ**: Used for input commitment and proof blinding

In some Groth16 descriptions, γ and δ are derived from α and β (γ = α·β, δ = α·β²). However, our implementation generates all four independently for defense-in-depth: compromising α and β doesn't automatically compromise γ and δ.

## Key Structure

### Proving Key (PK)

The proving key contains everything the prover needs:
- **α, β, γ, δ** in G₁/G₂ (encrypted forms)
- **Powers of tau** in G₁ and G₂
- **β·H** for division polynomial (computed from QAP target polynomial)

```rust
pub struct ProvingKey {
    // Alpha and beta in G1
    pub alpha_g1: G1,
    pub beta_g1: G1,

    // Beta in G2
    pub beta_g2: G2,

    // Gamma and delta in G2
    pub gamma_g2: G2,
    pub delta_g2: G2,

    // Powers of tau
    pub powers_of_tau_g1: Vec<G1>,  // [τ⁰, τ¹, τ², ..., τⁿ]
    pub powers_of_tau_g2: Vec<G2>,  // [τ⁰, τ¹, τ², ..., τⁿ]

    // Division polynomial
    pub beta_h: G1,  // β · H(tau) where H is target polynomial
}
```

### Verification Key (VK)

The verification key is the public part:
- **α, β, γ, δ** in G₁/G₂ (same as PK but public)
- **IC** (input commitment): List of G₁ points for public inputs

```rust
pub struct VerifyingKey {
    // Alpha in G1
    pub alpha_g1: G1,

    // Beta in G2
    pub beta_g2: G2,

    // Gamma and delta in G2
    pub gamma_g2: G2,
    pub delta_g2: G2,

    // Input commitment
    pub ic: Vec<G1>,  // [IC₀, IC₁, IC₂, ..., ICₘ]
}
```

**Why IC is public:**
The prover computes:
```
public_commitment = Σ public_input[i] · ICᵢ
```

And the verifier checks:
```
e(A, B) = e(α, β) · e(public_commitment, γ) · e(C, δ)
```

This allows the verifier to incorporate public inputs into the pairing check.

### Computing IC from QAP

The input commitment (IC) is computed from the QAP's A-polynomials at τ:

```
IC₀ = G₁^A₀(τ)  // Constant term (usually G₁)
IC₁ = G₁^A₁(τ)  // First public input
IC₂ = G₁^A₂(τ)  // Second public input
...
ICₘ = G₁^Aₘ(τ)  // Last public input
```

Where Aᵢ(x) are the QAP A-polynomials for each variable, evaluated at τ.

**Implementation Note:** The actual implementation in `../week11/crates/groth16/src/keys.rs` stores encrypted polynomial evaluations (`a_query`, `b_g1_query`, `b_g2_query`, `c_query`, `h_query`) rather than raw powers of tau. These are computed by evaluating QAP polynomials at tau and encrypting with alpha/beta. The conceptual structures above show what these values represent mathematically.

## Single-Party Setup

### The Algorithm

**Input:** QAP with max degree n
**Output:** Proving key PK and verification key VK

**Step 1: Generate toxic waste**
```
τ ← random field element
α ← random field element
β ← random field element
```

**Step 2: Compute powers of tau**
```rust
let mut powers_of_tau_g1 = Vec::with_capacity(n + 1);
let mut powers_of_tau_g2 = Vec::with_capacity(n + 1);

let g1 = G1::generator();  // Generator of G1
let g2 = G2::generator();  // Generator of G2

let mut current_g1 = g1;
let mut current_g2 = g2;

for _ in 0..=n {
    powers_of_tau_g1.push(current_g1);
    powers_of_tau_g2.push(current_g2);
    current_g1 = current_g1.mul(tau);
    current_g2 = current_g2.mul(tau);
}
```

**Step 3: Compute beta · H**
```rust
// H is the target polynomial: t(x) = Π (x - i) for i=1 to n
// Evaluate H at τ and multiply by β

let h_tau = qap.target.evaluate(Scalar::from(n as u64));
let beta_h = g1.mul(beta).mul(h_tau);
```

**Step 4: Compute IC**
```rust
let mut ic = Vec::with_capacity(num_public_inputs + 1);

// IC[0] = β·G₁ (for constant 1 in witness)
ic.push(g1.mul(beta));

// IC[i] = β·Aᵢ(τ)·G₁ for each public input
for a_poly in a_polys.iter().take(num_public_inputs + 1).skip(1) {
    let a_j_tau = evaluate_polynomial(a_poly, tau);
    ic.push(g1.mul(beta * a_j_tau));
}
```

**Step 5: Construct keys**
```rust
let pk = ProvingKey {
    alpha_g1: g1.mul(alpha),
    beta_g1: g1.mul(beta),
    beta_g2: g2.mul(beta),
    gamma_g2: g2.mul(alpha * beta),
    delta_g2: g2.mul(alpha * beta * beta),
    powers_of_tau_g1,
    powers_of_tau_g2,
    beta_h,
};

let vk = VerifyingKey {
    alpha_g1: pk.alpha_g1,
    beta_g2: pk.beta_g2,
    gamma_g2: pk.gamma_g2,
    delta_g2: pk.delta_g2,
    ic,
};
```

### Implementation Note

The code above is simplified for clarity. The actual implementation in `../week11/crates/groth16/src/setup.rs` uses proper arkworks 0.4 APIs and handles edge cases.

## Multi-Party Ceremonies

### The Trust Problem

Single-party setup requires **complete trust** in the participant:
- They must not leak τ, α, β
- They must use high-quality randomness
- They must properly destroy the toxic waste

This is a **single point of failure**.

### Multi-Party Setup Solution

Multi-party ceremonies distribute the trust across N participants. The key insight: if **ANY ONE participant is honest** (doesn't leak their randomness), the setup is secure.

### The MPC Protocol

**Participants:** N parties
**Goal:** Compute powers of τ without any party knowing τ

**Round 1:**
1. Each party i generates their own τᵢ
2. Each party computes: [τᵢ⁰, τᵢ¹, ..., τᵢⁿ] and broadcasts
3. Everyone combines: τʲ = Π τᵢʲ (multiplication in the exponent)
4. Result: Powers of the product τ = τ₁ · τ₂ · ... · τₙ

**Why This Works:**
- No single party knows τ (the product)
- To leak τ, ALL parties must collude
- If ANY ONE party destroys their τᵢ, τ is unrecoverable

**Security:** N-of-N trust → Any 1 honest participant secures the setup

### Aggregating Contributions

**Power aggregation formula:**
```
G₁^τʲ = G₁^(τ₁·τ₂·...·τₙ)ʲ = Πᵢ G₁^(τᵢ·ʲ)
```

Each participant computes their contribution:
```
Party i: [G₁^(τᵢ·¹), G₁^(τᵢ·²), ..., G₁^(τᵢ·ⁿ)]
```

Everyone multiplies:
```
G₁^τ¹ = Πᵢ G₁^(τᵢ·¹)
G₁^τ² = Πᵢ G₁^(τᵢ·²)
...
```

The final powers are the product of all contributions.

### Real-World MPC Ceremonies

**Zcash Powers of Tau Ceremony (2016):**
- 2,900+ participants
- Generated parameters for Zcash
- Took several months
- Open source code
- Anyone could participate

**Ethereum KZG Ceremony (2022):**
- Thousands of participants
- Generated parameters for proto-danksharding
- Each contribution was verified on-chain
- Final parameters are trustless

**Perpetual Powers of Tau (PPOT):**
- Ongoing ceremony
- Results can be used for any circuit up to degree 2²²
- Anyone can contribute
- ceremony.ethereum.org

## Toxic Waste Management

### What Makes It Toxic?

The "toxic waste" (τ, α, β) is called toxic because:
1. **Secretive:** Must never be revealed
2. **Powerful:** Knowing it breaks soundness
3. **Unrecoverable:** Once destroyed, can't be recovered

### Secure Destruction

**Best Practices:**
1. **Generate in air-gapped environment** - No network, no disk persistence
2. **Use hardware RNG** - High-entropy randomness
3. **Never store to disk** - Keep only in RAM
4. **Overwrite RAM** - Zero memory after use
5. **Multiple witnesses** - Have others observe destruction
6. **Documentation** - Publish destruction process

**Example Process:**
```bash
# Generate in secure environment
cd /tmp/ramdisk
python3 generate_toxic_waste.py

# Use immediately for setup
python3 trusted_setup.py --toxic-waste toxic.bin

# Securely delete
shred -vfu toxic.bin
rm -f toxic.bin
sync
```

### Verification of Destruction

How can we trust the toxic waste was destroyed?

**Single-party:** We can't verify - must trust the participant

**Multi-party:** Each participant proves they destroyed their τᵢ by:
- Publishing a hash commitment beforehand
- Showing the contribution during the ceremony
- Destroying τᵢ after contributing
- Publishing a signed statement of destruction

## Implementation

Our trusted setup implementation is in `../week11/crates/groth16/src/setup.rs`.

**Note:** The following code illustrates the concepts. The actual implementation uses arkworks 0.4 APIs which encapsulate much of the complexity.

### Data Structures

```rust
use ark_ec::ProjectiveCurve;
use ark_ff::Field;

/// Proving key - kept private by prover
pub struct ProvingKey {
    pub alpha_g1: G1,
    pub beta_g1: G1,
    pub beta_g2: G2,
    pub gamma_g2: G2,
    pub delta_g2: G2,
    pub powers_of_tau: Vec<G1>,
    pub beta_h: G1,
}

/// Verification key - public
pub struct VerifyingKey {
    pub alpha_g1: G1,
    pub beta_g2: G2,
    pub gamma_g2: G2,
    pub delta_g2: G2,
    pub ic: Vec<G1>,
}
```

### Setup Function

```rust
pub fn trusted_setup(qap: &QAP, num_public_inputs: usize) -> Result<(ProvingKey, VerifyingKey), SetupError> {
    // In production: use secure RNG and proper key destruction
    let mut rng = rand::thread_rng();

    // Generate toxic waste (WARNING: must be destroyed!)
    let tau = Fr::rand(&mut rng);
    let alpha = Fr::rand(&mut rng);
    let beta = Fr::rand(&mut rng);

    // Get generators
    let g1 = G1::generator();
    let g2 = G2::generator();

    // Compute powers of tau
    let n = qap.target.degree();
    let mut powers_of_tau = Vec::with_capacity(n + 1);

    let current = g1.clone();
    for _ in 0..=n {
        powers_of_tau.push(current);
        current = current.mul(tau);
    }

    // Compute beta * H(tau)
    let h_tau = qap.target.evaluate(tau);
    let beta_h = g1.mul(beta).mul(h_tau);

    // Compute IC from QAP A-polynomials
    let mut ic = Vec::with_capacity(num_public_inputs + 1);
    for j in 0..=num_public_inputs {
        let a_j_tau = evaluate_polynomial(&qap.a_polys[j], tau);
        ic.push(g1.mul(a_j_tau));
    }

    // Construct keys
    let pk = ProvingKey {
        alpha_g1: g1.mul(alpha),
        beta_g1: g1.mul(beta),
        beta_g2: g2.mul(beta),
        gamma_g2: g2.mul(alpha * beta),
        delta_g2: g2.mul(alpha * beta * beta),
        powers_of_tau,
        beta_h,
    };

    let vk = VerifyingKey {
        alpha_g1: pk.alpha_g1,
        beta_g2: pk.beta_g2,
        gamma_g2: pk.gamma_g2,
        delta_g2: pk.delta_g2,
        ic,
    };

    // WARNING: Securely destroy toxic waste here!
    // (In production, use zeroization and secure memory)

    Ok((pk, vk))
}

fn evaluate_polynomial(poly: &Polynomial, point: Scalar) -> Scalar {
    let mut result = Scalar::ZERO;
    let mut x = Scalar::ONE;

    for coeff in &poly.coeffs {
        result += *coeff * x;
        x *= point;
    }

    result
}
```

### Key Serialization

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableProvingKey {
    pub alpha_g1: [u8; 48],
    pub beta_g1: [u8; 48],
    pub beta_g2: [u8; 96],
    pub gamma_g2: [u8; 96],
    pub delta_g2: [u8; 96],
    pub powers_of_tau: Vec<[u8; 48]>,
    pub beta_h: [u8; 48],
}

pub fn serialize_pk(pk: &ProvingKey) -> Vec<u8> {
    let spk = SerializableProvingKey::from(pk);
    bincode::serialize(&spk).unwrap()
}

pub fn deserialize_pk(bytes: &[u8]) -> ProvingKey {
    let spk: SerializableProvingKey = bincode::deserialize(bytes).unwrap();
    ProvingKey::from(spk)
}
```

## Security Analysis

### Setup Security Requirements

**For Soundness (no fake proofs):**
1. τ must be uniformly random
2. τ must be kept secret
3. γ and δ must be correctly derived from τ, α, β

**For Zero-Knowledge (no witness leakage):**
1. α, β must be uniformly random
2. Blinding factors in proof generation must be random

### Attack Scenarios

**Attack 1: Compromised τ**

If attacker learns τ:
```
1. Compute G₁^τ, G₁^(τ²), ...
2. Create fake proofs that verify
3. Soundness completely broken
```

**Defense:**
- Use MPC ceremony (any 1 honest participant secures τ)
- Generate τ in air-gapped environment
- Destroy τ immediately after use

**Attack 2: Weak Randomness**

If τ has low entropy:
```
1. Attacker can brute force search
2. Once τ is found, same as Attack 1
```

**Defense:**
- Use hardware RNG (not pseudo-RNG)
- Combine multiple entropy sources
- Test randomness quality (NIST SP 800-90B)

**Attack 3: Imperfect Destruction**

If τ is not properly destroyed:
```
1. Attacker recovers τ from RAM or disk
2. Soundness broken
```

**Defense:**
- Zero memory after key generation
- Use encrypted RAM
- Verify with memory dumps
- Use TPM for secure key generation

### Trust Assumptions

**Single-party setup:**
- Trust that the single participant is honest
- Trust that their RNG is good
- Trust that they properly destroyed τ

**N-party MPC setup:**
- Trust that at least 1 participant is honest
- Trust that honest participants properly destroyed their τᵢ
- No trust required in other participants

**Perpetual Powers of Tau:**
- Trust that someone honest contributed
- Public verifiable contributions
- Trust can be independently verified

## Summary

**Key Takeaways:**
1. Trusted setup generates proving and verification keys
2. Powers of tau compute sequential exponentiations of a random point
3. Proving key contains secret parameters (α, β, γ, δ)
4. Verification key contains public parameters and input commitment
5. "Toxic waste" (τ, α, β) must be destroyed after setup
6. Multi-party ceremonies distribute trust (any 1 honest participant secures setup)
7. MPC ceremonies are used in production (Zcash, Ethereum)

**Next Chapter:** With proving and verification keys, we can now generate zero-knowledge proofs.

## Further Reading

- [The Perpetual Powers of Tau Ceremony](https://ceremony.ethereum.org/) - Ongoing MPC ceremony
- [Zcash Ceremony](https://z.cash/blog/the-powers-of-tau-ceremony/) - Zcash's setup ceremony
- [MPC in Practice](https://blog.ethereum.org/2016/12/05/trusted-setup-ceremonies/) - Ethereum's approach
- [ark-groth16 Setup](https://docs.rs/ark-groth16/latest/ark_groth16/struct.ProvingKey.html) - API documentation
