# Security Policy

## CRITICAL SECURITY WARNINGS

This implementation of KZG10 requires careful security considerations. **Read this document carefully before using in production.**

## Table of Contents

- [Critical Warnings](#critical-warnings)
- [Trusted Setup Requirements](#trusted-setup-requirements)
- [Production Deployment Checklist](#production-deployment-checklist)
- [Security Best Practices](#security-best-practices)
- [Known Security Considerations](#known-security-ecurity-considerations)
- [Audit Status](#audit-status)
- [Reporting Vulnerabilities](#reporting-vulnerabilities)

## Critical Warnings

### 1. Toxic Waste Problem

The KZG10 scheme requires a **secret value `s`** (called "toxic waste") that:

- **MUST be kept secret** - If `s` is known, an attacker can:
  - Forge arbitrary polynomial commitments
  - Create fake proofs that verify correctly
  - Break the entire security of the KZG10 scheme

- **MUST be destroyed after use** - Even if `s` is used in a ceremony, it must be:
  - Zeroized from memory immediately after computing powers
  - Never written to disk, logs, or crash dumps
  - Never transmitted over any network

- **MUST be generated securely** - The entropy source for `s` must be:
  - Cryptographically secure random number generator
  - Not predictable or reproducible by attackers
  - Sufficiently large (field element, ~254 bits for BLS12-381)

### 2. NEVER Use `setup_for_testing()` in Production

The `setup_for_testing()` function:
- Uses a predictable secret (dangerous!)
- Is **ONLY for testing and development**
- Is feature-gated behind `test-only` feature
- Must **NEVER** be used with real data or values

```rust
// ❌ NEVER DO THIS IN PRODUCTION
let srs = SRS::<Bls12_381>::setup_for_testing(degree, &mut rng);

// ✅ INSTEAD: Use a trusted ceremony output
let srs = SRS::<Bls12_381>::from_powers_of_tau(
    &g1_powers,  // From verified ceremony
    h,
    s_h,
)?;
```

## Trusted Setup Requirements

### Multi-Party Computation (MPC) Ceremony

In production, you **MUST** use an MPC ceremony where:

1. **Multiple participants contribute randomness**
   - Each participant generates random `sᵢ`
   - Final secret: `s = s₁ ⊕ s₂ ⊕ ... ⊕ sₙ`
   - No single participant knows final `s`

2. **Each participant verifies the previous contribution**
   - Prevents malicious contributions
   - Ensures cryptographic consistency

3. **The ceremony is publicly verifiable**
   - All contributions are recorded
   - Anyone can verify the transcript
   - Transparent and auditable

### Trusted Ceremony Outputs

Use established, audited ceremony outputs:

#### Ethereum's KZG Ceremony (Proto-Danksharding)
- **Purpose**: EIP-4844 data availability sampling
- **Participants**: Thousands of contributors
- **Degree**: Supports up to 2²⁴ (16M)
- **Verification**: Publicly verifiable transcript
- **Download**: https://github.com/ethereum/consensus-specs/tree/dev/specs/eip4844

#### Celo's Ceremony
- **Purpose**: Celo blockchain light clients
- **Participants**: Multiple organizations
- **Verification**: Publicly auditable

#### Perpetual Powers of Tau
- **Purpose**: General-purpose ZK-SNARK setup
- **Participants**: Open community ceremony
- **Degree**: Supports very high degrees
- **Download**: https://github.com/protocol/PowersOfTau

### Verifying Ceremony Transcripts

Before using any ceremony output, verify:

```rust
impl<E: Pairing> SRS<E> {
    /// Verify that the SRS is well-formed
    fn verify_powers_of_tau(&self) -> Result<bool> {
        // 1. Check degree bounds
        // 2. Verify pairing checks: e(sⁱ·G₁, H₂) = e(G₁, sⁱ·H₂)
        // 3. Ensure G₁ and G₂ are valid curve points
        // 4. Check sⁱ are correctly computed
    }
}
```

Verification steps:
1. Download ceremony transcript from official source
2. Verify digital signatures on transcript
3. Run `verify_powers_of_tau()` to ensure consistency
4. Check degree is sufficient for your use case
5. Store SRS securely (with integrity checks)

## Production Deployment Checklist

Before deploying to production, ensure:

### Phase 1: Setup (Before Deployment)

- [ ] **Source SRS from trusted ceremony**
  - Never generate your own SRS
  - Use Ethereum's KZG ceremony or similar
  - Download from official sources only

- [ ] **Verify ceremony transcript**
  - Run `verify_powers_of_tau()`
  - Check digital signatures
  - Verify degree bounds

- [ ] **Enable secure features**
  ```toml
  [dependencies]
  kzg10 = { version = "0.1", features = ["secure", "serde"] }
  ```

- [ ] **Review this security policy**
  - Understand all security considerations
  - Review known limitations
  - Plan for incident response

### Phase 2: Implementation (During Development)

- [ ] **Use constant-time operations**
  - Field arithmetic is constant-time (ark-ff)
  - Pairing operations are constant-time (ark-ec)
  - No timing leaks in verification

- [ ] **Enable zeroization**
  ```rust
  // With 'secure' feature enabled
  let srs = SRS::<Bls12_381>::from_powers_of_tau(...)?;
  // Secrets are auto-zeroized when dropped
  ```

- [ ] **Validate all inputs**
  - Check polynomial degree ≤ SRS max_degree
  - Verify points are in the field
  - Ensure proofs are valid curve points

- [ ] **Use secure random number generation**
  - For batch verification weights
  - For any non-deterministic operations
  - Use `ark_std::test_rng()` or `rand::rngs::OsRng`

- [ ] **Implement error handling**
  - Never ignore `Error` variants
  - Log security-relevant errors
  - Fail securely (don't reveal secrets)

### Phase 3: Testing (Before Release)

- [ ] **Run comprehensive tests**
  ```bash
  cargo test --all-features
  ```

- [ ] **Run property-based tests**
  ```bash
  cargo test --test properties
  ```

- [ ] **Compare against reference implementation**
  ```bash
  cargo test --test cross_impl
  ```

- [ ] **Fuzz critical paths**
  - Input validation
  - Proof verification
  - SRS loading

- [ ] **Benchmark performance**
  ```bash
  cargo bench --all-features
  ```
  Ensure verification time meets requirements

### Phase 4: Monitoring (After Deployment)

- [ ] **Log security events**
  - Failed proof verifications
  - SRS validation failures
  - Unexpected errors

- [ ] **Monitor for attacks**
  - Unusual verification patterns
  - Invalid proof spam
  - Timing anomalies

- [ ] **Plan incident response**
  - What if a proof is forged?
  - What if SRS is compromised?
  - How to rotate SRS?

- [ ] **Regular security updates**
  - Update dependencies: `cargo update`
  - Monitor security advisories
  - Re-audit code periodically

## Security Best Practices

### 1. Input Validation

Always validate inputs:

```rust
// Validate polynomial degree
if poly.degree() > srs.max_degree() {
    return Err(Error::DegreeTooLarge {
        max: srs.max_degree(),
        requested: poly.degree(),
    });
}

// Validate field elements
if !point.is_valid() {
    return Err(Error::InvalidFieldElement);
}
```

### 2. Error Handling

Never reveal sensitive information in errors:

```rust
// ❌ BAD: reveals internal state
return Err(Error::InternalError(format!("s = {:?}", secret_s)));

// ✅ GOOD: generic error message
return Err(Error::VerificationFailed);
```

### 3. Constant-Time Operations

Ensure verification is constant-time:

```rust
// Use Subtle or constant-time comparisons
use subtle::ConstantTimeEq;

let is_valid = commitment.verify::<Bls12_381>(&opening, &srs);
// verify() uses constant-time pairing operations
```

### 4. Memory Safety

Enable zeroization:

```toml
[features]
secure = ["zeroize"]  # Auto-zeroize sensitive data
```

This prevents memory disclosure attacks:
- Core dumps
- Memory inspection
- Cache attacks

### 5. Network Security

If transmitting commitments/proofs:

- Use TLS for network transmission
- Verify peer certificates
- Implement replay protection
- Add rate limiting

```rust
// Serialize with serde
let proof_bytes = serde_json::to_vec(&proof)?;

// Transmit over TLS
tls_stream.write_all(&proof_bytes)?;
```

### 6. Storage Security

When storing commitments:

- Encrypt at rest
- Use authenticated encryption (AES-GCM)
- Store SRS separately with access controls
- Implement integrity checks (HMAC, signatures)

```rust
// Encrypt before storage
let encrypted = encrypt(&commitment, key)?;
storage.write(&encrypted)?;
```

## Known Security Considerations

### Cryptographic Assumptions

KZG10 security relies on:

1. **Bounded Strong Diffie-Hellman (BSDH)**
   - Attacker cannot distinguish `e(g, g)^(s^q)` from random
   - Where `s` is the toxic waste
   - Security level: ~128 bits for BLS12-381

2. **Discrete Logarithm Problem (DLP)**
   - Intractable to recover `s` from `s·G`
   - For BLS12-381: ~256-bit security

3. **Bilinear Pairing Security**
   - No efficient pairing inversion
   - Target group security: ~128 bits

If any of these assumptions break, KZG10 is broken.

### Known Limitations

1. **Trusted Setup Required**
   - Single point of failure (if `s` is compromised)
   - Cannot be fully transparent
   - Requires periodic ceremonies (for long-term security)

2. **No Quantum Resistance**
   - Vulnerable to quantum algorithms (Shor's algorithm)
   - Consider post-quantum alternatives for long-term security

3. **Proof Size**
   - Constant size (48 bytes) is good
   - But not as small as some newer schemes (e.g., FRI, Bulletproofs)

4. **Verification Time**
   - Requires pairing operation (~1ms)
   - Slower than some schemes (e.g., Inner Product Arguments)

### Side-Channel Attacks

Mitigated but be aware of:

- **Timing attacks**: Constant-time operations mitigate
- **Cache attacks**: Use constant-time memory access
- **Power analysis**: Relevant for embedded devices
- **Memory disclosure**: Zeroization mitigates

## Audit Status

### Current Audit Status

**NOT AUDITED**

This implementation has NOT been professionally audited. Use at your own risk.

### Recommended Audit Checklist

For production use, ensure:

1. **Code Audit**
   - [ ] Professional cryptography firm review
   - [ ] Mathematical proof verification
   - [ ] Side-channel analysis
   - [ ] Implementation correctness

2. **Formal Verification**
   - [ ] Verify protocol correctness
   - [ ] Prove security properties
   - [ ] Verify constant-time properties

3. **Penetration Testing**
   - [ ] Fuzz testing of critical paths
   - [ ] Adversarial testing
   - [ ] Implementation attacks

4. **Comparison with Reference**
   - [ ] Test vectors from reference implementation
   - [ ] Cross-implementation verification
   - [ ] Mathematical consistency checks

### Reference Implementations

Compare against:

- [ark-poly-commit](https://github.com/arkworks-rs/poly-commit)
- [c-kzg-4844](https://github.com/ethereum/c-kzg-4844)
- [constantine/kzg](https://github.com/mratsim/constantine) (Nim implementation)

## Reporting Vulnerabilities

If you discover a security vulnerability:

### Responsible Disclosure

1. **DO NOT create a public issue**
2. Email: security@example.com
3. Include:
   - Description of vulnerability
   - Steps to reproduce
   - Impact assessment
   - Suggested fix (if any)

### Response Timeline

- **Initial response**: Within 48 hours
- **Assessment**: Within 1 week
- **Fix**: Within 2-4 weeks (depending on severity)
- **Public disclosure**: After fix is deployed

### Security Researchers

We welcome responsible security research. Please:

- Use responsible disclosure
- Provide sufficient detail
- Allow reasonable time for fix
- Follow ethical guidelines

## Security Updates

### Staying Informed

- Watch this repository for security advisories
- Subscribe to security announcements
- Monitor dependency updates
- Follow best practices

### Update Procedure

When security update is released:

1. Review security advisory
2. Update to patched version
3. Run tests: `cargo test --all-features`
4. Deploy to staging first
5. Monitor for issues
6. Deploy to production

### Vulnerability Response

If a vulnerability is discovered in this library:

1. **Immediate**: Disclosure to users
2. **Within 24 hours**: Preliminary assessment
3. **Within 1 week**: Patch released
4. **Within 2 weeks**: Full disclosure

## Best Practices Summary

### DO ✓

- Use trusted ceremony SRS in production
- Enable `secure` feature
- Verify all inputs
- Use constant-time operations
- Log security events
- Plan incident response
- Update dependencies regularly
- Follow security best practices

### DON'T ✗

- NEVER use `setup_for_testing()` in production
- NEVER generate your own SRS for production
- NEVER trust unverified ceremony outputs
- NEVER reveal secrets in error messages
- NEVER skip input validation
- NEVER disable security features for convenience
- NEVER use outdated dependencies
- NEVER deploy without testing

## Additional Resources

### Reading

- [KZG10 Security Analysis](https://eprint.iacr.org/2020/1616)
- [Trusted Setup Ceremonies](https://ethresear.ch/t/trusted-setup-ceremonies/7320)
- [BLS12-381 Security](https://electriccoin.co/blog/new-snark-curve/)

### Tools

- [arkworks-rs](https://github.com/arkworks-rs)
- [constantine](https://github.com/mratsim/constantine)
- [blst](https://github.com/supranational/blst)

### Communities

- [ZKProof](https://zkproof.org/)
- [Applied zkP](https://aopvc.oasislabs.io/)
- [Ethereum Research](https://ethresear.ch/)

---

**Remember**: Cryptography is hard. When in doubt, consult with cryptography experts and security professionals.
