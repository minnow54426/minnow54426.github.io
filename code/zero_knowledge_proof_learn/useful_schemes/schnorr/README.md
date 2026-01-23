# BIP340 Schnorr Signatures in Rust

A production-ready implementation of BIP340 Schnorr signatures on secp256k1.

## Features

- **BIP340 Compliant**: Compatible with Bitcoin's Schnorr specification
- **Batch Verification**: Verify multiple signatures 5-10x faster
- **Type-Safe API**: Newtype wrappers prevent misuse
- **Constant-Time**: Timing-attack resistant operations
- **Deterministic Nonces**: Prevents nonce reuse bugs

## Installation

```toml
[dependencies]
schnorr = "0.1"
```

## Usage

```rust
use schnorr::KeyPair;
use rand::rngs::OsRng;

let mut rng = OsRng;
let keypair = KeyPair::new(&mut rng);
let message = b"Hello, Schnorr!";
let signature = keypair.sign(message);
assert!(keypair.public_key().verify(message, &signature).is_ok());
```

## Examples

```bash
cargo run --example basic_sign
cargo run --example batch_verify
```

## Testing

```bash
cargo test
cargo bench
```

⚠️ **Warning**: This library has not been audited. Use at your own risk.
