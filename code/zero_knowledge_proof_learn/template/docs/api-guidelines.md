# API Documentation Guidelines

All public APIs must follow these documentation standards to ensure consistency and clarity across all weeks.

## Core Principles

1. **Document everything public** - All `pub` items must have rustdoc comments
2. **Examples first** - Lead with runnable examples showing common usage
3. **Explain why, not just what** - Help learners understand the concepts
4. **Cross-reference liberally** - Link to related types, functions, and concepts
5. **Document errors** - Explain what can go wrong and why

## rustdoc Structure

### Functions

```rust
/// Performs [operation] on [input]
///
/// This function [does what] and is used when [use case].
///
/// # Examples
///
/// ```
/// use weekN_topic::function_name;
///
/// let result = function_name(input);
/// assert_eq!(result, expected);
/// ```
///
/// # Inputs
///
/// - `input`: [Explanation of what this parameter represents]
///
/// # Outputs
///
/// Returns [what it returns] on success.
///
/// # Errors
///
/// This function returns an error if:
/// - [Error condition 1]: [Explanation]
/// - [Error condition 2]: [Explanation]
///
/// # Panics
///
/// This function panics if [condition]. (Prefer returning Results over panicking)
///
/// # Context
///
/// [Explain the algorithm or concept briefly - 1-2 sentences]
///
/// # See Also
///
/// - [`related_function`] - [How it relates]
/// - [`RelatedType`] - [How it's used]
pub fn function_name(input: Type) -> Result<OutputType> {
    // Implementation
}
```

### Types (Structs, Enums)

```rust
/// Represents [what this type models]
///
/// This type is used to [purpose] and encapsulates [what it contains].
///
/// # Examples
///
/// ```
/// use weekN_topic::TypeName;
///
/// let instance = TypeName::new(param1, param2)?;
/// ```
///
/// # Fields
///
/// - `field1`: [What it represents and any invariants]
/// - `field2`: [What it represents and any invariants]
///
/// # Invariants
///
/// - [Invariant 1]: [Explanation]
/// - [Invariant 2]: [Explanation]
///
/// # When to Use
///
/// Use this type when [use case]. For [other use case], consider using [`OtherType`].
///
/// # Performance
///
/// [Note any performance characteristics, e.g., "O(n) operations" or "O(1) lookups"]
///
/// # See Also
///
/// - [`related_function`] - [Relationship]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName {
    /// Field documentation
    pub field1: Type1,
    /// Field documentation
    pub field2: Type2,
}
```

### Modules

```rust
//! # Module Name
//!
//! [Brief description of what this module does - 1-2 sentences]
//!
//! ## Overview
//!
//! [Longer explanation of the module's purpose and main functionality]
//!
//! ## Main Types
//!
//! - [`TypeName`]: [What it's for]
//! - [`TypeName2`]: [What it's for]
//!
//! ## Usage
//!
//! ```rust
//! use weekN_topic::module_name;
//!
//! // Example usage
//! ```
//!
//! ## Design Decisions
//!
//! - [Decision 1]: [Why it was made]
//! - [Decision 2]: [Why it was made]
```

## Documentation Checklist

For every public API item, verify:

### Functions
- [ ] One-sentence summary
- [ ] Detailed explanation (if complex)
- [ ] Runnable example (if applicable)
- [ ] All parameters documented
- [ ] Return value explained
- [ ] All error cases listed
- [ ] Panic conditions documented (if any)
- [ ] Cross-references to related items

### Types
- [ ] Purpose explained
- [ ] Usage example
- [ ] All fields documented
- [ ] Invariants explained (if any)
- [ ] Performance notes (if relevant)
- [ ] When to use/when not to use

### Modules
- [ ] Module-level `//!` comment
- [ ] Overview of purpose
- [ ] Main types listed
- [ ] Usage example
- [ ] Design decisions explained

## Examples Quality Standards

All examples must:

1. **Compile** - Run `cargo test --doc` to verify
2. **Be realistic** - Show actual use cases, not contrived ones
3. **Be complete** - Include necessary imports and setup
4. **Be clear** - Prefer simple examples over clever ones

### Good Example

```rust
/// Computes the Merkle root of a set of leaves
///
/// # Examples
///
/// ```
/// use weekN_topic::MerkleTree;
///
/// let leaves = vec![
///     b"alice".to_vec(),
///     b"bob".to_vec(),
///     b"charlie".to_vec(),
/// ];
///
/// let tree = MerkleTree::from_leaves(leaves);
/// let root = tree.root();
///
/// println!("Merkle root: {:x}", root);
/// ```
pub fn compute_root(leaves: Vec<Vec<u8>>) -> Hash32 {
    // Implementation
}
```

### Bad Example

```rust
/// Computes root
pub fn compute_root(l: Vec<Vec<u8>>) -> [u8; 32] {
    // Too terse, no example, unclear parameter names
}
```

## Common Patterns

### Error Handling

```rust
/// # Errors
///
/// Returns [`Error::InvalidInput`] if [condition].
/// Returns [`Error::Io`] if [condition].
```

### Performance Notes

```rust
/// # Performance
///
/// - Time complexity: O(n log n) where n is [what]
/// - Space complexity: O(n)
/// - This function allocates [what]
```

### Safety Notes (if using unsafe)

```rust
/// # Safety
///
/// This function uses unsafe code because [reason].
/// The caller must ensure that [conditions].
```

## Cross-Reference Style

Use these cross-reference patterns:

- **Types**: ``[`TypeName`]``
- **Functions**: ``[`function_name`]``
- **Modules**: ``[`module_name`]``
- **Other crates**: ``[`extern_crate::Item`](https://docs.rs/extern_crate/latest/extern_crate/Item)``

## Running Documentation Tests

```bash
# Build and test documentation
cargo doc --no-deps

# Build and open in browser
cargo doc --no-deps --open

# Test all doctests
cargo test --doc

# Test specific doctest
cargo test --doc TypeName::new
```

## Review Process

Before considering documentation complete:

1. **Run `cargo doc --no-deps`** - Should have no warnings
2. **Run `cargo test --doc`** - All examples must pass
3. **Read the generated docs** - Open in browser and verify clarity
4. **Get a review** - Have someone else read for clarity

## Further Reading

- [Rust Doc Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html)
- [How to Write Documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
- [The Rust Book on Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
