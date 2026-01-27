# ZK Proof Artifacts - Design Document

**Date:** 2025-01-27
**Week:** 10 - Integration view: where proofs live in blockchain systems
**Status:** Design Approved

## Overview

This project creates a CLI tool (`zk-artifacts`) that bridges zero-knowledge proofs with real-world blockchain systems. It provides standardized file formats, commands, and workflows for generating, serializing, and verifying ZK-SNARK proofs produced by the Week 8 `zk-groth16-snark` library.

**Project Type:** Hybrid - New CLI project that imports Week 8 as a library dependency
**Supported Circuits:** All (identity, membership, privacy)
**Serialization Format:** JSON (all artifacts)

## Architecture

### Project Structure

```
zk-proof-artifacts/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point with subcommands
│   ├── lib.rs            # Library API for programmatic use
│   ├── artifacts.rs      # Proof package format definitions
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── prove.rs      # prove subcommand
│   │   ├── verify.rs     # verify subcommand
│   │   └── setup.rs      # setup subcommand
│   └── serialization.rs  # JSON conversion logic
├── tests/
│   ├── integration_test.rs
│   ├── serialization_test.rs
│   └── cli_tests.sh
└── README.md
```

### Key Design Decisions

1. **Library + CLI Pattern**: Core functionality exposed as library (`src/lib.rs`) for programmatic use, while `src/main.rs` provides CLI interface
2. **Generic Wrapper Format**: All artifacts use JSON with metadata wrapper including `circuit_type`, `version`, `timestamp`
3. **Type-Safe Serialization**: Each circuit has specific witness structure, unified under `WitnessWrapper` enum
4. **Early Validation**: Check circuit_type consistency before expensive operations
5. **Clear Error Messages**: Actionable guidance for every error case

## Artifact Format Definitions

### Generic Metadata Wrapper

All artifacts share common metadata:

```json
{
  "metadata": {
    "circuit_type": "identity|membership|privacy",
    "version": "0.1.0",
    "timestamp": 1706361600,
    "description": "Optional human-readable description"
  },
  "data": { /* circuit-specific data */ }
}
```

### Witness Structure

```json
{
  "metadata": { ... },
  "witness": {
    "Identity": {
      "preimage": "secret_value"
    }
  }
}
```

Or for membership:

```json
{
  "metadata": { ... },
  "witness": {
    "Membership": {
      "leaf": "0x1234...",
      "path": ["0xabcd...", "0xef01..."],
      "path_indices": [true, false, true]
    }
  }
}
```

### Proof Package

A complete proof package is a directory containing:

```
proof_package/
├── public_inputs.json    # Public inputs for verification
├── proof.json            # Generated proof
└── vk.json              # Verifying key
```

Optional (for proving):
```
├── pk.json              # Proving key
└── witness.json         # Witness data
```

## CLI Interface

### Commands

```bash
zk-artifacts <COMMAND>
```

**Available Commands:**
- `setup` - Generate proving/verifying keys for a circuit
- `prove` - Generate a proof from witness and keys
- `verify` - Verify a proof against public inputs and verifying key
- `help` - Print help information

### Setup Command

```bash
zk-artifacts setup --circuit <TYPE> --output-dir <DIR>
```

**Flags:**
- `--circuit <TYPE>` - Circuit type: identity | membership | privacy
- `--output-dir <DIR>` - Output directory for keys [default: ./keys]
- `--force` - Overwrite existing keys

**Output:**
- Creates `vk.json` and `pk.json` in output directory

### Prove Command

```bash
zk-artifacts prove --witness <FILE> --pk <FILE> --output <FILE>
```

**Flags:**
- `--witness <FILE>` - Input witness JSON file
- `--pk <FILE>` - Proving key JSON file
- `--output <FILE>` - Output proof JSON file [default: proof.json]
- `--benchmark` - Show timing information

**Output:**
- Creates `proof.json`
- Prints public inputs to stdout or separate file

### Verify Command

```bash
zk-artifacts verify --proof <FILE> --vk <FILE> --public <FILE>
```

**Flags:**
- `--proof <FILE>` - Proof JSON file
- `--vk <FILE>` - Verifying key JSON file
- `--public <FILE>` - Public inputs JSON file

**Output:**
- Prints "✓ Proof verified" or "✗ Verification failed" with error details

## Data Flow

### Prove Flow

1. Load `witness.json` → Validate circuit_type matches
2. Load `pk.json` → Deserialize using arkworks
3. Call Week 8's `generate_proof()` function
4. Serialize proof to JSON → `proof.json`
5. Extract public inputs → `public_inputs.json`
6. (Optional) Print timing benchmarks

### Verify Flow

1. Load `proof.json` → Validate circuit_type
2. Load `vk.json` → Deserialize using arkworks
3. Load `public_inputs.json` → Parse into circuit-specific struct
4. Call Week 8's `verify_proof()` function
5. Return success/failure with clear error message

## Error Handling

### Error Types

```rust
enum CliError {
    // File I/O errors
    FileNotFound(PathBuf),
    InvalidJson(String),

    // Circuit errors
    CircuitMismatch { expected: CircuitType, found: CircuitType },
    UnsupportedCircuit(CircuitType),

    // Proof errors (from Week 8 library)
    ProveFailed(String),
    VerifyFailed(String),

    // Serialization errors
    SerializationError(String),
}
```

### Error Handling Strategy

- **Early validation**: Check circuit_type consistency across all inputs
- **Clear messages**: Each error provides actionable guidance
- **Graceful degradation**: If benchmarking fails, still complete operation
- **Exit codes**: Standard Unix codes (0 = success, 1 = error)

## Testing Strategy

### Integration Tests

**`tests/end_to_end.rs`:**
- `test_identity_full_flow()` - Setup keys → create witness → prove → verify
- `test_membership_full_flow()` - Setup keys → create Merkle tree → prove → verify
- `test_privacy_full_flow()` - Setup keys → create witness → prove → verify
- Each test includes failure cases (wrong public input, wrong root, etc.)

### Serialization Tests

**`tests/serialization.rs`:**
- Round-trip tests for each witness type
- Invalid circuit_type rejection
- Missing required fields validation

### CLI Tests

**`tests/cli_tests.sh`:**
- Setup command creates files
- Prove command generates output
- Verify command accepts valid proof
- Verify rejects invalid proof
- Error messages are helpful

## Dependencies

```toml
[dependencies]
# ZK library from Week 8
zk-groth16-snark = { path = "../week8/code" }

# CLI framework
clap = { version = "4.4", features = ["derive"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Timing (for benchmarking)
humantime = "2.1"
```

## Implementation Challenges

### 1. Arkworks Serialization to JSON

Arkworks uses custom binary serialization. Solution:
- Extract field elements to hex/decimal string representations
- Implement `serde::Serialize` for proof data structures
- Round-trip: JSON → hex bytes → arkworks types

### 2. Type Dispatch

Match `circuit_type` string to correct Week 8 circuit:

```rust
match circuit_type {
    CircuitType::Identity => run_identity_circuit(...),
    CircuitType::Membership => run_membership_circuit(...),
    CircuitType::Privacy => run_privacy_circuit(...),
}
```

### 3. Generic Public Inputs Extraction

Each circuit returns different public inputs. Solution:
- Use trait or enum to normalize output
- Serialize to JSON with circuit-specific structure

### 4. Benchmarking (Optional Extra)

Simple wall-clock timing:
```rust
let start = Instant::now();
// ... prove operation ...
let duration = start.elapsed();
println!("Proving time: {:?}", duration);
```

## Success Criteria

- ✓ All three circuits generate proofs via CLI
- ✓ Proofs verify correctly
- ✓ JSON files are human-readable
- ✓ README has reproducible demo steps
- ✓ Clean `cargo clippy` (zero warnings)
- ✓ Passing `cargo test` (all tests pass)

## Demo Workflow

```bash
# 1. Generate keys for identity circuit
zk-artifacts setup --circuit identity --output-dir ./keys

# 2. Create witness file
cat > witness.json << EOF
{
  "metadata": {
    "circuit_type": "identity",
    "version": "0.1.0",
    "timestamp": 1706361600
  },
  "witness": {
    "Identity": {
      "preimage": "secret123"
    }
  }
}
EOF

# 3. Generate proof
zk-artifacts prove --witness witness.json --pk keys/pk.json --output proof.json

# 4. Verify proof
zk-artifacts verify --proof proof.json --vk keys/vk.json --public public_inputs.json
```

## Next Steps

1. Create implementation plan with detailed task breakdown
2. Set up git worktree for isolated development
3. Implement core modules (artifacts, serialization)
4. Build CLI subcommands
5. Write comprehensive tests
6. Create README with demo workflow
