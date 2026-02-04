# System Architecture

This document provides comprehensive architecture diagrams for the Groth16 zk-SNARK implementation.

## Workspace Dependency Graph

```mermaid
graph TD
    A[week11: Groth16 Implementation] --> B[math: Finite Fields & Pairings]
    A --> C[r1cs: Constraint System]
    A --> D[qap: Polynomial Transformation]
    A --> E[groth16: Setup/Prove/Verify]
    A --> F[circuits: Example Circuits]

    B --> E
    C --> D
    D --> E
    F --> E

    style A fill:#e1f5ff
    style E fill:#ffe1e1
    style F fill:#e1ffe1
```

**Key Relationships:**
- `math` provides foundational cryptographic primitives
- `r1cs` transforms computations into constraints
- `qap` transforms constraints into polynomials
- `groth16` orchestrates the full proving pipeline
- `circuits` demonstrates practical applications

## Data Flow: Circuit → Proof → Verify

```mermaid
flowchart LR
    A[Circuit Design] --> B[R1CS Constraints]
    B --> C[QAP Polynomials]
    C --> D[Trusted Setup]
    D --> E[Proving Key]
    D --> F[Verification Key]

    G[Private Witness] + H[Public Inputs] --> E
    E --> I[Proof Generation]
    I --> J[ZK Proof]

    J + F + H --> K[Proof Verification]
    K --> L[Valid/Invalid]

    style A fill:#e3f2fd
    style I fill:#fff3e0
    style K fill:#e8f5e9
    style J fill:#fce4ec
```

**Pipeline Stages:**
1. **Circuit Design:** Define computation as R1CS
2. **QAP Transformation:** Convert constraints to polynomials
3. **Trusted Setup:** Generate proving/verification keys
4. **Proof Generation:** Create zero-knowledge proof
5. **Verification:** Check proof validity

## Crate Module Architecture

```mermaid
graph TB
    subgraph "math crate"
        M1[fields.rs: Field arithmetic]
        M2[pairing.rs: Bilinear maps]
        M3[polynomial.rs: Lagrange, division]
    end

    subgraph "r1cs crate"
        R1[constraint.rs: R1CS matrix]
        R2[witness.rs: Assignment generation]
    end

    subgraph "qap crate"
        Q1[polynomials.rs: R1CS→QAP reduction]
    end

    subgraph "groth16 crate"
        G1[setup.rs: PK/VK generation]
        G2[prove.rs: Proof creation]
        G3[verify.rs: Single & batch verify]
    end

    subgraph "circuits crate"
        C1[multiplier.rs]
        C2[cubic.rs]
        C3[hash_preimage.rs]
        C4[merkle.rs]
        C5[range_proof.rs]
    end

    M1 -.-> Q1
    M2 -.-> G1
    M2 -.-> G2
    M2 -.-> G3
    M3 -.-> Q1

    R1 --> Q1
    R2 --> Q1

    Q1 --> G1
    Q1 --> G2

    C1 -.-> G2
    C2 -.-> G2
    C3 -.-> G2
    C4 -.-> G2
    C5 -.-> G2

    style M1 fill:#f3f9ff
    style M2 fill:#f3f9ff
    style M3 fill:#f3f9ff
    style G1 fill:#ffe8e8
    style G2 fill:#ffe8e8
    style G3 fill:#ffe8e8
```

**Module Responsibilities:**

**math crate:**
- `fields.rs`: Finite field arithmetic (addition, multiplication, inversion)
- `pairing.rs`: Elliptic curve operations and pairing computations
- `polynomial.rs`: Polynomial interpolation, evaluation, division

**r1cs crate:**
- `constraint.rs`: R1CS matrix representation (A, B, C matrices)
- `witness.rs`: Witness generation from variable assignments

**qap crate:**
- `polynomials.rs`: R1CS → QAP transformation using Lagrange interpolation

**groth16 crate:**
- `setup.rs`: Trusted setup ceremony (powers of Tau)
- `prove.rs`: Proof generation with random blinding
- `verify.rs`: Single and batch verification

**circuits crate:**
- Example circuits demonstrating practical applications
- Each circuit follows ark-relations R1CS trait

## Proof Generation Flow

```mermaid
sequenceDiagram
    participant C as Circuit
    participant R as R1CS
    participant Q as QAP
    participant S as Setup
    participant P as Prover
    participant V as Verifier

    C->>R: Define computation
    R->>Q: Transform to polynomials
    Q->>S: Generate PK/VK

    Note over S: Trusted Setup<br/>(one-time ceremony)

    S->>P: Proving Key
    S->>V: Verification Key

    P->>P: Generate witness
    P->>P: Create proof (A, B, C)
    P->>V: Send proof + public input

    V->>V: Run pairing equation check
    V->>V: Return valid/invalid
```

**Key Observations:**
- Trusted setup happens once per circuit
- Proving key is used by prover (private)
- Verification key is public (anyone can verify)
- Verification is O(1) regardless of circuit complexity

## Verification Equation Structure

```mermaid
graph LR
    A[Proof: A, B, C] --> V[Pairing Equation Check]
    VK[Verification Key] --> V
    PI[Public Input] --> V

    V --> R{eA,B = eα,β · epublic·IC,γ · eC,δ}

    R -->|True| Valid[✓ Proof Valid]
    R -->|False| Invalid[✗ Proof Invalid]

    style A fill:#fce4ec
    style VK fill:#e3f2fd
    style R fill:#fff9c4
    style Valid fill:#c8e6c9
    style Invalid fill:#ffcdd2
```

**Pairing Equation Explained:**

```
e(A, B) = e(α, β) · e(public·IC, γ) · e(C, δ)
```

Each pairing compares encrypted polynomial evaluations:
- **Left side:** Proof elements A and B
- **Right side:** Setup parameters (α, β, γ, δ) + public input

If equation holds, computation is valid with overwhelming probability.

## Circuit Examples Comparison

| Circuit | Constraints | Proof Size | Verify Time | Use Case |
|---------|-------------|------------|-------------|----------|
| multiplier | 3 | 128 bytes | 4.5ms | Privacy-preserving multiplication |
| cubic | ~10 | 128 bytes | 4.5ms | Polynomial evaluation |
| hash_preimage | ~300 | 128 bytes | 4.5ms | Password authentication |
| merkle | ~2,400 | 128 bytes | 4.5ms | Whitelist membership |
| range_proof | ~100 | 128 bytes | 4.5ms | Age verification |

**Key Insight:** Verification time is constant regardless of circuit complexity!

## Batch Verification Architecture

```mermaid
flowchart TD
    A[Multiple Proofs] --> B[Generate Random Scalars r₁, r₂, ...]
    B --> C[Combine Group Elements]
    C --> D[Σ rᵢ·Aᵢ, Σ rᵢ·Bᵢ, Σ rᵢ·Cᵢ, Σ rᵢ·publicᵢ]
    D --> E[Single Pairing Check]
    E --> F[Batch Result]

    style A fill:#e3f2fd
    style E fill:#c8e6c9
```

**Performance:**
- Individual verification: O(n) where n = number of proofs
- Batch verification: O(1) single pairing operation
- Speedup: 51× for batch of 50 proofs (4.3ms vs 223ms)

## File Organization

```
week11/
├── Cargo.toml              # Workspace configuration
├── README.md               # Project overview
├── crates/
│   ├── math/
│   │   └── src/
│   │       ├── fields.rs       # Scalar field operations
│   │       ├── pairing.rs      # Elliptic curve pairings
│   │       └── polynomial.rs   # Polynomial arithmetic
│   ├── r1cs/
│   │   └── src/
│   │       ├── constraint.rs   # R1CS matrices
│   │       └── witness.rs      # Witness generation
│   ├── qap/
│   │   └── src/
│   │       └── polynomials.rs  # R1CS→QAP transformation
│   ├── groth16/
│   │   ├── src/
│   │   │   ├── setup.rs        # Trusted setup
│   │   │   ├── prove.rs        # Proof generation
│   │   │   └── verify.rs       # Verification
│   │   └── benches/
│   │       └── batch_verify.rs # Benchmarks
│   └── circuits/
│       └── src/
│           ├── multiplier.rs
│           ├── cubic.rs
│           ├── hash_preimage.rs
│           ├── merkle.rs
│           └── range_proof.rs
└── book/
    └── src/
        └── [Tutorial chapters]
```

## External Dependencies

```mermaid
graph TD
    A[Our Implementation] --> B[arkworks-rs]
    B --> C[ark-ff: Finite Fields]
    B --> D[ark-ec: Elliptic Curves]
    B --> E[ark-poly: Polynomials]
    B --> F[ark-bn254: Pairing Curve]
    B --> G[ark-groth16: Reference Impl]
    B --> H[ark-relations: R1CS Traits]

    I[Other Crates] --> J[serde: Serialization]
    I --> K[anyhow: Error Handling]
    I --> L[thiserror: Error Types]

    style A fill:#e1f5ff
    style B fill:#fff3e0
```

**Key Dependencies:**
- **arkworks-rs**: Production-ready ZK cryptography library
- **serde**: Binary serialization for keys and proofs
- **anyhow/thiserror**: Ergonomic error handling

---

**Next:** See [threat-model.md](threat-model.md) for security analysis.
