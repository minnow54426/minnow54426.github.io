# Week 12 — Capstone polish + portfolio + interview narrative
### Role in the whole picture
This turns “it works on my machine” into “someone can review and trust my work.” This is critical for hiring.

### Learn
- Threat model basics: what assumptions your scheme relies on
- How to communicate ZK statements precisely
- Performance reporting (even simple timing)

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