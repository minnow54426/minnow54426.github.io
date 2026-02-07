#!/bin/bash
# check-std.sh - Verify that a week meets all quality standards
#
# Usage: ./scripts/check-std.sh <week_number>
# Example: ./scripts/check-std.sh 1

set -e  # Exit on error

WEEK_NUM=$1
WEEK_DIR="week${WEEK_NUM}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

if [ -z "$WEEK_NUM" ]; then
    echo "Usage: $0 <week_number>"
    echo "Example: $0 1"
    exit 1
fi

if [ ! -d "$WEEK_DIR" ]; then
    echo -e "${RED}❌ Error: Directory $WEEK_DIR does not exist${NC}"
    exit 1
fi

# Find Cargo.toml (handles both week1/code and week1/code/rust-protocol-basics structures)
CODE_DIR=$(find "$WEEK_DIR" -name "Cargo.toml" -type f -exec dirname {} \; | head -1)

if [ -z "$CODE_DIR" ]; then
    echo -e "${RED}❌ Error: Could not find Cargo.toml in $WEEK_DIR${NC}"
    exit 1
fi

echo "Checking Week ${WEEK_NUM} standards..."
echo "=================================="

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

# Function to run a check
run_check() {
    local name=$1
    local command=$2
    local fix_hint=$3

    echo -n "Checking $name... "

    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASS_COUNT++))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}"
        ((FAIL_COUNT++))
        if [ -n "$fix_hint" ]; then
            echo "   💡 Fix: $fix_hint"
        fi
        return 1
    fi
}

# Function to run a warning check (doesn't fail overall)
run_warn_check() {
    local name=$1
    local command=$2
    local hint=$3

    echo -n "Checking $name... "

    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASS_COUNT++))
    else
        echo -e "${YELLOW}⚠️  WARN${NC}"
        ((WARN_COUNT++))
        if [ -n "$hint" ]; then
            echo "   💡 Tip: $hint"
        fi
    fi
}

pushd "$CODE_DIR" > /dev/null

# 1. Code formatting
run_check \
    "Code formatting (cargo fmt)" \
    "cargo fmt --check" \
    "Run: cargo fmt"

# 2. Clippy warnings
run_check \
    "Clippy (zero warnings)" \
    "cargo clippy --all-targets --all-features -- -D warnings" \
    "Fix warnings: cargo clippy --fix"

# 3. Tests pass
run_check \
    "All tests pass" \
    "cargo test" \
    "Check test failures: cargo test -- --nocapture"

# 4. Documentation builds
run_check \
    "Documentation builds" \
    "cargo doc --no-deps" \
    "Fix doc warnings"

# 5. README exists and follows template
# Check multiple locations for README
README_PATH=""
if [ -f "../README.md" ]; then
    README_PATH="../README.md"
elif [ -f "README.md" ]; then
    README_PATH="README.md"
elif [ -f "../../README.md" ]; then
    README_PATH="../../README.md"
fi

if [ -n "$README_PATH" ]; then
    echo -n "Checking README structure... "
    if grep -q "Role in the Whole Picture" "$README_PATH" && \
       grep -q "Learning Objectives" "$README_PATH" && \
       grep -q "Quick Start" "$README_PATH"; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((PASS_COUNT++))
    else
        echo -e "${YELLOW}⚠️  WARN${NC}"
        echo "   💡 README should follow template structure"
        ((WARN_COUNT++))
    fi
else
    echo -e "${YELLOW}⚠️  WARN${NC} README.md not found in standard locations"
    echo "   💡 Add README.md to week directory or code directory"
    ((WARN_COUNT++))
fi

# 6. Check for doctests
run_warn_check \
    "Doctests present" \
    "cargo test --doc 2>&1 | grep -q 'running [1-9]'" \
    "Add doctests to public API"

# 7. Check coverage (optional - requires tarpaulin)
if command -v cargo-tarpaulin &> /dev/null; then
    echo -n "Checking test coverage (target: 80%)... "
    COVERAGE=$(cargo tarpaulin --output-dir /tmp --out Stdout 2>/dev/null || echo "0")

    # Extract coverage percentage (this is simplified)
    if [ "$COVERAGE" != "0" ]; then
        COVERAGE_PERCENT=$(echo "$COVERAGE" | grep -oP '\d+\.\d+' | tail -1)

        if (( $(echo "$COVERAGE_PERCENT >= 80.0" | bc -l) )); then
            echo -e "${GREEN}✅ PASS${NC} (${COVERAGE_PERCENT}%)"
            ((PASS_COUNT++))
        else
            echo -e "${YELLOW}⚠️  WARN${NC} (${COVERAGE_PERCENT}% - target: 80%)"
            echo "   💡 Add more tests to reach 80% coverage"
            ((WARN_COUNT++))
        fi
    else
        echo -e "${YELLOW}⚠️  SKIP${NC} (tarpaulin not working)"
    fi
else
    echo -n "Checking test coverage... "
    echo -e "${YELLOW}⚠️  SKIP${NC} (cargo-tarpaulin not installed)"
    echo "   💡 Install: cargo install cargo-tarpaulin"
fi

# 8. Check Cargo.toml metadata
echo -n "Checking Cargo.toml metadata... "
if grep -q "description = " Cargo.toml && \
   grep -q "license = " Cargo.toml && \
   grep -q "authors = " Cargo.toml; then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASS_COUNT++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}"
    echo "   💡 Ensure description, license, and authors are set"
    ((WARN_COUNT++))
fi

popd > /dev/null

echo ""
echo "=================================="
echo -e "Results: ${GREEN}${PASS_COUNT} passed${NC}, ${YELLOW}${WARN_COUNT} warnings${NC}, ${RED}${FAIL_COUNT} failed${NC}"
echo "=================================="

if [ $FAIL_COUNT -gt 0 ]; then
    echo -e "${RED}❌ Week ${WEEK_NUM} does not meet standards${NC}"
    exit 1
elif [ $WARN_COUNT -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Week ${WEEK_NUM} meets standards with warnings${NC}"
    exit 0
else
    echo -e "${GREEN}✅ Week ${WEEK_NUM} meets all standards!${NC}"
    exit 0
fi
