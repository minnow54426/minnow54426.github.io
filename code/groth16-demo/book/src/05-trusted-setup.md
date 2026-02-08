# Trusted Setup

We have our QAP polynomials and understand pairings. Now we need to generate the cryptographic keys that will be used for proving and verifying. This process is called the **trusted setup**.

## Learning Objectives

After this chapter, you will understand:
- What trusted setup is and why it's necessary
- The "toxic waste" problem and how to mitigate it
- How proving and verification keys are generated
- The structure of Groth16 keys
- Powers of Tau and structured reference strings

## Motivating Example: Why We Need Setup

Recall from Chapter 4 that we want to check:
```text
P(x) = H(x) · T(x)
```

Using pairings, we can check:
```text
e(P₁, G₂) = e(H₁, T₁)
```

**Problem**: We need to encrypt the QAP polynomials!
- We need A(τ)·G₁, B(τ)·G₂, C(τ)·G₁ for each polynomial
- We need random values α, β, γ, δ to ensure soundness
- We need a random τ to evaluate polynomials at

**Solution**: Run a trusted setup ceremony to generate these values!

## Theory Deep Dive: What is Trusted Setup?

### The Setup Ceremony

Trusted setup generates random secrets that are used to encrypt the QAP:

**Secrets generated**:
- **α** (alpha): Used for proof A component
- **β** (beta): Used for proof B and C components
- **γ** (gamma): Used for public input encryption
- **δ** (delta): Used for proof C component
- **τ** (tau): Random point to evaluate polynomials at

**Key properties**:
1. These secrets are random and unpredictable
2. They MUST be deleted after the ceremony
3. If leaked, anyone can forge proofs!

### Toxic Waste

The secrets (α, β, γ, δ, τ) are called **"toxic waste"** because:

1. **Dangerous**: If an attacker obtains them, they can forge proofs
2. **Must be destroyed**: They should be securely deleted after the ceremony
3. **Unverifiable**: We can't prove they were deleted (trust required!)

### Powers of Tau

The **Powers of Tau** is a structured reference string (SRS):
```text
[τ⁰·G₁, τ¹·G₁, τ²·G₁, ..., τⁿ·G₁]
[τ⁰·G₂, τ¹·G₂, τ²·G₂, ..., τⁿ·G₂]
```

These powers of τ allow us to evaluate any polynomial at τ:
```text
P(x) = a₀ + a₁x + a₂x²
P(τ)·G₁ = a₀·τ⁰·G₁ + a₁·τ¹·G₁ + a₂·τ²·G₁
```

**Key insight**: We can compute P(τ)·G₁ without knowing τ!

## Theory Deep Dive: Key Structure

### Proving Key (PK)

The proving key contains encrypted elements needed to generate proofs:

From `crates/groth16/src/keys.rs:51-86`:

```rust,ignore
/// Proving key for Groth16
#[derive(Clone, Debug)]
pub struct ProvingKey {
    /// α·G₁ (used in proof A component)
    pub alpha_g1: G1Affine,

    /// β·G₁ (used in proof A component)
    pub beta_g1: G1Affine,

    /// β·G₂ (used in verification)
    pub beta_g2: G2Affine,

    /// δ·G₁ (used in proof C component)
    pub delta_g1: G1Affine,

    /// δ·G₂ (used in verification)
    pub delta_g2: G2Affine,

    /// Encrypted A-polynomials: [α·Aᵢ(τ)·G₁] for i=0..m
    pub a_query: Vec<G1Affine>,

    /// Encrypted B-polynomials in G1: [β·Bᵢ(τ)·G₁] for i=0..m
    pub b_g1_query: Vec<G1Affine>,

    /// Encrypted B-polynomials in G2: [β·Bᵢ(τ)·G₂] for i=0..m
    pub b_g2_query: Vec<G2Affine>,

    /// Encrypted C-polynomials: [β·Cᵢ(τ)·G₁] for i=0..m
    pub c_query: Vec<G1Affine>,

    /// Encrypted division polynomials: [Hᵢ(τ)·G₁] for i=0..n-2
    pub h_query: Vec<G1Affine>,
}
```

**Key components**:
- **alpha_g1, beta_g1, beta_g2, delta_g1, delta_g2**: Encrypted secrets
- **a_query, b_g1_query, b_g2_query, c_query**: Encrypted QAP polynomials
- **h_query**: Encrypted division polynomials (for H(x))

### Verification Key (VK)

The verification key contains public elements for verification:

From `crates/groth16/src/keys.rs:166-188`:

```rust,ignore
/// Verification key for Groth16
#[derive(Clone, Debug)]
pub struct VerificationKey {
    /// α·G₁ (part of verification equation)
    pub alpha_g1: G1Affine,

    /// β·G₂ (part of verification equation)
    pub beta_g2: G2Affine,

    /// γ·G₂ (base for public input encryption)
    pub gamma_g2: G2Affine,

    /// δ·G₂ (base for proof C encryption)
    pub delta_g2: G2Affine,

    /// Public input encryption: [β·G₁, β·U₁(τ)·G₁ + α·H₁(τ)·G₁, ...]
    pub ic: Vec<G1Affine>,
}
```

**Key components**:
- **alpha_g1, beta_g2, gamma_g2, delta_g2**: Public encrypted secrets
- **ic**: Input consistency vector for verifying public inputs

## Implementation: Trusted Setup in Rust

Now let's see how trusted setup is implemented.

### The Setup Function

From `crates/groth16/src/setup.rs:69-213`:

```rust,ignore
/// Performs the trusted setup ceremony to generate proving and verification keys.
pub fn trusted_setup<R>(
    a_polys: &[Polynomial<ark_bn254::Fq>],
    b_polys: &[Polynomial<ark_bn254::Fq>],
    c_polys: &[Polynomial<ark_bn254::Fq>],
    num_inputs: usize,
    rng: &mut R,
) -> Result<(ProvingKey, VerificationKey), Groth16Error>
where
    R: Rng,
{
    // Step 1: Generate random secrets (TOXIC WASTE)
    let alpha = Fr::rand(rng);
    let beta = Fr::rand(rng);
    let gamma = Fr::rand(rng);
    let delta = Fr::rand(rng);
    let tau = Fr::rand(rng);

    // Step 2: Compute powers of tau encrypted in G1 and G2
    let _tau_powers_g1 = compute_powers_of_tau_g1(tau, num_constraints + 2);
    let _tau_powers_g2 = compute_powers_of_tau_g2(tau, num_constraints + 2);

    // Step 3: Encrypt the secrets with generators
    let alpha_g1 = (G1Affine::generator() * alpha).into_affine();
    let beta_g1 = (G1Affine::generator() * beta).into_affine();
    let beta_g2 = (G2Affine::generator() * beta).into_affine();
    let gamma_g2 = (G2Affine::generator() * gamma).into_affine();
    let delta_g1 = (G1Affine::generator() * delta).into_affine();
    let delta_g2 = (G2Affine::generator() * delta).into_affine();

    // Step 4: Compute encrypted A-polynomials
    let mut a_query = Vec::with_capacity(num_vars);
    for poly in a_polys {
        let eval = poly.evaluate(&tau_field);
        let encrypted = (G1Affine::generator() * alpha * eval).into_affine();
        a_query.push(encrypted);
    }

    // Step 5: Compute encrypted B-polynomials in G1 and G2
    let mut b_g1_query = Vec::with_capacity(num_vars);
    let mut b_g2_query = Vec::with_capacity(num_vars);
    for poly in b_polys {
        let eval = poly.evaluate(&tau_field);
        b_g1_query.push((G1Affine::generator() * beta * eval).into_affine());
        b_g2_query.push((G2Affine::generator() * beta * eval).into_affine());
    }

    // Step 6: Compute encrypted C-polynomials in G1
    let mut c_query = Vec::with_capacity(num_vars);
    for poly in c_polys {
        let eval = poly.evaluate(&tau_field);
        c_query.push((G1Affine::generator() * beta * eval).into_affine());
    }

    // Step 7: Compute division polynomials
    let target = target_polynomial::<ark_bn254::Fq>(num_constraints);
    let h_query = compute_division_polynomials_encrypted(&target, num_constraints, tau_field)?;

    // Step 8: Compute IC for public inputs
    let mut ic = Vec::with_capacity(num_inputs + 1);
    ic.push((G1Affine::generator() * beta).into_affine());  // IC[0] = β·G₁
    for a_poly in a_polys.iter().take(num_inputs + 1).skip(1) {
        let a_eval = a_poly.evaluate(&tau_field).value;
        let ic_point = (G1Affine::generator() * beta * a_eval).into_affine();
        ic.push(ic_point);
    }

    // Construct keys
    let pk = ProvingKey { /* ... */ };
    let vk = VerificationKey { /* ... */ };

    Ok((pk, vk))
}
```

### Powers of Tau Computation

From `crates/groth16/src/setup.rs:231-254`:

```rust,ignore
/// Computes powers of tau encrypted in G1
fn compute_powers_of_tau_g1(tau: Fr, degree: usize) -> Vec<G1Affine> {
    let mut result = Vec::with_capacity(degree);
    let mut current = G1::from(G1Affine::generator());

    for _ in 0..degree {
        result.push(current.into_affine());
        current *= tau;  // Multiply by τ
    }

    result
}
```

This computes: [G₁, τ·G₁, τ²·G₁, ..., τⁿ·G₁]

### Division Polynomials

From `crates/groth16/src/setup.rs:257-288`:

```rust,ignore
/// Computes division polynomials and encrypts them
fn compute_division_polynomials_encrypted(
    target: &Polynomial<ark_bn254::Fq>,
    num_constraints: usize,
    tau: FieldWrapper<ark_bn254::Fq>,
) -> Result<Vec<G1Affine>, Groth16Error> {
    let mut result = Vec::new();

    for j in 0..num_constraints.saturating_sub(2) {
        // Compute t(x) / (x - j)
        let j_field = FieldWrapper::<ark_bn254::Fq>::from(j as u64);
        let divisor = Polynomial::<ark_bn254::Fq>::new(vec![
            FieldWrapper::<ark_bn254::Fq>::zero() - j_field.clone(),
            FieldWrapper::<ark_bn254::Fq>::one(),
        ]);

        let (quotient, _remainder) = divide_polynomials(target, &divisor)?;
        let h_j_at_tau = quotient.evaluate(&tau);
        let encrypted = (G1Affine::generator() * h_j_at_tau).into_affine();
        result.push(encrypted);
    }

    Ok(result)
}
```

## Running the Code

### Example: Multiplier Circuit Setup

```bash
cd /path/to/groth16-demo
cargo test --trusted_setup_test
```

### What Gets Generated

For the multiplier circuit (a × b = c):

**Proving Key**:
```text
alpha_g1: α·G₁
beta_g1: β·G₁
beta_g2: β·G₂
delta_g1: δ·G₁
delta_g2: δ·G₂

a_query: [α·A₀(τ)·G₁, α·A₁(τ)·G₁, α·A₂(τ)·G₁, α·A₃(τ)·G₁]
b_g1_query: [β·B₀(τ)·G₁, β·B₁(τ)·G₁, β·B₂(τ)·G₁, β·B₃(τ)·G₁]
b_g2_query: [β·B₀(τ)·G₂, β·B₁(τ)·G₂, β·B₂(τ)·G₂, β·B₃(τ)·G₂]
c_query: [β·C₀(τ)·G₁, β·C₁(τ)·G₁, β·C₂(τ)·G₁, β·C₃(τ)·G₁]

h_query: []  (no division polynomials for single constraint)
```

**Verification Key**:
```text
alpha_g1: α·G₁
beta_g2: β·G₂
gamma_g2: γ·G₂
delta_g2: δ·G₂

ic: [β·G₁, β·A₁(τ)·G₁]  (for constant 1 and public input c)
```

## Security Considerations

### Toxic Waste Destruction

After the ceremony, the secrets (α, β, γ, δ, τ) **must be destroyed**:

```rust,ignore
// Security: DO NOT do this!
let secrets = (alpha, beta, gamma, delta, tau);
println!("Toxic waste: {:?}", secrets);  // LEAK!

// Correct: Let them go out of scope and be dropped
drop(rng);  // The secrets only existed in the RNG state
```

### Multi-Party Computation (MPC)

To reduce trust, we can use **MPC ceremonies**:

1. **Participant 1** generates secrets₁ and creates SRS₁
2. **Participant 2** generates secrets₂ and updates: SRS₂ = SRS₁ · secrets₂
3. **Participant N** generates secretsₙ and updates: SRSₙ = SRSₙ₋₁ · secretsₙ
4. **Final**: The toxic waste is secrets₁ · secrets₂ · ... · secretsₙ

**Security**: All N participants must collude to recover the toxic waste!

### Per-Circuit Setup

Groth16 requires a **new setup for each circuit**:
- Each circuit has its own QAP polynomials
- The setup evaluates these specific polynomials at τ
- Can't reuse keys across different circuits

**Limitation**: This is expensive! Newer protocols (PLONK, Halo 2) support universal setup.

## Connection to Groth16

Trusted setup is the **preparation step** before proving:

```text
Computation
    ↓
R1CS (matrix constraints)
    ↓
QAP (polynomial divisibility)
    ↓
Elliptic Curve Pairings
    ↓
Trusted Setup ← You are here
    ↓
Proof Generation (Chapter 6)
```

## Exercises

1. **Key structure**:
   ```text
   Why do we need both b_g1_query and b_g2_query?
   Can we use just one?
   ```

2. **Toxic waste**:
   ```text
   If alpha is leaked, what attacks become possible?
   Hint: Look at the proof A component formula
   ```

3. **IC vector**:
   ```text
   Why is IC[0] = β·G₁?
   What does it represent in the witness?
   ```

4. **Challenge question**:
   ```text
   Why can't we use a universal setup for all circuits?
   What part of the setup is circuit-specific?
   ```

## Further Reading

- **Zcash Ceremony**: [The Powers of Tau Ceremony](https://z.cash/blog/snark-explainer.html)
- **Perconia Ceremony**: [Zcash Sapling Ceremony](https://www.zfnd.org/blog/powers-of-tau/)
- **MPC Techniques**: [MPC for ZK-SNARKs](https://eprint.iacr.org/2017/1050)

---

**Ready to generate proofs? Continue to [Chapter 6: Proof Generation](./06-proof-generation.md)**
