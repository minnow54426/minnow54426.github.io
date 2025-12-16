```md
# 12-Week (3-Month) Plan — Blockchain + Zero Knowledge Proofs in Rust (10h/week)

This plan is designed so you always know:
- **Role in the whole picture** (why this week exists)
- **What to learn** (concepts)
- **Materials** (high-signal references)
- **Coding goals** (deliverables you can ship)
- **Checks** (how you know you’re done)

Assumed cadence each week (~10h):
- **3h** reading/notes
- **6h** coding + tests
- **1h** recap (README + what you learned + next steps)

---

## Whole-picture map (where all weeks fit)

### Blockchain stack
1. **Crypto primitives**: hashes, Merkle trees, signatures  
2. **Data structures**: txs, blocks, headers, state  
3. **STF (state transition function)**: validity rules + state updates  
4. **Consensus + networking**: agreement, forks, finality, gossip  
5. **Apps**: tokens, identity, scaling, privacy

### ZK stack
1. **ZK theory**: statements/witnesses, completeness/soundness/ZK  
2. **Constraints/circuits**: R1CS mindset (prove computation)  
3. **Proving systems**: setup/prove/verify (SNARK/STARK families)  
4. **Gadgets**: Merkle verification, hashes, range checks, etc. “in-circuit”  
5. **Apps**: membership/credentials, privacy, rollups

### Bridge (integration)
- Use ZK to prove **“the rules were followed”** without revealing private inputs or to compress verification (rollup).

---

# Week 1 — Rust foundations for crypto & protocol code
### Role in the whole picture
Everything else depends on writing correct Rust that manipulates bytes, errors, and serialization safely.

### Learn
- Ownership/borrowing: enough to write clean APIs without fighting the compiler
- Modules/crates, `pub` visibility, traits (basic)
- Error handling: `Result`, `anyhow`, `thiserror`
- Serialization: deterministic binary encoding (prefer `bincode`)
- Testing: unit tests, table-driven tests

### Materials
- The Rust Book: https://doc.rust-lang.org/book/
  - Focus: Ch 3–5, 7–9, 11
- Rust by Example (spot-check topics): https://doc.rust-lang.org/rust-by-example/
- `anyhow`: https://docs.rs/anyhow/
- `thiserror`: https://docs.rs/thiserror/
- `serde`: https://serde.rs/
- `bincode`: https://docs.rs/bincode/

### Coding goals (deliverables)
Create repo `rust-protocol-basics`:
- `bytes` module:
  - hex encode/decode helpers (`hex` crate OK)
  - `to_bytes()` for your structs via `bincode`
- `hash` module:
  - `sha256(data: &[u8]) -> [u8; 32]`
- `types` module:
  - define `Hash32([u8; 32])` newtype + display in hex

### Checks (done when)
- `cargo test` passes
- `cargo fmt` + `cargo clippy` clean enough (few/no warnings)
- README shows: serialize a struct → hash it → print hash

### Extra (optional, if time)
- Tiny CLI: `hash "hello"` prints SHA-256

---

# Week 2 — Merkle trees (commitment to a set)
### Role in the whole picture
Merkle trees appear in blockchains (transaction trees, state trees) and ZK apps (membership proofs). This week builds a reusable primitive you’ll later re-use inside ZK circuits (conceptually, and possibly in-circuit).

### Learn
- Merkle tree construction choices (pairing, padding)
- Inclusion proofs (authentication path)
- Domain separation idea (avoid ambiguity: leaf-hash vs internal-node-hash)

### Materials
- Merkle tree overview: https://en.wikipedia.org/wiki/Merkle_tree
- Ethereum blog “Merkling in Ethereum”: https://blog.ethereum.org/2015/11/15/merkling-in-ethereum/
- `sha2` crate: https://docs.rs/sha2/
- `proptest` (optional for robustness): https://docs.rs/proptest/

### Coding goals
Create crate `merkle-rs`:
- `MerkleTree::from_leaves(leaves: Vec<Vec<u8>>) -> MerkleTree`
- `root() -> Hash32`
- `prove(index) -> MerkleProof { siblings: Vec<Hash32>, path_bits: Vec<bool> }`
- `verify(root, leaf, proof) -> bool`

### Checks
- Tests:
  - deterministic root for same leaves
  - valid proof verifies
  - tamper leaf/proof/root fails
- Document your hashing scheme (leaf vs node hashing) in README

### Extra (optional)
- Benchmark: root computation for 10k leaves

---

# Week 3 — Signatures + transactions (authorization)
### Role in the whole picture
A blockchain is a state machine driven by *authorized* messages. Signatures answer: “who is allowed to move funds/change state?”

### Learn
- Signing vs hashing (what each guarantees)
- Transaction fields, canonical encoding
- Nonce/replay protection intuition

### Materials
- Transactions (Ethereum docs, conceptual): https://ethereum.org/en/developers/docs/transactions/
- Digital signatures primer: https://cryptobook.nakov.com/digital-signatures
- `ed25519-dalek`: https://docs.rs/ed25519-dalek/

### Coding goals
Create crate `tx-rs`:
- `Transaction { from_pubkey, to_pubkey, amount: u64, nonce: u64 }`
- `TxId = Hash32(sha256(serialize(tx)))`
- `SignedTransaction { tx, sig }`
- `sign(tx, sk) -> sig`
- `verify(signed_tx) -> bool`

### Checks
- Tests:
  - sign then verify passes
  - modifying any field breaks signature
- README: explain nonce and why deterministic encoding matters

### Extra (optional)
- Add “mempool” vector and basic dedup by TxId

---

# Week 4 — Minimal state transition function (STF) + blocks
### Role in the whole picture
This is “blockchain core engineering”: define state, define validity, apply transitions deterministically. Later, a ZK rollup proves the STF was applied correctly.

### Learn
- Account-based state model (balances + nonces)
- Block structure: prev hash + list of txs + metadata
- Validity rules: signature valid, sufficient balance, nonce correct

### Materials
- Mastering Bitcoin (free): https://github.com/bitcoinbook/bitcoinbook  
  - Focus: Ch 1–2 (concepts), skim tx chapters as needed
- Mastering Ethereum: https://github.com/ethereumbook/ethereumbook  
  - Focus: Ch 1–3 (overview)

### Coding goals
Repo `toychain-rs`:
- `State` with `HashMap<PubKey, Account { balance, nonce }>`
- `apply_tx(state, signed_tx) -> Result<()>`
- `Block { prev_hash, txs, height, timestamp }`
- `apply_block(state, block) -> Result<()>`
- `block_hash(block) -> Hash32`

### Checks
- End-to-end test:
  - genesis balances
  - create keys
  - sign txs
  - include in blocks
  - verify final balances/nonces

### Extra (optional)
- Add block validation: all txs valid, no duplicate nonces for same sender within a block

---

# Week 5 — Forks + consensus concepts + refactor to “real” structure
### Role in the whole picture
Consensus and forks explain why distributed chains need rules beyond “apply blocks.” You’ll keep it toy-level but learn the mental model used in real clients.

### Learn
- Forks and fork-choice rules (longest chain as toy model)
- Finality concepts (probabilistic vs deterministic)
- Separation of concerns: `core` vs `p2p` vs `consensus` modules

### Materials
- Ethereum consensus overview: https://ethereum.org/en/developers/docs/consensus-mechanisms/
- Princeton Bitcoin course (forks/consensus lectures): https://www.coursera.org/learn/cryptocurrency
- libp2p overview (conceptual): https://libp2p.io/

### Coding goals
In `toychain-rs`:
- Refactor:
  - `core/types`, `core/state`, `core/tx`, `core/block`
- Implement a fork simulation:
  - store blocks in a map by hash
  - allow two competing tips
  - implement toy fork-choice: pick chain with highest height

### Checks
- Test:
  - create fork at height N
  - extend one branch
  - ensure canonical tip updates
- Architecture is readable; modules have clear responsibilities

### Extra (optional)
- Persist blocks to disk with `sled` or simple file storage

---

# Week 6 — ZK foundations: statement/witness + what ZK guarantees
### Role in the whole picture
You learn how to *specify* what you want to prove. This is prerequisite to circuits. Most ZK bugs start as “wrong statement”.

### Learn
- ZK properties: completeness, soundness, zero-knowledge (intuitive)
- Statement vs witness
- NP framing: “exists witness such that relation holds”
- Examples:
  - hash preimage
  - Merkle membership

### Materials
- ZKProof intro hub: https://zkproof.org/
- Vitalik context on SNARK/STARK tradeoffs: https://vitalik.ca/general/2017/11/09/starks_part_1.html
- Awesome ZK (use as directory, don’t read all): https://github.com/matter-labs/awesome-zero-knowledge-proofs

### Coding goals
In new repo `zk-notes-and-relations`:
- Write `notes/zk-model.md`:
  - define 3 statements with (public inputs, witness)
- Implement “relation checkers” (normal Rust):
  - `check_preimage(x, y)`
  - `check_merkle_membership(leaf, path, root)`

### Checks
- You can articulate precisely:
  - what is public
  - what is private
  - what verifier learns

### Extra (optional)
- Write a 1-page note: “Why ZK is not encryption”

---

# Week 7 — Constraints mindset (R1CS) with arkworks
### Role in the whole picture
This is the bridge from normal computation to *provable computation*. You’ll learn to express logic as constraints.

### Learn
- Finite fields (operational understanding)
- What R1CS is (high level)
- Gadgets concept: representing values/ops inside the constraint system

### Materials
- Arkworks org: https://github.com/arkworks-rs
- `ark-relations`: https://docs.rs/ark-relations/
- `ark-r1cs-std`: https://docs.rs/ark-r1cs-std/
- R1CS/QAP overview (skim): https://arxiv.org/abs/1906.07221

### Coding goals
Repo `zk-arkworks-lab`:
- Create a simple circuit:
  - Prove knowledge of `a, b` such that `a * b = c` (public `c`)
- Build constraint system, generate witness, check satisfaction

### Checks
- Tests:
  - valid witness satisfies constraints
  - invalid witness fails
- README explains the statement in (public, witness) form

### Extra (optional)
- Add a second circuit: `a + b = c`

---

# Week 8 — First end-to-end SNARK in Rust (Groth16)
### Role in the whole picture
You now can produce a proof artifact that a verifier can check quickly. This is the “SNARK pipeline” you’ll reuse in the capstone.

### Learn
- Setup/Prove/Verify lifecycle (and what “trusted setup” means for Groth16)
- Public inputs encoding order and correctness

### Materials
- `ark-groth16`: https://docs.rs/ark-groth16/
- Groth16 paper (reference only): https://eprint.iacr.org/2016/260

### Coding goals
In `zk-arkworks-lab`:
- Turn Week 7 circuit into Groth16 proof:
  - generate parameters (pk/vk)
  - create proof for witness
  - verify proof with public inputs

### Checks
- Tests:
  - proof verifies
  - changing public input breaks verification
  - wrong proof fails

### Extra (optional)
- Persist pk/vk to disk and reload (serialization)

---

# Week 9 — ZK Merkle membership circuit (core ZK application)
### Role in the whole picture
Merkle membership is a foundational ZK pattern used in credentials, allowlists, mixers, anonymous voting, and more.

### Learn
- Merkle verification “in-circuit” (constraints)
- SNARK-friendly hash consideration (Poseidon/MiMC vs SHA)
- Public root + private leaf/path modeling

### Materials
- Semaphore docs (pattern reference): https://semaphore.pse.dev/
- Poseidon hash paper (reference): https://eprint.iacr.org/2019/458
- `ark-crypto-primitives` (hash/merkle tools vary by version): https://docs.rs/ark-crypto-primitives/

### Coding goals
In `zk-arkworks-lab` (or new repo `zk-merkle-membership`):
- Implement a circuit proving:
  - given public `root`
  - private `leaf` and `path`
  - prover shows membership
- Provide a CLI or test harness that:
  - builds a Merkle tree (off-circuit)
  - extracts a path
  - generates proof
  - verifies proof

### Checks
- Tests:
  - valid path verifies
  - wrong leaf/path fails
  - wrong root fails
- README includes:
  - statement definition
  - what is private/public
  - limitations (hash choice, tree depth fixed, etc.)

### Extra (optional)
- Add “nullifier” concept (conceptual):
  - prove knowledge of leaf + secret, output public nullifier = H(secret, root)
  - (Even if not fully production-grade, helps you understand anonymity apps.)

---

# Week 10 — Integration view: where proofs live in blockchain systems
### Role in the whole picture
This week connects the ZK proof artifact to “system boundaries”: file formats, APIs, verifiers, and how blockchains would consume proofs.

### Learn
- Off-chain prover vs on-chain verifier responsibilities
- What data must be published (public inputs, sometimes calldata/data availability)
- Performance knobs: proving time vs verification time

### Materials
- Ethereum scaling overview: https://ethereum.org/en/developers/docs/scaling/
- zk-rollups overview: https://ethereum.org/en/developers/docs/scaling/zk-rollups/
- MoonMath Manual (use as reference): https://github.com/LeastAuthority/moonmath-manual

### Coding goals
Create repo `zk-proof-artifacts` (or extend capstone repo skeleton):
- Define a proof package format:
  - `public_inputs.json`
  - `proof.bin` (or JSON if library supports)
  - `vk.bin`
- Build CLI:
  - `prove --witness witness.json --vk vk.bin --pk pk.bin -> proof.bin`
  - `verify --proof proof.bin --vk vk.bin --public public.json`

### Checks
- Reproducible demo steps in README
- One command generates artifacts, another verifies them

### Extra (optional)
- Add basic benchmarking output (elapsed time)

---

# Week 11 — Capstone build (job-relevant)
### Role in the whole picture
You consolidate skills into a coherent application with clear statement design, correct proof plumbing, and clean Rust engineering.

### Recommended capstone (fits your current path)
**ZK Membership Credential (Merkle allowlist)**:
- Public: Merkle root, maybe an “app scope”
- Private: leaf data + path
- Output: proof that user is in allowlist without revealing which entry

### Materials
- Semaphore concepts (again): https://semaphore.pse.dev/
- PSE repos (practical ZK engineering inspiration): https://github.com/privacy-scaling-explorations
- Rust project organization guidance:
  - Rust API guidelines: https://rust-lang.github.io/api-guidelines/

### Coding goals
Repo `zk-capstone-membership`:
- Modules:
  - `circuits/` (membership circuit)
  - `prover/` (generate proof)
  - `verifier/` (verify proof)
  - `types/` (public inputs, witness format)
- Provide:
  - `make_demo_data` (build tree, choose leaf, produce witness)
  - `prove`
  - `verify`

### Checks
- End-to-end demo works from clean checkout
- Tests cover:
  - happy path
  - at least 2 failure cases (wrong root, wrong path)

### Extra (optional)
- Add “revocation” simulation by changing root

---

# Week 12 — Capstone polish + portfolio + interview narrative
### Role in the whole picture
This turns “it works on my machine” into “someone can review and trust my work.” This is critical for hiring.

### Learn
- Threat model basics: what assumptions your scheme relies on
- How to communicate ZK statements precisely
- Performance reporting (even simple timing)

### Materials
- README guide: https://www.makeareadme.com/
- OWASP threat modeling (lightweight reference): https://owasp.org/www-community/Threat_Modeling
- Awesome ZK list for quick gap-filling: https://github.com/matter-labs/awesome-zero-knowledge-proofs

### Coding goals
- Add:
  - Clear top-level README with:
    - statement definition
    - public vs private
    - how to run demo
    - limitations and assumptions
  - `docs/architecture.md` with a diagram (mermaid ok)
  - Bench output (proving time, verifying time)
  - `cargo deny` or dependency review note (optional)

### Checks
- A reviewer can:
  - run demo in <10 minutes
  - understand the statement and privacy property
- You can explain in 2–3 minutes:
  - what is proven
  - what remains private
  - what the verifier checks
  - what trusted setup means (if using Groth16)

---

## Notes / Practical guidance (important)
### Keep circuits small and fixed-shape
- Merkle trees in-circuit usually have **fixed depth**.
- Start with depth 8 or 16 for learning; scale later.

### Avoid getting stuck on “perfect hash choice”
- For learning, prefer what has solid arkworks support.
- SHA-256 in-circuit is often heavy; SNARK-friendly hashes are common in ZK apps.

### Portfolio tip
Pin 2–3 repos:
- `toychain-rs` (STF + blocks + forks)
- `merkle-rs` (clean primitive)
- `zk-capstone-membership` (end-to-end ZK app)

---

## If you want, I can tailor the plan further
Answer:
1) Do you want to target **Ethereum/ZK-rollup ecosystem** specifically, or general blockchain?
2) Do you prefer capstone as **CLI-only**, or **CLI + small web API**?
```
