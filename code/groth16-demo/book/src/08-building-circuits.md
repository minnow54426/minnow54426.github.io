# Building Your Own Circuits

Congratulations! You've made it through the theory. Now let's put it all together and learn how to build your own zero-knowledge circuits!

## Learning Objectives

After this chapter, you will understand:
- How to design a circuit from a computation
- Best practices for circuit construction
- Common circuit patterns and idioms
- How to test and debug circuits
- Performance considerations

## Motivating Example: From Idea to Circuit

Let's say you want to prove you know a secret password hash without revealing the password.

**Computation**: Hash(password) = claimed_hash

**Circuit design**:
1. Identify inputs: password (private), claimed_hash (public)
2. Define the computation: Hash function
3. Encode as R1CS constraints
4. Generate witness, proof, and verify!

## Circuit Design Process

### Step 1: Identify Inputs and Outputs

First, clearly define what's private and what's public:

**Example**: Age verification
- **Private**: Your age
- **Public**: Threshold (e.g., 18)
- **Output**: Are you over 18? (yes/no)

### Step 2: Break Down the Computation

Complex computations need to be broken into simple operations:

**Supported operations**:
- Addition: `a + b = c`
- Multiplication: `a × b = c`
- Subtraction: `a - b = c` (addition with negative)
- Division: `a / b = c` (multiplication with inverse)

**Not directly supported**:
- Bitwise operations (need to decompose into bits)
- Comparison (need to decompose into bits)
- Range checks (need specialized circuits)

### Step 3: Add Intermediate Variables

For complex computations, introduce intermediate variables:

**Example**: `(a + b) × (a - b) = c`

Variables: `z = [1, c, a, b, t₁, t₂]`

Constraints:
```text
t₁ = a + b
t₂ = a - b
c = t₁ × t₂
```

### Step 4: Encode as R1CS

Convert each operation into an R1CS constraint:

**Addition constraint**: `t₁ = a + b`
```text
A = [0, 0, 1, 1, 0, 0]  ← a + b
B = [1, 0, 0, 0, 0, 0]  ← 1
C = [0, 0, 0, 0, 1, 0]  ← t₁
```

**Multiplication constraint**: `c = t₁ × t₂`
```text
A = [0, 0, 0, 0, 1, 0]  ← t₁
B = [0, 0, 0, 0, 0, 1]  ← t₂
C = [0, 1, 0, 0, 0, 0]  ← c
```

## Circuit Patterns

### Pattern 1: Boolean Circuits

For operations on bits (0 or 1):

**Bit constraint**: `b ∈ {0, 1}`
```text
b × (1 - b) = 0
```

R1CS:
```text
A = [0, 0, 1, 0]  ← b
B = [1, 0, -1, 0] ← 1 - b
C = [0, 1, 0, 0]  ← 0
```

**Bitwise AND**: `c = a AND b`
```text
c = a × b
```

**Bitwise OR**: `c = a OR b`
```text
c = a + b - (a × b)
```

**Bitwise XOR**: `c = a XOR b`
```text
c = a + b - 2(a × b)
```

### Pattern 2: Comparison Circuits

To check if `a > b`:

1. Compute `d = a - b`
2. Decompose `d` into bits
3. Use the most significant bit (sign bit) to determine comparison

### Pattern 3: Range Check Circuits

To check if `x` is in range `[0, 2ⁿ)`:

1. Decompose `x` into `n` bits
2. Each bit must satisfy the bit constraint
3. This ensures `0 ≤ x < 2ⁿ`

### Pattern 4: Equality Check

To check if `a = b`:

1. Compute `d = a - b`
2. Check if `d = 0` using `d × d = 0` (or more sophisticated methods)

## Implementation: Building a Circuit

Let's implement a simple circuit from scratch.

### Example: Cubic Circuit

Circuit: `y = x³`

```rust,ignore
use ark_bn254::Fq;
use groth16_math::fields::FieldWrapper;
use groth16_r1cs::constraint::R1CSConstraint;

pub struct CubicCircuit {
    pub x: u64,
    pub y: u64,
}

impl CubicCircuit {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraints = Vec::new();

        // Constraint 1: t = x × x (square)
        let mut c1 = R1CSConstraint::<Fq>::new();
        c1.add_a_variable(1, FieldWrapper::<Fq>::from(1u64));  // x
        c1.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));  // x
        c1.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));  // t
        constraints.push(c1);

        // Constraint 2: y = t × x (cube)
        let mut c2 = R1CSConstraint::<Fq>::new();
        c2.add_a_variable(2, FieldWrapper::<Fq>::from(1u64));  // t
        c2.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));  // x
        c2.add_c_variable(3, FieldWrapper::<Fq>::from(1u64));  // y
        constraints.push(c2);

        constraints
    }

    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.x),  // x
            FieldWrapper::<Fq>::from(self.x * self.x),  // t = x²
            FieldWrapper::<Fq>::from(self.y),  // y
        ]
    }
}
```

### Example: Multiplier Circuit (Recap)

From `crates/circuits/src/multiplier.rs:45-131`:

```rust,ignore
pub struct MultiplierCircuit {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl MultiplierCircuit {
    pub fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }

    pub fn to_r1cs(&self) -> Vec<R1CSConstraint<Fq>> {
        let mut constraint = R1CSConstraint::<Fq>::new();

        // A vector: selects variable a (index 2)
        constraint.add_a_variable(2, FieldWrapper::<Fq>::from(1u64));

        // B vector: selects variable b (index 3)
        constraint.add_b_variable(3, FieldWrapper::<Fq>::from(1u64));

        // C vector: selects variable c (index 1)
        constraint.add_c_variable(1, FieldWrapper::<Fq>::from(1u64));

        vec![constraint]
    }

    pub fn witness(&self) -> Vec<FieldWrapper<Fq>> {
        vec![
            FieldWrapper::<Fq>::from(1u64),   // constant 1
            FieldWrapper::<Fq>::from(self.c), // public output c
            FieldWrapper::<Fq>::from(self.a), // private input a
            FieldWrapper::<Fq>::from(self.b), // private input b
        ]
    }
}
```

## Testing Your Circuit

Always test your circuit thoroughly!

### Unit Tests

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_valid() {
        let circuit = CubicCircuit::new(3, 27);  // 3³ = 27
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // Verify each constraint
        for constraint in &constraints {
            assert!(constraint.is_satisfied(&witness));
        }
    }

    #[test]
    fn test_circuit_invalid() {
        let circuit = CubicCircuit::new(3, 26);  // 3³ ≠ 26
        let constraints = circuit.to_r1cs();
        let witness = circuit.witness();

        // Should fail at least one constraint
        let all_satisfied = constraints.iter()
            .all(|c| c.is_satisfied(&witness));
        assert!(!all_satisfied);
    }
}
```

### Integration Tests

```rust,ignore
#[test]
fn test_full_groth16_flow() {
    use groth16_groth16::{trusted_setup, generate_proof, verify_proof};
    use groth16_qap::r1cs_to_qap;

    // Create circuit
    let circuit = CubicCircuit::new(3, 27);
    let witness = circuit.witness();
    let constraints = circuit.to_r1cs();

    // Convert to QAP
    let (a_polys, b_polys, c_polys) = r1cs_to_qap(&constraints, witness.len()).unwrap();

    // Trusted setup
    let (pk, vk) = trusted_setup(&a_polys, &b_polys, &c_polys, 1, &mut rng).unwrap();

    // Generate proof
    let proof = generate_proof(&pk, &witness, &a_polys, &b_polys, &c_polys, 1, &mut rng).unwrap();

    // Verify with public input y=27
    let public_inputs = vec![FieldWrapper::<Fq>::from(27u64)];
    let is_valid = verify_proof(&vk, &proof, &public_inputs).unwrap();

    assert!(is_valid);
}
```

## Best Practices

### 1. Minimize Constraints

Fewer constraints = faster proving and smaller keys.

**Bad**:
```text
a + a + a + a + a = b  (5 additions)
```

**Good**:
```text
t = a × 5  (1 multiplication)
t = b
```

### 2. Reuse Intermediate Values

Compute once, use multiple times.

**Bad**:
```text
t₁ = x × x
t₂ = x × x
```

**Good**:
```text
t = x × x
```

### 3. Use Field Arithmetic

Take advantage of field properties:
- Modular arithmetic is automatic
- No overflow concerns
- Use field inverses for division

### 4. Document Your Circuits

Other developers need to understand your circuit:

```rust,ignore
/// Range proof circuit: proves that x is in range [0, 2ⁿ)
///
/// Private inputs:
/// - x: the value to prove is in range
///
/// Public inputs:
/// - n: the bit length
///
/// Constraints:
/// - Decompose x into n bits
/// - Each bit must satisfy b × (1 - b) = 0
```

## Performance Considerations

### Proving Time

Proving time is **O(n)** where n is the number of constraints:

- Small circuits (< 1000 constraints): < 1 second
- Medium circuits (1000-100000): 1-10 seconds
- Large circuits (> 100000): 10+ seconds

**Optimization**: Use parallelization for large circuits.

### Proof Size

Proof size is **constant**: 128 bytes

- Doesn't grow with circuit complexity
- Efficient for transmission and storage

### Verification Time

Verification time is **O(1)**: constant regardless of circuit size

- Usually < 10 milliseconds
- Efficient for on-chain verification

## Common Pitfalls

### 1. Incorrect Witness Order

The witness order matters: `[1, public..., private...]`

**Bad**:
```rust,ignore
vec![
    FieldWrapper::<Fq>::from(self.a),  // Wrong order!
    FieldWrapper::<Fq>::from(self.b),
    FieldWrapper::<Fq>::from(self.c),
]
```

**Good**:
```rust,ignore
vec![
    FieldWrapper::<Fq>::from(1u64),   // Constant first
    FieldWrapper::<Fq>::from(self.c), // Public output
    FieldWrapper::<Fq>::from(self.a), // Private inputs
    FieldWrapper::<Fq>::from(self.b),
]
```

### 2. Forgetting Intermediate Variables

Complex computations need intermediate variables:

**Bad**:
```text
y = (a + b) × (c + d)  // Can't encode directly!
```

**Good**:
```text
t₁ = a + b
t₂ = c + d
y = t₁ × t₂
```

### 3. Not Testing Edge Cases

Test with:
- Zero values
- Maximum values
- Boundary conditions
- Invalid inputs

## Next Steps

Now that you know how to build circuits:

1. **Practice**: Implement your own circuits
2. **Optimize**: Learn circuit optimization techniques
3. **Explore**: Study advanced circuit patterns
4. **Build**: Create real-world applications!

## Exercises

1. **Square circuit**:
   ```rust
   Implement a circuit that computes y = x²
   Test it with various inputs
   ```

2. **Comparison circuit**:
   ```rust
   Implement a circuit that checks if a > b
   Hint: Decompose into bits and compare
   ```

3. **Range proof**:
   ```rust
   Implement a circuit that proves x is in [0, 256)
   Hint: Use 8 bits and bit constraints
   ```

4. **Challenge question**:
   ```text
   How would you implement a hash function circuit?
   What operations do you need?
   ```

## Further Reading

- **Circuit Optimization**: [Bellman Documentation](https://github.com/zkcrypto/bellman)
- **Circuit Patterns**: [ZK-SNARKs Circuit Design](https://media.githubusercontent.com/media/zcash/zcash/master/doc/protocol/sapling.pdf)
- **Advanced Circuits**: [Zcash Orchard Specification](https://zips.z.cash/protocol/protocol.pdf#orchard)

---

**Congratulations! You've completed the Groth16 tutorial!**

Now you're ready to:
- Build your own zero-knowledge proofs
- Contribute to zk-SNARK projects
- Explore advanced protocols (PLONK, Halo 2, etc.)

**Happy proving!** 🎉
