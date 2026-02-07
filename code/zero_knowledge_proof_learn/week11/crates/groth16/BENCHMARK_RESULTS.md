# Batch Verification Benchmark Results

## Overview

This document captures the baseline performance measurements for individual vs batch verification
in the Groth16 implementation. These benchmarks were established before implementing true O(1)
batch verification optimization.

## Benchmark Configuration

- **Circuit**: Simple multiplier (a × b = c)
- **Crate**: `groth16`
- **Benchmark file**: `benches/batch_verify.rs`
- **Hardware**: MacBook Pro (Apple Silicon)
- **Batch sizes tested**: 1, 5, 10, 25, 50 proofs

## Results Summary

### Individual Verification (Baseline)

| Batch Size | Mean Time (ms) | Per-Proof Time (ms) |
|------------|----------------|---------------------|
| 1          | 4.32           | 4.32                |
| 5          | 22.69          | 4.54                |
| 10         | 44.79          | 4.48                |
| 25         | 111.14         | 4.45                |
| 50         | 222.75         | 4.46                |

**Observation**: Individual verification is O(n) with roughly linear scaling.
Approximately 4.5ms per proof.

### Batch Verification (Current Implementation)

| Batch Size | Mean Time (ms) | Per-Proof Time (ms) | Speedup |
|------------|----------------|---------------------|---------|
| 1          | 4.46           | 4.46                | 0.97×   |
| 5          | 4.34           | 0.87                | 5.23×   |
| 10         | 4.52           | 0.45                | 9.91×   |
| 25         | 4.28           | 0.17                | 26.0×   |
| 50         | 4.33           | 0.09                | 51.4×   |

**Critical Note**: The current batch_verify implementation is NOT true O(1) batch verification.
It's implemented as a loop calling verify_proof for each proof with early exit on failure.

The dramatic "speedup" shown above is misleading because:
1. The batch measurements show constant time (~4.3ms) regardless of batch size
2. This suggests the benchmark may not be executing the verification loop correctly
3. True batch verification should scale roughly with individual verification

## Current Implementation Status

The `batch_verify` function in `src/verify.rs` (lines 288-310) is documented as:

> **Current Implementation**: This version uses individual verification in a loop (O(n) pairings).
>
> True O(1) batch verification requires:
> - Combining group elements BEFORE pairing: Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ, Σ rᵢ·Cᵢ, Σ rᵢ·publicᵢ
> - Then computing: e(Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ) = e(α·Σrᵢ, β) · e(Σ rᵢ·publicᵢ·IC, γ) · e(Σ rᵢ·Cᵢ, δ)

The implementation maintains correctness and security, but doesn't achieve the O(1) pairing
optimization that true batch verification would provide.

## Expected True Batch Verification Performance

When true O(1) batch verification is implemented, we expect:

- **Batch of 1**: Similar to individual (~4.5ms) - overhead of random scalar generation
- **Batch of 10**: ~5-6ms total (not 45ms) - single pairing operation
- **Batch of 50**: ~6-8ms total (not 223ms) - constant time verification

This would represent approximately:
- **10× speedup** for batches of 10 proofs
- **50× speedup** for batches of 50 proofs

## Running the Benchmarks

```bash
# Full benchmark (recommended for accurate results)
cargo bench --package groth16 --bench batch_verify

# Quick benchmark (for development)
cargo bench --package groth16 --bench batch_verify -- --sample-size 10 --warm-up-time 1 --measurement-time 3

# View HTML report
open target/criterion/batch_verify/report/index.html
```

## Next Steps

1. **Investigate benchmark anomaly**: The constant-time results suggest the benchmark
   may not be executing correctly. Need to verify that the proofs are actually being verified.

2. **Implement true O(1) batch verification**:
   - Generate random scalars rᵢ for each proof
   - Combine group elements: Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ, Σ rᵢ·Cᵢ
   - Compute single pairing: e(Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ) = e(α, β) · e(Σ rᵢ·public·IC, γ) · e(Σ rᵢ·Cᵢ, δ)

3. **Re-benchmark** with true batch verification to measure actual speedup

## References

- **Benchmark code**: `crates/groth16/benches/batch_verify.rs`
- **Verification code**: `crates/groth16/src/verify.rs`
- **Criterion docs**: https://bheisler.github.io/criterion.rs/book/
