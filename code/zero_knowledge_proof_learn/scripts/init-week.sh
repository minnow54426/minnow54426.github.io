#!/bin/bash
# init-week.sh - Bootstrap a new week from the template
#
# Usage: ./scripts/init-week.sh <week_number> <topic_name>
# Example: ./scripts/init-week.sh 13 "bulletproofs"

set -e

WEEK_NUM=$1
TOPIC_NAME=$2
WEEK_DIR="week${WEEK_NUM}"
CODE_DIR="${WEEK_DIR}/code"

if [ -z "$WEEK_NUM" ] || [ -z "$TOPIC_NAME" ]; then
    echo "Usage: $0 <week_number> <topic_name>"
    echo "Example: $0 13 \"bulletproofs\""
    exit 1
fi

# Convert topic name to format suitable for Cargo.toml
# e.g., "bulletproofs" -> "bulletproofs", "Merkle Trees" -> "merkle-trees"
TOPIC_SLUG=$(echo "$TOPIC_NAME" | tr '[:upper:]' '[:upper:]' | tr ' ' '-' | tr '[:upper:]' '[:lower:]')

echo "Initializing Week ${WEEK_NUM}: ${TOPIC_NAME}"
echo "=================================="

# Create directory structure
echo "Creating directory structure..."
mkdir -p "$CODE_DIR"/{src,bin,examples,tests,benches}
mkdir -p "$WEEK_DIR"/docs

# Copy README template
echo "Creating README from template..."
sed -e "s/Week N/Week ${WEEK_NUM}/g" \
    -e "s/\[Topic Name\]/${TOPIC_NAME}/g" \
    -e "s/\[topic-name\]/${TOPIC_SLUG}/g" \
    template/README-template.md > "$WEEK_DIR/README.md"

# Copy Cargo.toml template
echo "Creating Cargo.toml from template..."
sed -e "s/weekN-topic/week${WEEK_NUM}-${TOPIC_SLUG}/g" \
    -e "s/Week N/Week ${WEEK_NUM}/g" \
    -e "s/\[Brief 1-sentence description\]/${TOPIC_NAME}/g" \
    template/Cargo-template.toml > "$CODE_DIR/Cargo.toml"

# Create src/lib.rs from template
echo "Creating src/lib.rs from template..."
sed -e "s/Week N: \[Topic Name\]/Week ${WEEK_NUM}: ${TOPIC_NAME}/g" \
    -e "s/weekN_topic/week${WEEK_NUM}_${TOPIC_SLUG}/g" \
    -e "s/\[brief description of what this week does\]/Implementation of ${TOPIC_NAME}/g" \
    template/src-template/lib.rs > "$CODE_DIR/src/lib.rs"

# Create placeholder files
echo "Creating placeholder files..."

# prompt.md (week goals and requirements)
cat > "$WEEK_DIR/prompt.md" << EOF
# Week ${WEEK_NUM}: ${TOPIC_NAME}

## Goals

- [ ] Goal 1
- [ ] Goal 2
- [ ] Goal 3

## Requirements

### Concepts to Understand

1. [Concept 1]
2. [Concept 2]
3. [Concept 3]

### Implementation Tasks

- [ ] Task 1
- [ ] Task 2
- [ ] Task 3

### Tests

- [ ] Unit tests for core functionality
- [ ] Integration tests for workflows
- [ ] Doctests for public API

## Resources

- [Resource 1](URL)
- [Resource 2](URL)

## Notes

[Space for notes during implementation]
EOF

# Basic example
cat > "$CODE_DIR/examples/basic_usage.rs" << EOF
//! Basic usage example for Week ${WEEK_NUM}: ${TOPIC_NAME}
//!
//! This example demonstrates the fundamental concepts of ${TOPIC_NAME}.

use week${WEEK_NUM}_${TOPIC_SLUG}::*;

fn main() -> anyhow::Result<()> {
    println!("Week ${WEEK_NUM}: ${TOPIC_NAME} - Basic Usage");
    println!("==================================\\n");

    // TODO: Add example code

    Ok(())
}
EOF

# Integration test placeholder
cat > "$CODE_DIR/tests/integration_test.rs" << EOF
//! Integration tests for Week ${WEEK_NUM}: ${TOPIC_NAME}

use week${WEEK_NUM}_${TOPIC_SLUG}::*;

#[test]
fn test_end_to_end_workflow() {
    // TODO: Add integration test
    // Test realistic usage scenarios
}
EOF

# Benchmark placeholder
cat > "$CODE_DIR/benches/main_bench.rs" << EOF
//! Benchmarks for Week ${WEEK_NUM}: ${TOPIC_NAME}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use week${WEEK_NUM}_${TOPIC_SLUG}::*;

fn bench_function(c: &mut Criterion) {
    // TODO: Add benchmark
    c.bench_function("function_name", |b| {
        b.iter(|| {
            // Function to benchmark
        });
    });
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
EOF

# Learning notes
cat > "$WEEK_DIR/docs/learning-notes.md" << EOF
# Learning Notes: Week ${WEEK_NUM}

## Key Insights

[What you learned]

## Challenges

[What was difficult and how you overcame it]

## Questions to Explore

[What you still want to understand]

## Resources Used

- [Resource](URL) - [Notes]
EOF

echo ""
echo "✅ Week ${WEEK_NUM} initialized successfully!"
echo ""
echo "Next steps:"
echo "1. Review the README: cd ${WEEK_DIR} && vim README.md"
echo "2. Check requirements: cat ${WEEK_DIR}/prompt.md"
echo "3. Start implementing: cd ${CODE_DIR} && vim src/lib.rs"
echo ""
echo "When ready, verify standards:"
echo "  ./scripts/check-std.sh ${WEEK_NUM}"
