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