# toychain-rs: Minimal Blockchain State Transition Function

A clean, simple Rust implementation of a blockchain's core state transition logic. This project demonstrates how blocks containing transactions are validated and applied to account state.

## Learning Objectives

After studying this code, you'll understand:

- **State Transition Function (STF)**: How blockchain state changes deterministically
- **Account-Based Model**: How Ethereum-style accounts work (balance + nonce)
- **Transaction Validation**: Signature verification, balance checks, nonce validation
- **Block Structure**: How blocks chain together via hashes
- **Atomic Updates**: How all transactions in a block succeed or fail together

## Requirements Met

✅ Week 4 of ZK Learning Plan:
- `State` with `HashMap<PubKey, Account { balance, nonce }>`
- `apply_tx(state, signed_tx) -> Result<()>`
- `Block { prev_hash, txs, height, timestamp }`
- `apply_block(state, block) -> Result<()>`
- `block_hash(block) -> Hash32`
- End-to-end test with genesis, keys, signed txs, blocks

## Quick Start

```rust
use toychain_rs::{State, Account, Block, block_hash, apply_block};
use tx_rs::{Transaction, SignedTransaction, sign};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut csprng = OsRng;

    // Create accounts
    let mut state = State::new();
    let alice = Keypair::generate(&mut csprng);
    let bob = Keypair::generate(&mut csprng);

    // Genesis: fund Alice
    state.set_account(alice.public, Account::new(100, 0));

    // Create transaction
    let tx = Transaction::new(alice.public, bob.public, 30, 0);
    let sig = sign(&tx, &alice);
    let signed_tx = SignedTransaction::new(tx, sig);

    // Create block
    let block = Block::new([0u8; 32], vec![signed_tx], 1, 1234567890);

    // Apply block
    apply_block(&mut state, &block)?;

    Ok(())
}
```

## Key Concepts

### Account-Based State

Unlike UTXO-based systems (like Bitcoin), this uses an account model similar to Ethereum:

- **Account**: Identified by public key, stores balance and nonce
- **Balance**: Amount of tokens owned
- **Nonce**: Transaction counter, prevents replay attacks

### Transaction Validation Rules

Every transaction must pass ALL checks:

1. **Signature Valid**: Proved ownership of private key
2. **Sufficient Balance**: Sender has enough tokens
3. **Correct Nonce**: Matches account's current nonce

If any check fails, the transaction is rejected.

### Block Application

Blocks contain ordered transactions. All transactions must validate for the block to be applied. If any transaction fails, the entire block fails and no state changes occur.

## Architecture

```
Transaction (from Week 3)
    ↓
SignedTransaction (tx + signature)
    ↓ apply_tx() validation
State.update() [atomic]
    ↓
Block { prev_hash, txs, height, timestamp }
    ↓ apply_block()
State [updated with all txs]
```

## Project Structure

```
src/
├── lib.rs       # Public API exports
├── state.rs     # State, Account, apply_tx
├── block.rs     # Block, block_hash
└── chain.rs     # apply_block

tests/
└── integration_test.rs  # End-to-end workflow
```

## Testing

```bash
# Run all tests
cargo test

# Run integration test with output
cargo test test_end_to_end -- --nocapture

# Run specific test
cargo test test_apply_tx_insufficient_balance
```

## Dependencies

- `tx-rs`: Transaction types from Week 3
- `ed25519-dalek`: Digital signatures
- `sha2`: SHA-256 hashing
- `serde`: Serialization
- `anyhow`: Error handling

## Next Steps

This STF is the foundation for:
- **Week 5**: Forks and consensus
- **Week 6-12**: ZK proofs that STF was applied correctly

## License

MIT
