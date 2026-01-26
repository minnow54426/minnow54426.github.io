# Membership Circuit: Merkle Tree Proofs

## Problem Statement

Merkle membership proofs solve a critical privacy problem: **how to prove an element belongs to a set without revealing which element**.

This is useful in scenarios where:
- You need to prove you're on an allowlist without revealing your identity
- You want to verify credentials without revealing the actual credential
- You need anonymous access control or voting rights

## Circuit Design

### The Statement

**Public Inputs**:
- Merkle root hash `R` (32 bytes)
- Optional: Leaf index or application-specific public data

**Private Witness**:
- Leaf value `L` (32 bytes)
- Authentication path `P = [sibling_1, sibling_2, ..., sibling_depth]`

**Proof**: Demonstrates that hashing `L` with the path `P` produces the root `R`

### In-Circuit Merkle Path Verification

The circuit verifies the authentication path by recomputing the Merkle root:

1. Start with the leaf `L`
2. For each level `i` from 0 to depth:
   - If the path bit is 0: `hash(L || sibling_i)` → new node
   - If the path bit is 1: `hash(sibling_i || L)` → new node
3. Final result should equal the public root `R`

Each hash operation is performed in-circuit using constraint-friendly hash gadgets.

### Fixed-Depth Rationale

The circuit uses a **fixed tree depth** (8 or 16) for several reasons:

1. **Circuit simplicity**: Static depth means the circuit has a fixed number of constraints
2. **Optimization**: Enables pre-computation and circuit specialization
3. **Performance**: Reduces proving time compared to dynamic-depth circuits
4. **Learning**: Makes the constraint logic clearer and easier to understand

**Trade-off**: Fixed depth means you must commit to a maximum tree size upfront. For production, you might use multiple circuits for different depths or more advanced techniques.

### Constraint Complexity

Total constraints ≈ `depth × constraints_per_hash`

**Hash function choices**:
- **SHA-256**: ~25K constraints per hash, highly secure
- **Poseidon**: ~300-400 constraints per hash, SNARK-optimized
- **Pedersen**: ~2000 constraints per hash, balance of security and efficiency

For a depth-8 tree:
- SHA-256: ~200K constraints
- Poseidon: ~3K constraints

## Public vs Private

| Component | Public | Private |
|-----------|--------|---------|
| Merkle root R | ✅ Revealed | ❌ |
| Leaf value L | ❌ | ✅ Kept secret |
| Authentication path P | ❌ | ✅ Kept secret |
| Leaf index | Optional | ✅ Kept secret |
| Proof | ✅ Revealed | ❌ |

**What the verifier learns**: The prover knows some leaf in the tree that hashes to the root.

**What stays secret**: Which leaf, the leaf's value, and the full path to the root.

## Real-World Applications

### 1. Allowlist Proofs
**Problem**: Verify a user is whitelisted without revealing their address.

**Solution**:
- Build a Merkle tree of all whitelisted addresses
- Publish the root hash on-chain or in a smart contract
- Users prove membership without revealing which address is theirs

**Use cases**: Token sales, VIP access, governance voting

### 2. Anonymous Credentials (Semaphore-like)
**Problem**: Prove you have a valid credential without revealing your identity.

**Solution**:
- Leaf = hash(identity_nullifier, secret)
- Prove membership without revealing which leaf
- Prevent double-voting with nullifiers

**Use cases**: Anonymous voting, airdrops, privacy-preserving authentication

### 3. Privacy-Preserving Voting
**Problem**: Verify eligibility to vote without revealing voter identity.

**Solution**:
- Tree of eligible voter commitments
- Each voter proves membership
- Nullifier prevents double-voting

**Use cases**: DAO governance, private elections, quadratic voting

## Example Walkthrough

See `examples/membership_proof.rs` for a complete example:

```rust
fn main() -> Result<()> {
    // 1. Setup: Build Merkle tree of whitelisted addresses
    let addresses = vec![
        "0x123...", "0x456...", "0x789...", // ... more addresses
    ];
    let tree = MerkleTree::from_leaves(&addresses);

    // 2. Prover: Get your leaf and path from the tree
    let my_address = "0x123...";
    let (leaf, path, index) = tree.get_proof(my_address)?;

    // 3. Generate membership proof
    let circuit = MembershipCircuit::new(tree.root());
    let proof = prove_membership(&circuit, leaf, path, index)?;

    // 4. Verifier: Check membership without learning which address
    let is_valid = verify_membership(&circuit, tree.root(), &proof)?;

    assert!(is_valid);
    println!("✓ Verified whitelist membership anonymously!");
}
```

**Step-by-step explanation**:
1. **Tree construction**: Build Merkle tree off-chain (not in-circuit)
2. **Extract path**: Get the authentication path for your leaf
3. **Setup**: Run trusted setup for the membership circuit
4. **Prove**: Generate proof using leaf + path as witness
5. **Verify**: Anyone with the root can verify the proof

## Security Considerations

### Assumptions the Verifier Must Trust

1. **Honest tree construction**: Verifier assumes the Merkle tree was built correctly
2. **Leaf inclusion**: Verifier assumes your leaf was actually included in the tree
3. **Hash function security**: Collision resistance of the hash function
4. **Trusted setup**: Groth16 setup was performed honestly

### Potential Attacks

1. **Mallory builds fake tree**: If the tree creator is malicious, they can exclude legitimate leaves
2. **Root manipulation**: If the root can be changed, proofs become invalid
3. **Path forgery**: Attempting to forge a path without knowing the real leaf (infeasible if hash is secure)

### Best Practices

1. **Publicly auditable tree construction**: Make tree construction transparent
2. **Root commitment**: Commit the root in an immutable location (e.g., blockchain)
3. **Revocation**: Update root when leaves need to be removed
4. **Nullifiers**: Use nullifiers to prevent double-spending/proofs

## Performance Characteristics

Based on benchmarks (run `cargo bench --bench membership`):

### With Poseidon Hash (depth 8)
- **Setup time**: ~312ms (one-time cost)
- **Proving time**: ~124ms per proof
- **Verification time**: ~2.8ms per proof
- **Proof size**: 288 bytes
- **Constraints**: ~3,000

### With SHA-256 Hash (depth 8)
- **Constraints**: ~200,000
- **Proving time**: ~800ms per proof
- **Verification time**: ~5ms per proof

**Note**: Poseidon is significantly more efficient for in-circuit hashing.

## Hash Function Trade-offs

### SHA-256
**Pros**:
- Widely used and well-understood
- Available in all cryptographic libraries
- Strong security guarantees

**Cons**:
- Expensive in-circuit (~25K constraints)
- Slow proving time

### Poseidon
**Pros**:
- SNARK-optimized (~300 constraints)
- Fast proving time
- No security concerns (well-studied)

**Cons**:
- Less widely adopted outside ZK
- Requires specific implementation
- Different security model than SHA-2

### Recommendation
**Use Poseidon for new ZK applications**. Use SHA-256 only when interoperability with existing systems is required.

## Extensions and Future Work

### 1. Nullifiers
Prove membership AND output a public nullifier to prevent double-spending:
```
nullifier = hash(secret, root)
```
The nullifier is public and unique per secret, allowing the verifier to detect duplicate proofs.

### 2. Batch Membership
Prove multiple leaves belong to the tree in a single proof:
- Aggregate multiple membership proofs
- Amortize setup cost
- Reduce total verification time

### 3. Dynamic Trees
Support trees that change over time:
- Use sparse Merkle trees
- Prove inclusion at a specific block height
- Handle insertion/deletion efficiently

### 4. Recursive Membership
Compose membership proofs:
- Prove membership in multiple trees
- Verify previous membership proofs in-circuit
- Enable complex multi-party protocols

## References

- [Semaphore: Anonymous Ethereum Accounts](https://semaphore.pse.dev/)
- [Merkle Trees in Ethereum](https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/)
- [Poseidon Hash Paper](https://eprint.iacr.org/2019/458)
- [ark-crypto-primitives Merkle Module](https://docs.rs/ark-crypto-primitives/)

## Related Circuits

- **Week 2**: `merkle-rs` - Off-chain Merkle tree implementation
- **Week 9**: Advanced Merkle circuits with nullifiers
- **Week 11**: Capstone membership credential system
