# Testing Standards and Patterns

This document defines the testing standards and patterns used across all weeks to ensure code quality and facilitate learning.

## Core Principles

1. **Test what matters** - Focus on public API and critical logic
2. **Be explicit** - Tests should serve as documentation
3. **Test thoroughly** - Aim for 80% coverage minimum
4. **Test realistically** - Use actual use cases, not edge cases only
5. **Test for failure** - Verify error handling works

## Coverage Requirements

```bash
# Install tarpaulin for coverage reports
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --output-dir target/coverage --out Html

# Minimum requirements:
# - 80% line coverage overall
# - 90% coverage on core types and functions
# - 100% coverage on public API functions
```

## Test Organization

### Unit Tests (src/*.rs)

Place unit tests in the same file as the code:

```rust
// In lib.rs or module files
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Test basic usage
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases
    }

    #[test]
    fn test_error_conditions() {
        // Test error handling
    }
}
```

### Integration Tests (tests/)

Create integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use weekN_topic::*;

#[test]
fn test_end_to_end_workflow() {
    // Test realistic usage scenario
    let result = complete_workflow(input);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_components_together() {
    // Test interaction between modules
}
```

### Doctests (Examples in documentation)

Examples in rustdoc comments double as tests:

```rust
/// Creates a new instance
///
/// # Examples
///
/// ```
/// use weekN_topic::TypeName;
///
/// let instance = TypeName::new(42)?;
/// assert_eq!(instance.value(), 42);
/// # Ok::<(), Error>(())
/// ```
pub fn new(value: u32) -> Result<Self> {
    // Implementation
}
```

## Test Patterns

### 1. Table-Driven Tests

Use table-driven tests for multiple test cases:

```rust
#[test]
fn test_function_with_multiple_cases() {
    struct TestCase {
        input: Type,
        expected: Type,
        description: &'static str,
    }

    let cases = vec![
        TestCase {
            input: 1,
            expected: 2,
            description: "simple case",
        },
        TestCase {
            input: 10,
            expected: 20,
            description: "larger value",
        },
        TestCase {
            input: 0,
            expected: 0,
            description: "zero case",
        },
    ];

    for case in cases {
        let result = function_under_test(case.input);
        assert_eq!(
            result, case.expected,
            "{}: input={} expected={}, got={}",
            case.description, case.input, case.expected, result
        );
    }
}
```

### 2. Error Case Tests

Explicitly test all error conditions:

```rust
#[test]
fn test_error_cases() {
    // Test each documented error condition
    assert!(matches!(
        function_that_fails(invalid_input),
        Err(Error::InvalidInput(_))
    ));

    assert!(matches!(
        function_that_fails(bad_input),
        Err(Error::Io(_))
    ));
}
```

### 3. Property-Based Tests (Optional)

Use proptest for property-based testing:

```rust
#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_property(a in any::<u32>(), b in any::<u32>()) {
            // Property that should always hold
            let result = add(a, b);
            assert!(result >= a); // Example: commutative property
        }
    }
}
```

### 4. Benchmark Tests

Use Criterion for performance benchmarks:

```rust
// benches/main_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use weekN_topic::function_to_benchmark;

fn bench_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| {
            function_to_benchmark(black_box(test_input))
        });
    });
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
```

## Testing Checklist

For each module, verify:

### Unit Tests
- [ ] All public functions have tests
- [ ] Happy path tested
- [ ] Error cases tested
- [ ] Edge cases tested (boundary values, empty inputs, etc.)
- [ ] Table-driven tests used for multiple cases
- [ ] Tests have descriptive names

### Integration Tests
- [ ] End-to-end workflow tested
- [ ] Multiple components tested together
- [ ] Realistic usage scenarios covered

### Doctests
- [ ] All public API functions have examples
- [ ] All examples compile and pass
- [ ] Examples demonstrate real usage

### Coverage
- [ ] Overall coverage ≥ 80%
- [ ] Core logic coverage ≥ 90%
- [ ] Public API coverage = 100%

## Running Tests

```bash
# Run all tests
cargo test

# Run with output for debugging
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific file
cargo test --lib path::to::module

# Run integration tests only
cargo test --test integration_test

# Run doctests only
cargo test --doc

# Run tests with output and show test names
cargo test -- --nocapture -- --test-threads=1

# Run benchmarks
cargo bench

# Check coverage
cargo tarpaulin --output-dir target/coverage --out Html
open target/coverage/index.html
```

## Common Testing Pitfalls

### 1. Testing Implementation Details

❌ **Bad:**
```rust
#[test]
fn test_internal_variable() {
    // Tests how it's implemented, not what it does
    assert_eq!(internal_variable, 42);
}
```

✅ **Good:**
```rust
#[test]
fn test_public_api_behavior() {
    // Tests what the user experiences
    let result = public_api(input);
    assert_eq!(result, expected_output);
}
```

### 2. Brittle Tests

❌ **Bad:**
```rust
#[test]
fn test_exact_string() {
    // Fails if any whitespace changes
    assert_eq!(output, "exact string");
}
```

✅ **Good:**
```rust
#[test]
fn test_contains_content() {
    // More flexible
    assert!(output.contains("important content"));
}
```

### 3. Not Testing Errors

❌ **Bad:**
```rust
#[test]
fn test_only_happy_path() {
    let result = function(valid_input);
    assert!(result.is_ok());
}
```

✅ **Good:**
```rust
#[test]
fn test_all_cases() {
    // Test success
    assert!(function(valid_input).is_ok());

    // Test all documented errors
    assert!(matches!(
        function(invalid_input),
        Err(Error::InvalidInput(_))
    ));
}
```

## Test-Driven Development (TDD)

For new features, follow TDD:

1. **Write test first** - Define the desired behavior
2. **Run test** - Watch it fail (red)
3. **Implement** - Write minimum code to pass
4. **Run test** - Watch it pass (green)
5. **Refactor** - Improve code while keeping tests green

## Continuous Integration

Tests should run automatically:

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --verbose
      - run: cargo clippy -- -D warnings
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3
```

## Further Reading

- [The Rust Book on Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust By Example: Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Criterion User Guide](https://bheisler.github.io/criterion.rs/book/index.html)
- [Proptest Documentation](https://altsysrq.github.io/proptest-book/intro.html)
