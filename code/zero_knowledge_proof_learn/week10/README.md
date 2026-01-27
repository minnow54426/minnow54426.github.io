# zk-proof-artifacts

CLI tool for generating, serializing, and verifying zero-knowledge proofs with standardized JSON artifact formats.

## Overview

This tool bridges ZK-SNARK proofs with real-world blockchain systems by providing:
- Standardized JSON artifact formats for proofs, keys, and witnesses
- CLI interface for proof generation and verification
- Support for identity, membership, and privacy circuits
- Off-chain prover and on-chain verifier workflows

## Installation

```bash
# Build from source
cargo build --release

# Binary will be at: target/release/zk-artifacts
```

## Quick Demo

### 1. Generate Keys

```bash
zk-artifacts setup --circuit identity --output-dir ./keys
```

Creates:
- `keys/vk.json` - Verifying key
- `keys/pk.json` - Proving key

### 2. Create Witness File

Create `witness.json`:
```json
{
  "metadata": {
    "circuit_type": "identity",
    "version": "0.1.0",
    "timestamp": 1706361600
  },
  "witness": {
    "type": "Identity",
    "preimage": "secret123"
  }
}
```

### 3. Generate Proof

```bash
zk-artifacts prove --witness witness.json --pk keys/pk.json
```

Creates:
- `proof.json` - Generated proof
- `public_inputs.json` - Public inputs for verification

### 4. Verify Proof

```bash
zk-artifacts verify --proof proof.json --vk keys/vk.json --public public_inputs.json
```

Output: `✓ Proof verified`

## Commands

### setup

Generate proving/verifying keys for a circuit.

```bash
zk-artifacts setup --circuit <TYPE> --output-dir <DIR> [--force]
```

- `--circuit`: Circuit type (identity | membership | privacy)
- `--output-dir`: Output directory [default: ./keys]
- `--force`: Overwrite existing keys

### prove

Generate a proof from witness and proving key.

```bash
zk-artifacts prove --witness <FILE> --pk <FILE> [--output <FILE>] [--benchmark]
```

- `--witness`: Input witness JSON file
- `--pk`: Proving key JSON file
- `--output`: Output proof JSON file [default: proof.json]
- `--benchmark`: Show timing information

### verify

Verify a proof against public inputs and verifying key.

```bash
zk-artifacts verify --proof <FILE> --vk <FILE> --public <FILE>
```

- `--proof`: Proof JSON file
- `--vk`: Verifying key JSON file
- `--public`: Public inputs JSON file

## Artifact Formats

All artifacts use a generic wrapper format with metadata:

```json
{
  "metadata": {
    "circuit_type": "identity|membership|privacy",
    "version": "0.1.0",
    "timestamp": 1706361600,
    "description": "Optional description"
  },
  "data": { /* circuit-specific data */ }
}
```

### Witness Formats

**Identity:**
```json
{
  "witness": {
    "type": "Identity",
    "preimage": "secret_value"
  }
}
```

**Membership:**
```json
{
  "witness": {
    "type": "Membership",
    "leaf": "0x1234...",
    "path": ["0xabcd...", "0xef01..."],
    "path_indices": [true, false, true]
  }
}
```

**Privacy:**
```json
{
  "witness": {
    "type": "Privacy",
    "value": 42
  }
}
```

## Development

```bash
# Run tests
cargo test

# Run with clippy
cargo clippy

# Format code
cargo fmt

# Run integration tests (requires Week 8)
cargo test --test integration_test -- --ignored
```

## Architecture

- **Library + CLI pattern**: Core functionality exposed as library for programmatic use
- **Type-safe serialization**: Each circuit has specific witness structure
- **Generic wrapper format**: Unified metadata and versioning
- **Early validation**: Check circuit_type consistency before expensive operations

## Dependencies

- `zk-groth16-snark` - ZK proof library (Week 8)
- `clap` - CLI framework
- `serde/serde_json` - JSON serialization
- `thiserror/anyhow` - Error handling

## Limitations

- Requires Week 8 library for actual proof operations
- JSON serialization of arkworks types pending
- Integration tests marked as ignored until Week 8 connection complete

## Next Steps

1. Complete Week 8 library integration
2. Implement arkworks → JSON serialization
3. Add comprehensive integration tests
4. Add benchmarking output

## License

MIT OR Apache-2.0
