# Unified ZK-Rollup Learning Journey - Design Document

**Date:** 2026-02-07
**Status:** Approved
**Author:** Learning Journey Design

## Executive Summary

Transform 12 weeks of isolated ZK/blockchain learning projects into a unified, progressively-built ZK-rollup system. Each week contributes to a complete provable state machine, creating a cohesive portfolio piece that demonstrates deep understanding from first principles to working system.

**Current State:** 12 separate week directories, each with standalone projects
**Future State:** Single `zk-rollup-learning` workspace with 5 integrated crates
**Migration Timeline:** 14 days
**Final Deliverable:** Complete ZK-rollup with portfolio website, documentation, and working demo

## The North Star Vision

Build a complete ZK-rollup where every component is constructed from first principles. By Week 12, learners have a working system that demonstrates understanding of the entire proving pipeline.

**The Story Arc:** Every piece answers: *How do we build a machine whose state transitions can be verified without re-execution?*

## Architectural Principles

1. **Progressive Construction** - No throwaway code. Week 2's Merkle tree is used in Week 9's membership circuit.
2. **Unified Codebase** - One repository with workspace structure, not 12 separate projects.
3. **Clear Interfaces** - Each crate defines public APIs that subsequent weeks consume.
4. **Test-Driven Learning** - Every component has tests verifying correctness AND integration.

## Repository Structure

```
zk-rollup-learning/
├── Cargo.toml                 # Workspace root
├── crates/
│   ├── crypto/               # Week 1: Hashing, serialization
│   ├── state/                # Week 2-5: Merkle, txs, STF, batches
│   ├── zk/                   # Week 6-9: Circuits, Groth16
│   ├── rollup/               # Week 10-11: Artifacts, node
│   └── demo/                 # Week 12: Web demo, docs
├── docs/
│   ├── tutorial/             # 4-chapter learning guide
│   ├── architecture/         # System design, security, performance
│   ├── api/                  # API reference
│   └── learning-journey.md   # 12-week retrospective
├── examples/
│   ├── simple_transfer.rs    # Week 3 demo
│   ├── batch_processing.rs   # Week 5 demo
│   ├── zk_proof.rs           # Week 8 demo
│   └── full_rollup.rs        # Week 11 complete flow
├── tests/
│   ├── integration_*.rs      # Cross-crate tests
│   └── e2e/                  # End-to-end workflows
└── scripts/
    └── validate.sh           # Automated validation
```

## Week-by-Week Progressive Construction

### Phase 1: Crypto Foundations (Weeks 1-2)

**Week 1 - Rollup Data Types**
- **Build**: `crates/crypto` with `Hash32`, `Bytes`, serialization
- **Rollup relevance**: Every rollup component needs consistent data representation
- **Integration**: Used by every subsequent crate
- **Tests**: Hash examples, encode/decode roundtrips

**Week 2 - State Commitment Scheme**
- **Build**: `crates/state` with Merkle tree
- **Rollup relevance**: Rollup state is a Merkle tree of account balances
- **Design**: Fixed-depth tree (depth 8 = 256 accounts) for circuit simplicity
- **Integration**: Week 4 (state storage), Week 9 (in-circuit verification)

### Phase 2: State Machine (Weeks 3-5)

**Week 3 - Transaction Model**
- **Build**: `Transaction`, `SignedTransaction` types
- **Rollup relevance**: Users submit transfer transactions
- **Key insight**: Nonce replay prevention
- **Integration**: Week 4 (state transition), Week 7 (circuit witness)

**Week 4 - State Transition Function**
- **Build**: `State::apply_transaction()`, `State::apply_batch()`
- **Rollup relevance**: Core logic proved in ZK later
- **Critical**: Balance checks, nonce validation, state updates
- **Integration**: Direct execution (Week 4), proved execution (Week 7-9)

**Week 5 - Batch Management**
- **Build**: `Batch` struct with ordered txs, state roots
- **Rollup relevance**: Batches are unit of proving
- **Simplified**: Fixed batch size (4 txs) for predictable circuit size
- **Integration**: Week 8 (proving), Week 10 (artifact format)

### Phase 3: ZK Proving (Weeks 6-9)

**Week 6 - Proving Strategy**
- **Build**: `docs/proof-strategy.md` defining ZK statement
- **Public**: old_state_root, new_state_root, batch_hash
- **Private**: transactions, Merkle paths, witness data
- **Integration**: Guides Week 7-9 circuit design

**Week 7 - Circuit: State Update Logic**
- **Build**: `crates/zk/circuits` with `StateTransitionCircuit`
- **Rollup relevance**: Encodes Week 4's STF as R1CS constraints
- **Operations**: Balance checks (≤), nonce checks, arithmetic updates
- **Integration**: Week 3's tx model, Week 4's validation rules

**Week 8 - Proving Engine**
- **Build**: Integrate `ark-groth16` for setup/prove/verify
- **Rollup relevance**: Generates and verifies proofs for batches
- **Design**: One-time setup per batch size, reusable keys
- **Integration**: Proofs become Week 10 artifacts

**Week 9 - Circuit: Merkle State Access**
- **Build**: `MerkleInclusionCircuit` for in-circuit verification
- **Rollup relevance**: Prove tx inputs/outputs reference valid state leaves
- **Challenge**: SNARK-friendly hash (Poseidon) vs standard (SHA-256)
- **Integration**: Combines with Week 7 circuit for complete rollup proof

### Phase 4: Integration (Weeks 10-12)

**Week 10 - Proof Artifacts System**
- **Build**: `crates/rollup/artifacts` for PK/VK management
- **Rollup relevance**: Standardized format for storing, loading, verifying proofs
- **CLI**: `prove --batch batch.json --pk pk.bin`, `verify --proof proof.bin --vk vk.bin`
- **Integration**: Week 8's proofs become usable artifacts

**Week 11 - Rollup Assembly**
- **Build**: `crates/rollup/node` wiring everything together
- **Rollup relevance**: Complete rollup operator functionality
- **API**: `submit_tx()`, `create_batch()`, `prove_batch()`, `verify_proof()`
- **Demo**: Submit 4 txs → create batch → prove → verify

**Week 12 - Showcase & Portfolio**
- **Build**: `crates/demo/web-demo` + comprehensive documentation
- **Rollup relevance**: Demonstrates complete system understanding
- **Portfolio**: Tutorial, threat model, interview guide, performance benchmarks
- **Integration**: Shows how every piece contributes to the whole

## Learning Narrative Design

### Story Through-Line

**Theme**: "The Provable State Machine"

Each week answers: *How do we build a machine whose state transitions can be verified without re-execution?*

- **Weeks 1-2**: "How do we represent and commit to state?"
- **Weeks 3-5**: "How do we transition state validly?"
- **Weeks 6-7**: "How do we express validity as constraints?"
- **Weeks 8-9**: "How do we prove and verify constraints?"
- **Weeks 10-12**: "How do we make this practical?"

### Cross-Week Connections

**"Call Forward" Pattern** (referencing future weeks in current content):
- Week 2 Merkle tree: "This structure lets us prove account access inside a ZK circuit (Week 9) without revealing which account."
- Week 4 STF: "This nonce check prevents replay. When we encode this as a constraint (Week 7), it becomes part of proof validity."

**"Call Back" Pattern** (referencing past weeks):
- Week 9 circuit: "We use Poseidon hash here. Remember Week 2's SHA-256 Merkle tree? In-circuit, SNARK-friendly hashes are far more efficient."

### Weekly README Template

```markdown
# Week X: [Topic]

## 🎯 This Week's Role
[How this contributes to the provable state machine]

## 📚 What You'll Learn
- [Concept 1]: [Why it matters for the rollup]
- [Concept 2]: [How it connects to Week Y]

## 🚀 What You'll Build
- [Code deliverable]: [How it's used in later weeks]

## 🔗 Connections
← Builds on: [Weeks X-Y]
→ Used by: [Weeks Z]

## ✅ Completion Check
- [ ] Tests pass
- [ ] Integration with [crate] works
- [ ] You can explain: [Key question]
```

## Testing & Validation Strategy

### Three Testing Levels

1. **Unit Tests** (Inside each crate)
   - Test every public function
   - Cover edge cases
   - Run: `cargo test -p <crate>`

2. **Integration Tests** (Between crates)
   - Test that crates work together
   - Located in `tests/integration_*.rs`
   - Run: `cargo test --test integration_*`

3. **End-to-End Tests** (Complete workflows)
   - Test user-facing scenarios
   - Located in `tests/e2e/*.rs`
   - Run: `cargo test --test e2e_*`

### Integration Test Matrix

| Test | Validates | Crates Involved |
|------|-----------|-----------------|
| `test_crypto_serialization` | Data types serialize | `crypto` |
| `test_merkle_inclusion` | Merkle proofs verify | `crypto`, `state` |
| `test_transaction_execution` | Txs update state | `crypto`, `state` |
| `test_batch_state_transition` | Batch produces correct root | `state` |
| `test_circuit_satisfies_constraints` | Witness satisfies circuit | `state`, `zk` |
| `test_groth16_prove_verify` | Proofs generate/verify | `zk` |
| `test_merkle_in_circuit` | In-circuit Merkle verification | `state`, `zk` |
| `test_proof_artifact_serialization` | Proofs serialize/deserialize | `zk`, `rollup` |
| `test_rollup_end_to_end` | Full rollup workflow | All crates |

### Weekly Validation Checklist

At the end of each week:
1. Run all tests: `cargo test --workspace`
2. Run clippy: `cargo clippy --workspace -- -D warnings`
3. Check examples: `cargo check --examples`
4. Verify integration tests
5. Update README

### Automated Guardrails

**`scripts/validate.sh`:**
```bash
#!/bin/bash
set -e
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo doc --no-deps --document-private-items
echo "✅ All validations passed"
```

### Regression Testing Strategy

Keep old week directories as "test oracle":
1. Run tests in old directories, capture output
2. Compare new crate output with old output
3. Validate: If old produces `0xabc...`, new must produce same

## Portfolio & Documentation Structure

### Three Audiences, Three Layers

1. **For Learners**: Tutorial (`docs/tutorial/`)
2. **For Employers**: Architecture (`docs/architecture/`)
3. **For Users**: API Reference (`docs/api/`)

### Key Portfolio Artifacts

1. **Architecture Diagram** - System flow from user to verified proof
2. **Performance Table** - Benchmark results (prove ~10ms, verify ~2ms)
3. **Threat Model** - Security assumptions, mitigations, limitations
4. **Interview Prep** - 18 technical Q&A, behavioral examples

### Portfolio Website

```
portfolio-web/
├── index.html                   # Landing with stats
├── concepts.html                # Interactive explanations
├── demo.html                    # Running rollup demo
├── architecture.html            # System diagrams
├── timeline.html                # 12-week journey
└── docs/                        # Static rendered docs
```

**Resume Bullet:**
> "Built a complete Groth16-based ZK-rollup from scratch in Rust, implementing cryptographic primitives, state transition functions, R1CS circuits, and end-to-end proving pipeline. Documented the entire journey in a 38,000-word tutorial and portfolio website."

## Migration Timeline

### Phase 1: Foundation (Days 1-3)

**Day 1: Workspace Setup**
- Create new `zk-rollup-learning/` repository
- Set up workspace `Cargo.toml`
- Create empty crate structure
- Set up `scripts/validate.sh`

**Day 2-3: Port `crypto` Crate (Week 1)**
- Port `Hash32`, `Bytes`, serialization from week1
- Add comprehensive unit tests
- Write integration test: `test_crypto_serialization`
- Run regression: Compare hashes with old output

### Phase 2: State Machine (Days 4-7)

**Day 4-5: Port `state` Crate - Merkle (Week 2)**
- Port Merkle tree from week2
- Simplify API for rollup use case
- Write integration test: `test_merkle_inclusion`
- Run regression: Compare Merkle roots

**Day 6-7: Port `state` Crate - Transactions & STF (Week 3-4)**
- Port `Transaction`, `SignedTransaction` from week3
- Port `State`, `apply_transaction()` from week4
- Integrate with Merkle tree
- Write integration test: `test_transaction_execution`
- Run regression: Compare state transitions

### Phase 3: ZK Circuits (Days 8-10)

**Day 8: Port `zk` Crate - Circuits (Week 7-9)**
- Port circuit traits from week8
- Port `StateTransitionCircuit` (Week 7)
- Port `MerkleInclusionCircuit` (Week 9)
- Simplify to use unified `crypto` and `state` crates

**Day 9-10: Port Groth16 Integration (Week 8)**
- Port `setup`, `prove`, `verify` functions
- Integrate with circuits
- Add benchmarks for proving time
- Write integration test: `test_groth16_prove_verify`

### Phase 4: Rollup Integration (Days 11-12)

**Day 11: Build `rollup` Crate (Week 10-11)**
- Create `artifacts` module (PK/VK management)
- Create `node` module (submit tx, create batch, prove, verify)
- Write integration test: `test_rollup_end_to_end`
- Create `examples/full_rollup.rs`

**Day 12: Examples & Tests**
- Write progressive examples
- Verify all examples compile and run
- Run full test suite: `cargo test --workspace`

### Phase 5: Documentation (Days 13-14)

**Day 13: Tutorial & Architecture Docs**
- Write `docs/tutorial/*.md` (4 chapters)
- Write `docs/architecture/*.md` (design, security, performance)
- Write `docs/api/*.md` (public API docs)

**Day 14: Portfolio Website**
- Build `portfolio-web/` from Week 12's work
- Add interactive demo runner
- Add architecture diagrams
- Deploy and test locally

## Risk Mitigation

| Risk | Mitigation | Fallback |
|------|------------|----------|
| Migration takes longer | Prioritize weeks 8-12, simplify 1-5 | Keep old dirs as reference |
| Breaking existing code | Git commits at phase boundaries | Easy rollback |
| Integration issues | Integration tests incrementally | Test each crate immediately |
| Learning burnout | 2-week sprint, 4-6 hours/day max | Clear daily goals |

## Success Criteria

**Migration Complete:**
- ✅ All 44+ tests pass in unified workspace
- ✅ Zero clippy warnings (`-D warnings`)
- ✅ `examples/full_rollup.rs` executes end-to-end
- ✅ Portfolio website renders and demo works
- ✅ Documentation covers all 12 weeks
- ✅ Code compiles on clean checkout

**Portfolio Ready:**
- ✅ Can explain the system in 5 minutes
- ✅ Can answer technical questions off the cuff
- ✅ Demo impresses in interview setting
- ✅ Code is clean enough for others to contribute

## Key Design Decisions

### 1. Pedagogical Rollup (Not Production)
**Decision**: Simplified but conceptually complete rollup
- Single operator (no decentralized provers)
- Fixed-size batches (4 txs per proof)
- Transfer-only transactions (no smart contracts)
- Mock "L1" (struct, not real Ethereum)

**Rationale**: Focus on understanding the complete proving pipeline, not production scaling

### 2. SNARK-Friendly Hash in Circuits
**Decision**: Use Poseidon hash in-circuit, SHA-256 off-circuit
**Tradeoff**: Two different hash algorithms
**Rationale**: SNARK-friendly hashes are 100x more efficient in-circuit. This is common practice in real rollups.

### 3. Fixed-Depth Merkle Tree
**Decision**: Depth 8 tree (256 accounts)
**Rationale**: Fixed-depth circuits are simpler. Can document how to extend to dynamic depth.

### 4. Workspace Over Monolith
**Decision**: 5 crates, not 1 giant crate
**Rationale**: Each crate is independently testable, has clear responsibility, easier to understand.

### 5. Clean Slate Migration
**Decision**: Create new workspace, port code (not merge existing)
**Rationale**: Cleaner APIs, designed for integration from day one, refactoring IS learning

## Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Hash transaction | <0.01ms | SHA-256 via `sha2` |
| Verify signature | <0.05ms | Ed25519 via `ed25519-dalek` |
| Apply 4-tx batch | <0.5ms | State transition |
| Generate proof | <10ms | Groth16, batch size 4 |
| Verify proof | <2ms | Groth16 verification |
| **End-to-end** | **<12ms** | Tx to verified proof |

## Dependencies

### Crates
- `ark-groth16` - Groth16 proving system
- `ark-relations` - Constraint system traits
- `ark-r1cs-std` - R1CS standard library
- `ark-crypto-primitives` - Cryptographic gadgets
- `ark-ff`, `ark-ec`, `ark-bls12-381` - Field and curve primitives
- `serde`, `bincode` - Serialization
- `thiserror`, `anyhow` - Error handling
- `sha2` - SHA-256 hashing
- `ed25519-dalek` - Signatures

### Tooling
- `cargo` - Package manager
- `criterion` - Benchmarks
- `rustfmt` - Code formatting
- `clippy` - Linting

## Next Steps

1. ✅ Design approved
2. ⏭️ Create git worktree for isolated migration workspace
3. ⏭️ Begin Day 1: Workspace setup
4. ⏭️ Execute 14-day migration timeline
5. ⏭️ Validate and deploy portfolio

---

**This design transforms isolated learning exercises into a cohesive ZK-rollup system, demonstrating both deep technical understanding and the ability to ship production-quality code.**
