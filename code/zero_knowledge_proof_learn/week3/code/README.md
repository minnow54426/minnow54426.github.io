# tx-rs: Educational Transaction and Signature Library

A clean, simple Rust library demonstrating how blockchain transactions work with digital signatures. This project is designed for educational purposes to help understand the fundamental concepts of authorized transactions in blockchain systems.

## 🎯 Learning Objectives

After studying this code, you'll understand:

- **Signing vs Hashing**: What each cryptographic operation guarantees
- **Transaction Structure**: How blockchain transactions are organized
- **Digital Signatures**: How transactions are authorized without revealing private keys
- **Replay Protection**: How nonces prevent transaction replay attacks
- **Deterministic Encoding**: Why consistent serialization matters for signatures
- **Mempool Management**: How nodes manage pending transactions

## 📋 Requirements Met

✅ Create crate `tx-rs` with:
- `Transaction { from_pubkey, to_pubkey, amount: u64, nonce: u64 }`
- `TxId = Hash32(sha256(serialize(tx)))`
- `SignedTransaction { tx, sig }`
- `sign(tx, sk) -> sig`
- `verify(signed_tx) -> bool`

✅ Tests:
- Sign then verify passes
- Modifying any field breaks signature

✅ Extra features:
- Mempool with basic dedup by TxId
- Comprehensive examples and documentation

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
tx-rs = { path = "." }
ed25519-dalek = "2.0"
rand = "0.8"
```

## 📚 Usage Examples

### Basic Transaction

```rust
use tx_rs::{Transaction, sign, Mempool};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate keys for Alice and Bob
    let mut csprng = OsRng;
    let alice_key = SigningKey::generate(&mut csprng);
    let bob_key = SigningKey::generate(&mut csprng);

    // Create transaction: Alice sends 100 units to Bob
    let tx = Transaction::new(
        alice_key.public_key(),
        bob_key.public_key(),
        100, // Amount
        1,    // Nonce (prevents replay attacks)
    );

    // Alice signs the transaction
    let signature = sign(&tx, &alice_key);

    // Create signed transaction
    let signed_tx = tx_rs::SignedTransaction::new(tx, signature);

    // Verify the signature
    assert!(signed_tx.verify());

    // Add to mempool
    let mut mempool = Mempool::new();
    mempool.add_transaction(signed_tx)?;

    Ok(())
}
```

### Complete Workflow

```rust
// 1. Create transaction
let tx = Transaction::new(alice_pubkey, bob_pubkey, 100, 1);

// 2. Sign with Alice's private key
let signature = sign(&tx, &alice_secret_key);

// 3. Create signed transaction
let signed_tx = SignedTransaction::new(tx, signature);

// 4. Verify signature (anyone can do this)
let is_valid = signed_tx.verify();

// 5. Add to mempool if valid
if is_valid {
    mempool.add_transaction(signed_tx)?;
}
```

## 🧮 Key Concepts Explained

### What is a Nonce?

A nonce (number used once) is a counter that ensures each transaction from an account is unique. This prevents **replay attacks** where someone could resubmit the same transaction multiple times.

**Example**: If Alice sends 10 coins with nonce 1, her next transaction must use nonce 2, even if it's to the same person.

### Why Deterministic Encoding Matters

Digital signatures work on the exact byte representation of data. If the same transaction could be serialized in different ways, the signature would be invalid for one of them.

**This library uses JSON serialization** which always produces the same bytes for the same transaction data.

### Public vs Private Keys

- **Private Key**: Secret, used to sign transactions (proves ownership)
- **Public Key**: Public, used to verify signatures (identifies the account)
- **Signature**: Proves the private key holder authorized the transaction without revealing the private key

### Transaction ID (TxId)

The TxId is a SHA256 hash of the serialized transaction. It serves as:
- A unique identifier for the transaction
- A way to deduplicate transactions in the mempool
- A reference point when transactions are included in blocks

## 🏗️ Architecture

```
Transaction (data)
    ↓ serialize()
[bytes]
    ↓ sign()
Signature (authorization)
    ↓ combine()
SignedTransaction (tx + signature)
    ↓ verify()
Boolean (valid/invalid)
    ↓ add_to_mempool()
Mempool (pending transactions)
```

## 📁 Project Structure

```
src/
├── lib.rs          # Main library interface
├── transaction.rs  # Transaction struct and TxId
├── crypto.rs       # Signing and verification logic
└── mempool.rs      # Transaction pool management

examples/
├── basic_transaction.rs  # Simple transaction workflow
└── mempool_demo.rs       # Advanced mempool features

tests/
└── integration_tests.rs  # Comprehensive workflow tests
```

## 🧪 Running Examples

```bash
# Basic transaction example
cargo run --example basic_transaction

# Advanced mempool demonstration
cargo run --example mempool_demo

# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## 🔍 Security Notes

**⚠️ Educational Use Only**
This library is designed for learning, not production use. For real blockchain applications, consider:

- More robust error handling
- Additional validation rules
- Network protocol considerations
- Economic incentive mechanisms
- Formal verification

**Security Best Practices Demonstrated:**
- Constant-time comparisons (handled by ed25519-dalek)
- Proper key generation using cryptographically secure randomness
- Replay protection via nonces
- Signature verification before accepting transactions

## 📖 Further Learning

1. **Ethereum Transactions**: https://ethereum.org/en/developers/docs/transactions/
2. **Digital Signatures**: https://cryptobook.nakov.com/digital-signatures
3. **ed25519 Documentation**: https://docs.rs/ed25519-dalek/

## 🤝 Contributing

This is an educational project. Feel free to:
- Add more examples
- Improve documentation
- Suggest clarifications
- Report educational issues

## 📄 License

This project is open source and available under the MIT License.