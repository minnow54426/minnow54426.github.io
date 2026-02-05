# Interview Preparation Guide: ZK-SNARKs and Groth16

## Overview
This guide prepares you for technical interviews focused on zero-knowledge proofs, zk-SNARKs, and Groth16 implementation. It covers core concepts, implementation details, and real-world applications.

## Target Roles
- ZK Engineer / Cryptographer
- Blockchain Protocol Engineer
- Smart Contract Developer (ZK-focused)
- Privacy-Preserving Systems Engineer
- Applied Cryptography Researcher

---

## Part 1: Core Concepts (Fundamentals)

### Q1: What is a Zero-Knowledge Proof?

**Key Points to Cover:**
1. **Definition:** Proof that reveals nothing except statement truth
2. **Three Properties:**
   - Completeness: True statements convince honest verifier
   - Soundness: False statements cannot convince honest verifier
   - Zero-knowledge: Verifier learns nothing beyond statement truth
3. **Intuitive Example:** Ali Baba cave, Sudoku, or Where's Waldo
4. **Applications:** Privacy, scalability, authentication

**Sample Answer (2-3 minutes):**
"A zero-knowledge proof is a cryptographic protocol where a prover convinces a verifier that a statement is true without revealing any information beyond the truth of the statement itself. For example, I can prove I know a password without revealing the password itself. The three essential properties are completeness, soundness, and zero-knowledge. ZK proofs are used in privacy-preserving cryptocurrencies like Zcash, scaling solutions like ZK-rollups, and identity systems."

**Follow-up Questions to Prepare For:**
- What's the difference between interactive and non-interactive ZK proofs?
- Can you explain the Ali Baba cave example?
- What's a real-world use case where ZK is essential?

---

### Q2: What is a zk-SNARK?

**Key Points to Cover:**
1. **Acronym:** Zero-Knowledge Succinct Non-Interactive Argument of Knowledge
2. **Succinct:** Proof size is small (constant, e.g., 128 bytes for Groth16)
3. **Non-Interactive:** Single message from prover to verifier (Fiat-Shamir)
4. **Argument:** Computational soundness (not information-theoretic)
5. **Knowledge:** Prover knows witness (extractor can retrieve it)

**Sample Answer:**
"A zk-SNARK is a Zero-Knowledge Succinct Non-Interactive Argument of Knowledge. 'Succinct' means the proof is small and verification is fast - constant size regardless of statement complexity. 'Non-interactive' means the prover sends a single message to the verifier, made possible by the Fiat-Shamir heuristic. It's an 'argument' rather than 'proof' because soundness is computational, not information-theoretic. 'Knowledge' means if you can produce a proof, an extractor can retrieve the witness. Groth16 is a specific zk-SNARK construction."

**Code Example to Mention:**
```rust
// Groth16 proof size: 3 group elements = 3 * 32 bytes = 96 bytes (compressed)
// Verification time: O(1) - constant time
// Prover time: O(n log n) where n = circuit size
```

**Follow-up Questions:**
- What's the difference between zk-SNARK and zk-STARK?
- Why is succinctness important for blockchain?
- What's the Fiat-Shamir heuristic?

---

### Q3: Explain R1CS (Rank-1 Constraint System)

**Key Points to Cover:**
1. **Purpose:** Encode computation as quadratic constraints
2. **Format:** A·w ◦ B·w = C·w (element-wise multiplication)
3. **Components:**
   - A, B, C: Matrices (m×n where m = constraints, n = variables)
   - w: Witness vector (variables + inputs + outputs + 1)
4. **Example:** For a × b = c:
   ```
   w = [a, b, c, 1]
   A = [[1, 0, 0, 0], [0, 0, 0, 0]]
   B = [[0, 1, 0, 0], [0, 0, 0, 0]]
   C = [[0, 0, 1, 0], [0, 0, 0, 1]]
   ```

**Sample Answer:**
"R1CS is a way to represent computations as quadratic constraints. Each constraint has the form A·w times B·w equals C·w, where A, B, C are matrices and w is the witness vector containing all variables. For example, to prove a × b = c, we'd have constraints for the multiplication and for checking the output. R1CS is the first step in compiling a computation to a zk-SNARK - it's then transformed into a QAP."

**Worked Example to Present:**
"I can walk through a specific example. Suppose we want to prove knowledge of a, b such that a × b = c. Our witness is [a, b, c, 1]. We need constraints that enforce a·b = c. One constraint: [1,0,0,0] · w × [0,1,0,0] · w = [0,0,1,0] · w, which expands to a × b = c. The verifier checks c matches the public input."

**Follow-up Questions:**
- How many constraints does a circuit with N multiplications have?
- What's the time complexity of R1CS satisfaction checking?
- How do you add two numbers in R1CS?

---

### Q4: What is a QAP (Quadratic Arithmetic Program)?

**Key Points to Cover:**
1. **Purpose:** Transform R1CS into polynomial form for efficient verification
2. **Key Idea:** Polynomial divisibility test instead of checking all constraints
3. **Process:**
   - Use Lagrange interpolation on each column of A, B, C
   - Get polynomials A_i(x), B_i(x), C_i(x)
   - Target polynomial t(x) = (x-1)(x-2)...(x-m) for m constraints
4. **Verification:** Check if A(x)·B(x) - C(x) is divisible by t(x)
5. **Schwartz-Zippel:** If polynomials agree at random point, equal with high probability

**Sample Answer:**
"QAP transforms R1CS constraints into polynomials using Lagrange interpolation. Each column of the R1CS matrices becomes a polynomial. We then check if A(x)·B(x) - C(x) is divisible by the target polynomial t(x), where t(x) is the product of (x-i) for all constraint indices. If the division is exact, all constraints are satisfied. This reduces verification from O(m) checking all constraints to O(1) checking a single polynomial equation, thanks to the Schwartz-Zippel lemma."

**Mathematical Detail:**
"After interpolation, we have polynomials A_i(x), B_i(x), C_i(x) for each witness element w_i. We evaluate at a random point r, compute the sum, and check divisibility. This is the key to zk-SNARK succinctness - instead of checking m constraints, we verify one polynomial equation."

**Follow-up Questions:**
- Why is Lagrange interpolation used?
- What's the Schwartz-Zippel lemma?
- How does QAP reduce verification complexity from O(m) to O(1)?

---

### Q5: Explain Bilinear Pairings

**Key Points to Cover:**
1. **Definition:** Function e: G₁ × G₂ → Gₜ with bilinearity property
2. **Bilinearity:** e(g₁^a, g₂^b) = e(g₁, g₂)^(a·b) = e(g₁^a, g₂)^b
3. **Purpose:** Enable "encryption in the exponent" for comparing polynomials
4. **Curves:** BN254, BLS12-381 are pairing-friendly
5. **Security:** Based on Decisional Diffie-Hellman in G₁

**Sample Answer:**
"Bilinear pairings are special functions that map two points from different elliptic curve groups to a third group. The key property is bilinearity: e(g^a, h^b) = e(g, h)^(a·b). This allows us to 'multiply in the exponent' - we can compare encrypted values. For example, to check if A·B = C without revealing A, B, C, we check if e(g^A, h^B) equals e(g, h^C). Pairings are the cryptographic engine that makes Groth16 possible."

**Application Example:**
"In Groth16 verification, we check e(A, B) = e(α, β) · e(public·IC, γ) · e(C, δ). Without pairings, we'd have to reveal A and B. With pairings, we can verify the proof in the exponent while keeping A and B secret."

**Follow-up Questions:**
- What's the difference between G₁, G₂, and Gₜ?
- Why can't we use regular elliptic curve operations?
- What's the pairing-friendly curve problem?

---

### Q6: What is Trusted Setup in Groth16?

**Key Points to Cover:**
1. **Purpose:** Generate proving key (PK) and verification key (VK)
2. **Toxic Waste:** Random values τ, α, β that must be destroyed
3. **Ceremony:** Process to generate and destroy toxic waste
4. **Multi-Party (MPC):** Multiple participants, one honest suffices
5. **Per-Circuit:** Each circuit needs its own setup

**Sample Answer:**
"Trusted setup generates the proving and verification keys. The process involves sampling random values τ, α, β and computing powers of these values in the exponent. These values are 'toxic waste' - if an attacker learns them, they can forge arbitrary proofs. The setup must be done in a secure environment, and the toxic waste must be destroyed. In production, multi-party ceremonies are used where each participant contributes randomness. As long as one participant is honest and destroys their share, the setup is secure."

**Security Implications:**
"If τ, α, β are compromised, soundness is broken - attackers can forge proofs. This is why Zcash spent months on their Sapling ceremony with over 90 participants. Our learning implementation uses a single-party setup, which is fine for education but not production."

**Follow-up Questions:**
- Why is it called 'toxic waste'?
- How does MPC make setup safer?
- What happens if you lose the verification key?

---

## Part 2: Implementation (Technical Depth)

### Q7: Walk Through the Groth16 Proving Algorithm

**Step-by-Step Algorithm:**
1. **Input:** Witness w, proving key PK, public inputs pub
2. **Compute H(x):** Polynomial A(x)·B(x) - C(x) divided by t(x)
3. **Evaluate at random point:** A(r), B(r), C(r), H(r)
4. **Sample blinding factors:** Random r, s, δ
5. **Compute proof elements:**
   - A = α·G₁ + A(r)·G₁ + r·δ·G₂
   - B = β·G₂ + B(r)·G₂ + s·δ·G₂
   - C = H(r)·β·G₂ + C(r)·G₂ + s·A(r)·G₂ + r·B(r)·G₂ + r·s·δ·G₂
6. **Output:** Proof (A, B, C)

**Sample Answer:**
"The prover first computes the quotient polynomial H(x) by dividing A·B - C by the target polynomial t(x). Then they evaluate all polynomials at a secret random point r (derived from the trusted setup). They sample random blinding factors to achieve zero-knowledge. Finally, they compute the three proof elements A, B, C as linear combinations of the evaluated polynomials with the blinding factors. The proof is just these three group elements."

**Implementation Detail:**
"In arkworks, we use `Groth16::prove()` which takes the circuit instance, witness, and proving key. It computes the R1CS, transforms to QAP, performs polynomial division, and generates the proof. The blinding factors are sampled using `ark_std::rand::Rng`."

**Follow-up Questions:**
- Why are blinding factors needed?
- What happens if the prover uses the wrong r value?
- How long does proving take for a 100k-constraint circuit?

---

### Q8: Walk Through the Groth16 Verification Algorithm

**Step-by-Step Algorithm:**
1. **Input:** Proof (A, B, C), public inputs pub, verification key VK
2. **Compute input commitment:** pub_IC = Σ pub[i]·IC[i]
3. **Check pairing equation:**
   - e(A, B) = e(α, β) · e(pub_IC, γ) · e(C, δ)
4. **Output:** Accept if equation holds, reject otherwise

**Sample Answer:**
"Verification is straightforward: check a single pairing equation. We compute the commitment to the public inputs using the input commitment matrix IC from the verification key. Then we verify e(A, B) equals e(α, β) times e(pub_IC, γ) times e(C, δ). This single check verifies the proof is valid. The pairing equation ensures A·B = α·β + pub·IC + C·δ, which encodes the constraint satisfaction."

**Implementation Code:**
```rust
let input_commits: Vec<F> = public_inputs.iter()
    .zip(vk.ic.iter())
    .map(|(pub, ic)| ic * pub)
    .sum();

let result = E::multi_pairing(
    &[A, input_commits, C],
    &[B, vk.gamma_g2, vk.delta_g2]
);

let expected = E::pairing(vk.alpha_g1, vk.beta_g2);

result == expected
```

**Follow-up Questions:**
- Why is verification O(1)?
- What's the role of IC in verification?
- How does batch verification work?

---

### Q9: How Do You Design an Efficient Circuit?

**Design Principles:**
1. **Minimize Constraints:** Fewer constraints = faster proving
2. **Avoid Multiplications:** Use additions where possible
3. **Reuse Computation:** Cache intermediate results
4. **Choose Efficient Primitives:** Poseidon > SHA-256 for circuits
5. **Optimize Bit Width:** Use minimum required bits

**Example: Range Proof for Age ≥ 18**
**Naive Approach:**
- Direct comparison: ~100 constraints

**Optimized Approach:**
- Bit decomposition of age: 8 constraints for bits
- Binary comparison: ~50 constraints
- Total: ~58 constraints (42% reduction)

**Sample Answer:**
"Circuit optimization is both art and science. I start with the clearest implementation, then optimize. Key strategies: avoid unnecessary multiplications (they're expensive), use efficient hash functions like Poseidon instead of SHA-256, minimize bit widths, and reuse computations. For example, in a range proof, bit decomposition lets us use binary arithmetic instead of decimal, reducing constraints by 40-50%. I always benchmark - measure constraint count, prover time, and verifier time."

**Real-World Example:**
"In Zcash, the Sprout circuit had 2 million constraints. By switching to Sapling with optimizations like Poseidon hash and better circuit design, they reduced it to 70,000 constraints - a 96% reduction. This shows how important circuit optimization is."

**Follow-up Questions:**
- How do you measure circuit efficiency?
- What's the constraint cost of a SHA-256 hash?
- When would you choose SHA-256 over Poseidon?

---

### Q10: How Do You Handle Public vs Private Inputs?

**Design Considerations:**
1. **Public:** Known to verifier, affects proof verification
2. **Private:** Witness elements, kept secret
3. **Privacy Leakage:** Public outputs can reveal private inputs
4. **Example:** If a × b = c and c is public, attacker knows possible (a,b) pairs

**Sample Answer:**
"Public inputs are part of the statement being proven - the verifier knows them. Private inputs are witness elements only the prover knows. In arkworks, public inputs are passed separately from the witness. The key design challenge is avoiding privacy leakage. For example, if I prove a × b = 15 with public output 15, the verifier learns that (a,b) could be (1,15), (3,5), or (5,3). To avoid this, I might add random blinding or use range proofs."

**Implementation Pattern:**
```rust
struct MultiplierCircuit<F: Field> {
    // Private (witness)
    pub a: Option<F>,
    pub b: Option<F>,

    // Public (statement)
    pub c: Option<F>,  // Product
}
```

**Follow-up Questions:**
- What's a scenario where public inputs leak privacy?
- How do you prevent privacy leakage?
- Can all inputs be private?

---

## Part 3: Real-World Applications

### Q11: How Are ZK-Rollups Used for Ethereum Scaling?

**Key Points:**
1. **Problem:** Ethereum processes ~15 TPS, high gas fees
2. **Solution:** Batch transactions off-chain, single proof on-chain
3. **Throughput:** 100-2000 TPS (100× improvement)
4. **Gas Cost:** ~300k gas for 100 transactions vs 3M for direct
5. **Projects:** zkSync Era, Polygon zkEVM, Scroll, StarkNet

**Sample Answer:**
"ZK-rollups scale Ethereum by executing transactions off-chain and posting a single proof on-chain. The operator collects hundreds of transactions, executes them, computes the new state root, and generates a ZK proof that the execution was correct. The smart contract verifies the proof in ~300k gas instead of executing all transactions directly. This increases throughput from 15 TPS to 100-2000 TPS while inheriting Ethereum's security. Projects like zkSync and Polygon zkEVM are live in production."

**Technical Detail:**
"The rollup circuit proves: transaction validity (signatures, nonces), state transitions, and no double-spending. Constraint count is 100k-500k per batch, requiring 1-5 minutes to prove. Verification is O(1) in ~10ms. The proof size is ~200 bytes, much smaller than calldata for all transactions."

**Follow-up Questions:**
- What's data availability in ZK-rollups?
- How does ZK-rollup compare to Optimistic rollup?
- What happens if the operator disappears?

---

### Q12: How Does Zcash Use Groth16 for Privacy?

**Key Points:**
1. **Shielded Transactions:** Prove spend without revealing amount/sender/receiver
2. **Circuits:** Sprout (2M constraints), Sapling (70K), Orchard (50K)
3. **Components:** Merkle membership, commitment scheme, nullifiers
4. **Trusted Setup:** Per-circuit, MPC ceremonies (90+ participants)
5. **Privacy:** Sender, receiver, amount all hidden

**Sample Answer:**
"Zcash uses Groth16 for shielded transactions. When you send ZEC shielded, you create a commitment to your note and add it to a Merkle tree. To spend, you prove membership in the tree using a ZK proof. The proof shows you have a valid note with sufficient balance without revealing which note or how much you're sending. Zcash has evolved through Sprout, Sapling, and Orchard circuits, reducing constraints from 2M to 50K through optimization."

**Technical Flow:**
"Shielded spend: Prover creates note commitment C = H(address, amount, rho, r), adds to Merkle tree. To spend: prove Merkle membership, prove nullifier not seen before, prove balance. The proof doesn't reveal which leaf or amounts. Verifier checks nullifier uniqueness and Merkle proof validity."

**Follow-up Questions:**
- What's a nullifier in Zcash?
- Why did Zcash need multiple circuit updates?
- How does Zcash prevent double-spending?

---

### Q13: What Are the Limitations of Groth16?

**Limitations:**
1. **Per-Circuit Trusted Setup:** Each new circuit needs new ceremony
2. **Prover Time:** Slow for large circuits (minutes to hours)
3. **Memory Usage:** 16-64GB RAM for large circuits
4. **Quantum Vulnerability:** Broken by Shor's algorithm
5. **Circuit Complexity:** Designing circuits is difficult

**Comparison with Alternatives:**
- **PLONK:** Universal setup (no per-circuit ceremony), larger proofs
- **STARKs:** No trusted setup, larger proofs, slower verification
- **Bulletproofs:** No trusted setup, larger proofs, slower verification
- **Groth16:** Smallest proofs, fastest verification, but requires per-circuit setup

**Sample Answer:**
"Groth16's main limitation is the per-circuit trusted setup - every new circuit needs a new ceremony, which is operationally expensive. Prover time is also a bottleneck - large circuits take minutes to hours. Memory requirements are high (16-64GB). And it's quantum-vulnerable. Alternatives like PLONK use universal setup (one ceremony for all circuits), and STARKs have no trusted setup at all, but both have larger proofs and slower verification. The choice depends on the application."

**Follow-up Questions:**
- When would you choose PLONK over Groth16?
- Are Groth16 proofs quantum-safe?
- What's the largest Groth16 circuit you've seen?

---

## Part 4: Your Experience (Portfolio Questions)

### Q14: Walk Me Through Your ZK Learning Journey

**Narrative Arc:**
1. **Motivation:** Interest in blockchain privacy and scaling
2. **Foundation:** Week 1-2 - Rust basics, Merkle trees
3. **Theory:** Week 3-7 - R1CS, QAP, pairings, Groth16 protocol
4. **Practice:** Week 8-9 - Circuit design, real-world applications
5. **Capstone:** Week 12 - Complete tutorial with implementation
6. **Key Learnings:** Mathematical depth, implementation challenges, production considerations

**Sample Answer (Structured):**
"My ZK learning journey started 12 weeks ago with a goal to understand privacy-preserving technologies. I began with Rust fundamentals - hashing, serialization, Merkle trees. Then I dove into the theory: R1CS constraint systems, QAP polynomial transformations, elliptic curves and pairings. I implemented the Groth16 protocol from scratch, learning trusted setup, proof generation, and verification.

The capstone was building a complete tutorial - writing 7 chapters covering theory through real-world applications. I implemented example circuits, analyzed production systems like Zcash and zkSync, and documented security considerations. The biggest challenges were understanding the mathematical proofs and debugging constraint systems. Key insight: ZK is powerful but requires careful security engineering - trusted setup, circuit audits, and formal verification are essential for production."

**Projects to Highlight:**
- Implemented R1CS to QAP transformation with Lagrange interpolation
- Built Groth16 proving/verification using arkworks
- Designed circuits: multiplier, range proof, Merkle membership
- Analyzed production systems: Zcash circuits, ZK-rollup architectures
- Created comprehensive tutorial and threat model

**Follow-up Questions:**
- What was the hardest concept to understand?
- What would you do differently?
- What's your next ZK project?

---

### Q15: What Was Your Most Challenging Bug?

**Example Story:**

**Problem:** Chapter 3 (QAP) had incorrect polynomial interpolation
- R1CS matrix A[2,1] was 0, should be 1
- A₁(x) polynomial was wrong: x²-3x+3 instead of x²-4x+4

**Discovery Process:**
1. Code review revealed mathematical inconsistency
2. Manually recalculated Lagrange interpolation
3. Found error in interpolation coefficients
4. Fixed all affected polynomials

**Root Cause:**
- Misapplied Lagrange formula for point (3,0)
- Used y=0 incorrectly in ℓ₁(x) calculation

**Lesson Learned:**
"Always verify calculations step-by-step. Mathematical errors propagate silently - a single wrong coefficient breaks the entire QAP. Now I always double-check polynomial interpolations with multiple methods."

**Alternative Story (if relevant):**
"Prover was generating proofs that failed verification. Root cause: I wasn't normalizing public inputs correctly - they weren't being multiplied by the IC matrix. Lesson: always check the data flow from public inputs through the verification equation."

---

### Q16: How Would You Improve Our ZK System?

**Framework:**
1. **Understand Current System** - Ask clarifying questions
2. **Identify Bottlenecks** - Performance, security, usability
3. **Propose Solutions** - Prioritized by impact/effort
4. **Discuss Trade-offs** - No perfect solution

**Sample Framework Response:**
"I'd need to understand your current system first. What's your application? What's the constraint count? What are your pain points? Generally, I'd look at three areas:

**Performance:** If prover time is the bottleneck, consider hardware acceleration (GPUs, FPGAs) or circuit optimization. If verification is slow, look at batch verification.

**Security:** Review threat model - are you using MPC ceremony? Have you had audits? Consider formal verification for critical circuits.

**Usability:** Is circuit development easy? Consider higher-level DSLs like Circom or Noir.

Let me ask about your specific constraints and I can give more targeted recommendations."

---

## Part 5: Behavioral Questions

### Q17: Tell Me About a Time You Had to Learn Something Complex Quickly

**STAR Method:**
- **Situation:** Week 3 of ZK learning - needed to understand elliptic curves
- **Task:** Learn pairings, curve arithmetic, group theory in 1 week
- **Action:**
  - Read Vitalik's blog posts on ZK-SNARKs
  - Studied arkworks documentation
  - Implemented pairing operations from scratch
  - Worked through concrete examples
- **Result:** Understood bilinear pairings well enough to explain Chapter 4

**Key Themes:** Resourcefulness, structured learning, hands-on practice

---

### Q18: Describe a Project Where You Had Limited Resources

**Example:**
"Building the ZK tutorial with no external guidance. I had the Groth16 paper and arkworks docs, but no step-by-step tutorials. I broke it into 12 weekly milestones, focused on fundamentals first, then built up. When stuck on polynomial interpolation, I worked through examples manually. The constraint was time - I had to balance depth with progress. Solution: implement first, optimize later. Got curious, dive deep, but ship working code."

---

## Part 6: Questions to Ask the Interviewer

### For Protocol Teams:
1. "What ZK applications are you building? (Rollups, privacy, identity?)"
2. "What proving system do you use? (Groth16, PLONK, STARKs?)"
3. "How do you handle trusted setup? (MPC ceremony, universal setup?)"
4. "What's your constraint budget? (Are you optimizing for prover or verifier?)"
5. "Have you had security audits? What were the findings?"

### For Applied Crypto Roles:
1. "What's the biggest security challenge you're facing?"
2. "How do you balance mathematical elegance with practical performance?"
3. "What's your approach to circuit testing and verification?"
4. "Are you using any formal verification tools?"
5. "What's the roadmap for post-quantum migration?"

### For Engineering Teams:
1. "What's the development workflow for circuits? (DSL, Rust framework?)"
2. "How do you benchmark and profile proving performance?"
3. "What's your deployment strategy? (How do you update circuits?)"
4. "How do you monitor production ZK systems?"
5. "What's the team split between protocol engineering and application development?"

---

## Part 7: Quick Reference (Cheat Sheet)

### Constraint Costs (Rule of Thumb)
- Addition: 1 constraint
- Multiplication: 1 constraint
- Boolean check (x ∈ {0,1}): 1 constraint
- Bit decomposition (8-bit): ~8 constraints
- Range check (8-bit): ~50-60 constraints
- SHA-256: ~25,000 constraints
- Poseidon hash: ~300 constraints
- Merkle membership (depth-8): ~2,400 constraints

### Performance Benchmarks
- Simple multiplier: 3 constraints, prove in ~10ms
- Range proof (age ≥ 18): ~60 constraints, prove in ~50ms
- Merkle membership: ~2,400 constraints, prove in ~500ms
- ZK-rollup batch: ~200,000 constraints, prove in ~2-5 minutes
- Zcash Sapling: ~70,000 constraints, prove in ~1 minute

### Key Papers to Reference
- Groth16: "On the Size of Pairing-based Non-Interactive Arguments" (2016)
- Pinocchio: "Pinocchio: Nearly Practical Verifiable Computation" (2013)
- GM17: "Improved Short Proofs from Simple Assumptions" (2017)
- PLONK: "PLONK: Permutations over Lagrange-bases for Oecumenical Non-interactive arguments of Knowledge" (2019)
- Marlin: "Marlin: Preprocessing zkSNARKs with Universal and Updatable SRS" (2020)

### Key Libraries
- arkworks-rs: https://github.com/arkworks-rs
- bellman: https://github.com/zcash/bellman
- libsnark: https://github.com/scipr-lab/libsnark
- circom: https://github.com/iden3/circom
- noir: https://github.com/noir-lang/noir

### Common Acronyms
- ZKP: Zero-Knowledge Proof
- SNARK: Succinct Non-Interactive Argument of Knowledge
- STARK: Scalable Transparent Argument of Knowledge
- R1CS: Rank-1 Constraint System
- QAP: Quadratic Arithmetic Program
- MPC: Multi-Party Computation
- TVL: Total Value Locked
- TPS: Transactions Per Second

---

## Part 8: Practice Problems

### Problem 1: Design a Circuit
**Task:** Design a circuit to prove knowledge of a secret password hash

**Solution Outline:**
1. Public: Hash output H
2. Private: Password p
3. Constraint: H = SHA-256(p)
4. Constraint count: ~25,000
5. Optimization: Use Poseidon instead (~300 constraints)

### Problem 2: Optimize a Circuit
**Task:** Given a circuit computing x³ + 2x² + x = y, optimize it

**Current:** Naive approach, 7 constraints
**Optimized:** Factor to x(x+1)(x+2) = y, 3 constraints

### Problem 3: Debug a Failed Proof
**Symptom:** Proof generation succeeds but verification fails
**Debugging Steps:**
1. Check public inputs match between prover and verifier
2. Verify VK is correct (not corrupted)
3. Check subgroup constraints (A, B, C in correct groups)
4. Validate pairing equation components
5. Most common: Public input mismatch

---

## Closing Tips

1. **Be Honest:** If you don't know, say "I'm not sure, but here's how I'd figure it out"
2. **Show Your Work:** Walk through your thought process
3. **Connect to Practice:** Always link theory to real implementations
4. **Highlight Learnings:** Show growth mindset
5. **Ask Good Questions:** Demonstrates engagement and depth

## Recommended Further Preparation

1. **Implement from scratch:** Build a simple ZK system (even a toy one)
2. **Read papers:** Groth16, Pinocchio, PLONK - understand proofs
3. **Contribute to open source:** arkworks, circom, or similar
4. **Build a portfolio project:** ZK-rollup, privacy coin, identity system
5. **Practice explaining:** Teach ZK concepts to others

---

**Good luck with your interviews! You have deep knowledge from 12 weeks of intensive study. Trust your understanding.**
