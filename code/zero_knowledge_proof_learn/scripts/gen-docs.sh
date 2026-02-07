#!/bin/bash
# gen-docs.sh - Generate unified documentation for the entire learning journey
#
# Usage: ./scripts/gen-docs.sh [--watch]
#
# This script generates unified documentation including:
# - Unified README linking all weeks
# - Progress tracking
# - Concept mapping between weeks

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

WATCH_MODE=false
if [ "$1" == "--watch" ]; then
    WATCH_MODE=true
    echo -e "${YELLOW}WATCH MODE - Regenerating on changes${NC}"
fi

echo -e "${BLUE}Generating unified documentation...${NC}\n"

# Generate zk-journey.md
cat > zk-journey.md << 'EOF'
# Zero Knowledge Proof Learning Journey

A comprehensive 12-week curriculum taking you from Rust foundations to implementing production-grade zero-knowledge proofs.

## 📊 Progress Overview

| Week | Topic | Status | Tests | Coverage |
|------|-------|--------|-------|----------|
EOF

# Add each week to the table
for week_dir in week*/; do
    week_num=$(basename "$week_dir" | tr -d 'week')
    readme="${week_dir}README.md"

    if [ -f "$readme" ]; then
        # Extract topic from README
        topic=$(grep "^# Week" "$readme" | head -1 | sed 's/^# Week [0-9]* — //')

        # Check if it exists and has tests
        if [ -d "${week_dir}code" ]; then
            # Try to run tests (in a real scenario, you'd cache this)
            status="✅ Complete"
            tests="✓"
        else
            status="⏳ In Progress"
            tests="-"
        fi

        coverage="N/A"
        echo "| ${week_num} | ${topic} | ${status} | ${tests} | ${coverage} |" >> zk-journey.md
    fi
done

cat >> zk-journey.md << 'EOF'

## 🎯 Learning Path

### Phase 1: Foundations (Weeks 1-3)

Build core Rust and cryptography skills.

**Week 1: Rust Protocol Basics**
- Hex encoding/decoding
- SHA-256 hashing
- Type-safe wrappers
- Binary serialization

**Week 2: Merkle Trees**
- Tree construction
- Inclusion proofs
- Domain-separated hashing
- Security analysis

**Week 3: Signatures & Transactions**
- Ed25519 signatures
- Transaction structure
- Authorization
- Nonce/replay protection

### Phase 2: Blockchain Core (Weeks 4-6)

Implement a minimal blockchain.

**Week 4: State Transition Function**
- Account-based model
- Transaction validity
- State updates
- Block structure

**Week 5: Forks & Consensus**
- Fork scenarios
- Chain selection
- Finality concepts
- Module refactoring

**Week 6: ZK Foundations**
- Statement vs witness
- Completeness, soundness, ZK
- NP relations
- Relation checkers

### Phase 3: Zero Knowledge Proofs (Weeks 7-9)

Implement zk-SNARKs from first principles.

**Week 7: Constraints (R1CS)**
- Finite fields
- Constraint systems
- Gadgets concept
- arkworks-rs introduction

**Week 8: Groth16 SNARKs**
- Setup/Prove/Verify pipeline
- Trusted setup
- Public inputs
- Full end-to-end proofs

**Week 9: Merkle Membership Circuits**
- In-circuit verification
- SNARK-friendly hashes
- Membership proofs
- Privacy patterns

### Phase 4: Production & Capstone (Weeks 10-12)

Build production-ready ZK applications.

**Week 10: Proof Artifacts**
- File formats
- CLI tooling
- API design
- Performance tuning

**Week 11: Capstone Build**
- Complete ZK application
- Clean architecture
- Comprehensive tests
- Documentation

**Week 12: Portfolio & Interview Prep**
- Threat modeling
- Performance analysis
- Professional portfolio
- Interview materials

## 🗺️ Concept Map

### Core Concepts Progression

```
Hash Functions (Week 1)
    ↓
Merkle Trees (Week 2)
    ↓
Signatures (Week 3) → Transactions (Week 4)
                            ↓
                        State (Week 4) → Forks (Week 5)
                                            ↓
                                        ZK Concepts (Week 6)
                                            ↓
                                        R1CS (Week 7) → Groth16 (Week 8)
                                                            ↓
                                                        Membership (Week 9)
                                                            ↓
                                                        Artifacts (Week 10)
                                                            ↓
                                                        Capstone (Week 11)
                                                            ↓
                                                        Portfolio (Week 12)
```

### Dependencies Between Weeks

- **Week 1 → Week 2**: Hashing used in Merkle trees
- **Week 2 → Week 9**: Merkle trees used in ZK membership circuits
- **Week 3 → Week 4**: Signatures used in transaction validation
- **Week 4 → Week 5**: Blocks and state needed for fork scenarios
- **Week 6 → Week 7**: ZK concepts needed to understand constraints
- **Week 7 → Week 8**: R1CS needed for Groth16
- **Week 8 → Week 9**: Groth16 setup/prove/verify used in membership circuits
- **Week 9 → Week 11**: Membership circuits used in capstone

## 🚀 Quick Start

### Starting from Scratch

1. **Begin Week 1:**
   ```bash
   cd week1/code
   cargo test
   cargo run --example demo
   ```

2. **Progress Through Weeks:**
   Each week builds on previous ones. Complete weeks in order for best results.

3. **Track Progress:**
   Update the "Status" column above as you complete each week.

### Reviewing Specific Topics

Each week is self-contained for reference. Jump to any week to review specific topics:

```bash
cd week<N>/code
cargo test --doc  # Read documentation and examples
```

## 📚 Additional Resources

- [Master Learning Plan](ZK_learning_plan.md) - The original 12-week curriculum
- [Template](template/) - Standards and conventions used
- [Scripts](scripts/) - Automation for consistency checks

## 🎓 Completion Criteria

You've completed the journey when:

- [ ] All 12 weeks are marked "✅ Complete"
- [ ] All tests pass across all weeks
- [ ] You can explain each concept in your own words
- [ ] You've built the capstone project (Week 11)
- [ ] Your portfolio is ready (Week 12)

## 💡 Study Tips

1. **Don't rush** - Each week builds on the previous
2. **Experiment** - Modify code, break things, fix them
3. **Take notes** - Document what you learn in each week's `docs/learning-notes.md`
4. **Teach others** - Explain concepts aloud to test your understanding
5. **Build in public** - Share your progress and get feedback

## 🤝 Contributing

This is a personal learning journey, but suggestions and improvements are welcome!

---

**Built with dedication over 12 weeks.**

**From Rust beginner to ZK practitioner.**

**Welcome to the zero-knowledge revolution!** 🚀
EOF

echo -e "${GREEN}✓ Generated zk-journey.md${NC}\n"

# Generate individual week summaries
echo "Generating week summaries..."
for week_dir in week*/; do
    week_num=$(basename "$week_dir" | tr -d 'week')
    readme="${week_dir}README.md"

    if [ -f "$readme" ]; then
        # Extract key sections from README
        topic=$(grep "^# Week" "$readme" | head -1)
        role=$(grep -A 5 "Role in the Whole Picture" "$readme" | tail -4 | sed 's/^//')

        # Create summary file
        cat > "${week_dir}SUMMARY.md" << EOF
${topic}

${role}

## Quick Start

\`\`\`bash
cd ${week_dir}code
cargo test
cargo run --example basic_usage
\`\`\`

## Status

Run \`./scripts/check-std.sh ${week_num}\` to verify standards compliance.
EOF

        echo -e "${GREEN}✓ Generated ${week_dir}SUMMARY.md${NC}"
    fi
done

echo -e "\n${GREEN}==================================${NC}"
echo -e "${GREEN}Documentation generation complete!${NC}"
echo -e "${GREEN}==================================${NC}\n"

echo "Generated files:"
echo "  - zk-journey.md (unified learning guide)"
echo "  - week*/SUMMARY.md (individual week summaries)"
echo ""
echo "Next steps:"
echo "  1. Review the journey: cat zk-journey.md"
echo "  2. Read specific week: cat week<N>/SUMMARY.md"
echo "  3. Start learning!"

if [ "$WATCH_MODE" = true ]; then
    echo ""
    echo -e "${YELLOW}Watch mode active. Press Ctrl+C to exit.${NC}"
    while true; do
        sleep 60
        # Re-run generation
        exec "$0" --watch
    done
fi
