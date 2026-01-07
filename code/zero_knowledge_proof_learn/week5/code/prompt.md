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