# toychain-rs: Fork-Aware Toy Blockchain in Rust

A minimal blockchain implementation demonstrating fork handling, longest-chain consensus, and modular architecture.

## Overview

This project (Week 5 of the ZK learning journey) extends Week 4's state transition function with:
- **Fork support**: Store multiple competing chains
- **Consensus**: Longest-chain fork-choice rule
- **Reorgs**: Automatic switching to better chains
- **Modular architecture**: Clean separation of concerns

## Architecture

The codebase is organized into core modules:

```
src/
├── core/
│   ├── mod.rs       # Module declarations
│   ├── types.rs     # Common type aliases (Hash, Height, etc.)
│   ├── state.rs     # Account and State management
│   ├── block.rs     # Block structure and hashing
│   ├── tx.rs        # Transaction types (re-exports tx-rs)
│   └── chain.rs     # Blockchain storage and fork handling
└── lib.rs           # Public API exports
```

### Key Concepts

**Forks**: When two blocks are mined at the same height, both are stored. The blockchain maintains a "tip" representing the canonical chain.

**Fork-Choice Rule**: Simple "longest chain" rule - the chain with the highest height wins. On ties, first-to-arrive wins.

**Reorgs**: When a longer chain is discovered, the tip updates to point to it. This simulates how real blockchains handle forks.

## Usage

### Creating a Blockchain

```rust
use toychain_rs::Blockchain;

let mut blockchain = Blockchain::new();

// Add genesis block
let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);
blockchain.add_block(genesis)?;
```

### Creating Forks

```rust
// Both block1a and block1b extend genesis at height 1
let block1a = Block::new(genesis_hash, vec![], 1, 2000);
let block1b = Block::new(genesis_hash, vec![], 1, 2001);

blockchain.add_block(block1a)?;
blockchain.add_block(block1b)?;

// Tip will be block1a (first to arrive)
assert_eq!(blockchain.get_tip(), Some(&hash1a));
```

### Chain Reorganization

```rust
// Extend fork A to height 2
let block2a = Block::new(hash1a, vec![], 2, 3000);
blockchain.add_block(block2a)?;
assert_eq!(blockchain.get_tip(), Some(&hash2a));

// Extend fork B to height 3 (wins!)
let block2b = Block::new(hash1b, vec![], 2, 3001);
blockchain.add_block(block2b)?;

let block3b = Block::new(hash2b, vec![], 3, 4000);
blockchain.add_block(block3b)?;

// Tip reorgs to block3b (height 3 > height 2)
assert_eq!(blockchain.get_tip(), Some(&hash3b));
```

### Getting the Canonical Chain

```rust
let chain = blockchain.get_canonical_chain();
// Returns: [genesis, ..., ..., tip]
for block in &chain {
    println!("Height {}: {}", block.height, hex::encode(block.hash()));
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_fork_resolution

# Run integration tests
cargo test --test integration_test

# Check code
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Fork Handling Details

### Storage
All blocks are stored in a `HashMap<Hash, Block>`, regardless of whether they're on the canonical chain. This preserves fork history.

### Tip Selection
The tip is updated using these rules:
- New block height > current tip height → switch to new block
- New block height == current tip height → keep current tip (no reorg)
- New block height < current tip height → ignore (not canonical)

### Canonical Chain Reconstruction
`get_canonical_chain()` starts from the tip and follows `prev_hash` links backwards to genesis, then reverses the list.

## Limitations (Toy vs. Production)

This is a **toy** blockchain for learning. Key simplifications:

- **No proof-of-work**: Any block can be added
- **No difficulty adjustment**: All blocks are valid
- **No finality**: Tips can change arbitrarily
- **No validation**: Blocks aren't validated beyond basic structure
- **Simple consensus**: Longest chain only (no total difficulty, no GHOST)
- **No networking**: Single-machine only
- **No persistence**: In-memory only (lost on restart)

## What This Teaches

By working through this codebase, you'll learn:
1. **How forks happen**: Concurrent block creation at same height
2. **Fork-choice rules**: Selecting the "best" chain
3. **Chain reorganization**: Switching to a better chain
4. **Modular design**: Separating types, state, blocks, and chain logic
5. **Test-driven development**: Comprehensive tests for fork scenarios

## Next Steps (Week 6+)

This foundation prepares you for:
- ZK foundations (statements, witnesses, relations)
- Constraint systems (R1CS)
- SNARK proving systems
- ZK applications (Merkle membership, rollups)

## Dependencies

- `tx-rs`: Transaction types from Week 3
- `ed25519-dalek`: Digital signatures
- `sha2`: SHA-256 hashing
- `serde`: Serialization
- `anyhow`: Error handling

## License

Educational use only.
