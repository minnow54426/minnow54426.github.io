# Identity Circuit: Hash Preimage Proofs

## Problem Statement

Hash preimage proofs solve a fundamental problem in cryptographic systems: **how to prove you know a secret value without revealing it**.

This is useful in scenarios where:
- You need to authenticate without transmitting passwords
- You want to commit to a value now and reveal it later
- You need to prove ownership of a secret identifier

## Circuit Design

### The Statement

**Public Input**: The hash output `H = sha256(x)` (32 bytes)
**Private Witness**: The preimage `x` (32 bytes)
**Proof**: Demonstrates knowledge of `x` such that `sha256(x) = H`

### How SHA-256 Constraints Work in R1CS

The SHA-256 hash function is expressed as a sequence of arithmetic constraints over finite fields. Each round of SHA-256 involves:
- Bitwise operations (AND, XOR, NOT) expressed as field operations
- Modular addition with carries
- Bit rotations and shifts

Each operation becomes one or more R1CS constraints, resulting in approximately **25,000 constraints** for a full SHA-256 hash.

### Constraint Complexity

SHA-256 is relatively expensive in ZK circuits because:
1. **Bit operations**: Field elements don't natively support bitwise ops, requiring auxiliary variables
2. **Round function**: SHA-256 has 64 rounds, each expanding into multiple constraints
3. **Message schedule**: Preprocessing the input message adds additional constraints

**Optimization Consideration**: For production use, SNARK-friendly hashes like Poseidon or MiMC are often preferred (fewer constraints), but SHA-256 is valuable for compatibility with existing systems.

## Public vs Private

| Component | Public | Private |
|-----------|--------|---------|
| Hash output H | ✅ Revealed | ❌ |
| Preimage x | ❌ | ✅ Kept secret |
| Proof | ✅ Revealed (but reveals nothing about x) | ❌ |

**What the verifier learns**: The prover knows some value that hashes to the public hash H.

**What stays secret**: The actual value of x.

## Real-World Applications

### 1. Password Authentication
Traditional password authentication sends the password to the server. With hash preimage proofs:
- Server stores `H = sha256(password)`
- Client proves knowledge of password without revealing it
- No password transmitted over the network

### 2. Commitment Schemes
- **Commit phase**: Prover publishes `H = sha256(secret_value)`
- **Reveal phase**: Prover later reveals `secret_value` by proving it's the preimage of `H`
- **Binding property**: Can't change the value after committing (hash collision resistance)
- **Hiding property**: Value remains secret until reveal

### 3. Digital Identity
- Prove ownership of a secret key without revealing it
- Anonymous credentials: Prove you possess a credential without revealing which one
- Selective disclosure: Reveal only specific attributes

## Example Walkthrough

See `examples/identity_proof.rs` for a complete example:

```rust
fn main() -> Result<()> {
    // 1. Setup: Define the public hash
    let password = "my_secret_password";
    let password_hash = sha256(password.as_bytes());

    // 2. Prover: Generate proof of knowledge
    let circuit = IdentityCircuit::new(password_hash);
    let proof = prove_identity(&circuit, password)?;

    // 3. Verifier: Check proof without learning password
    let is_valid = verify_identity(&circuit, password_hash, &proof)?;

    assert!(is_valid);
    println!("✓ Password verified without transmission!");
}
```

**Step-by-step explanation**:
1. **Setup phase**: Run trusted setup to generate proving key (PK) and verifying key (VK)
2. **Witness generation**: Convert password into field elements for the circuit
3. **Constraint satisfaction**: Circuit enforces `sha256(password) = password_hash`
4. **Proof generation**: Prover uses PK and witness to create a compact proof (~288 bytes)
5. **Verification**: Anyone with VK and public hash can verify the proof in ~2ms

## Limitations and Future Improvements

### Current Limitations
1. **Fixed input size**: 32 bytes (256 bits) - doesn't support variable-length inputs
2. **Hash function**: SHA-256 is expensive (~25K constraints)
3. **No batching**: Each proof proves one preimage

### Potential Improvements
1. **SNARK-friendly hashes**: Switch to Poseidon for ~10x reduction in constraints
2. **Batching**: Prove multiple preimages in a single proof (amortization)
3. **Recursive composition**: Compose with other circuits (e.g., Merkle membership)
4. **Input flexibility**: Support arbitrary-length inputs with padding circuitry

### Security Considerations
- **Trusted setup**: Groth16 requires a one-time setup; must be performed honestly
- **Hash collision resistance**: Security relies on SHA-256 being collision-resistant
- **Preimage resistance**: Attacker shouldn't be able to find x from H alone

## Performance Characteristics

Based on benchmarks (run `cargo bench --bench identity`):
- **Setup time**: ~245ms (one-time cost)
- **Proving time**: ~89ms per proof
- **Verification time**: ~2.1ms per proof
- **Proof size**: 288 bytes
- **Constraints**: ~25,000

## References

- [SHA-2 Wikipedia](https://en.wikipedia.org/wiki/SHA-2)
- [Hash Preimage Proofs in ZK-SNARKs](https://zkproof.org/2022/03/31/annotated-zkprivacy-crypto/)
- [ark-crypto-primitives documentation](https://docs.rs/ark-crypto-primitives/)
