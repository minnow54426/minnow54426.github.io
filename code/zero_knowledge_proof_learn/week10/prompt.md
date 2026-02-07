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