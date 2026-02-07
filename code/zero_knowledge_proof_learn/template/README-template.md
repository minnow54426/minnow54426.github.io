# Week N — [Topic Name]

### 🎯 Role in the Whole Picture
[2-3 sentences explaining how this week fits into the 12-week journey and what problem it solves. Connect to previous weeks and foreshadow future weeks.]

Example: "This week builds on Week N's introduction to X by implementing Y. Understanding Y is essential before we can tackle Z in Week N+2, as it provides the foundation for [specific application]."

### 📚 Learning Objectives

**Concepts to understand:**
- [Concept 1]: [Brief explanation]
- [Concept 2]: [Brief explanation]
- [Concept 3]: [Brief explanation]

**Skills to develop:**
- [Skill 1]: [What you'll be able to do]
- [Skill 2]: [What you'll be able to do]
- [Skill 3]: [What you'll be able to do]

**Prerequisites:**
- Week N-1: [Previous topic]
- External: [Links to background reading if needed]

### 🚀 Quick Start

```bash
# Navigate to this week's code
cd weekN/code

# Run tests to verify setup
cargo test

# Run the example/demo
cargo run --example demo_name

# Run with output for learning
cargo test -- --nocapture
```

**Expected output:** [Brief description of what you should see]

### 📖 Concepts Overview

[Core concept 1 - 2-3 sentences]
[Why it matters - 1-2 sentences]
[Simple example or analogy]

[Core concept 2 - 2-3 sentences]
[Why it matters - 1-2 sentences]
[Simple example or analogy]

[Core concept 3 - 2-3 sentences]
[Why it matters - 1-2 sentences]
[Simple example or analogy]

### 💻 Implementation

**Project structure:**
```
src/
├── lib.rs           # Public API exports
├── types.rs         # Core data structures
├── core.rs          # Main business logic
├── error.rs         # Error types
└── [domain].rs      # Domain-specific modules
```

**Key types:**
- `TypeName`: [Purpose and when to use]
- `TypeName2`: [Purpose and when to use]

**Public API:**
- `function_name(param: Type) -> Result<Type>`: [What it does]
- `function_name2(param: Type) -> Type`: [What it does]

### 🧪 Testing

**Test coverage:** [XX]%

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check coverage (requires tarpaulin)
cargo tarpaulin --output-dir target/coverage --out Html
```

**Test breakdown:**
- Unit tests: [count] tests covering core functionality
- Integration tests: [count] tests for end-to-end workflows
- Doctests: [count] examples in documentation

**Test patterns used:**
- Table-driven tests for multiple cases
- Property-based tests (proptest) for [specific cases]
- Integration tests for realistic workflows

### 📚 Resources

**Primary learning materials:**
- [Resource 1](URL) - [What it covers]
- [Resource 2](URL) - [What it covers]

**Documentation:**
- [Official docs](URL) - API reference
- [Related crate](URL) - Dependency documentation

**Further reading:**
- [Article/Paper](URL) - [Why it's interesting]

### ✅ Completion Checklist

**Code quality:**
- [ ] All tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation builds without warnings (`cargo doc --no-deps`)

**Understanding checks:**
- [ ] Can explain [concept 1] in your own words
- [ ] Can implement [feature] from scratch
- [ ] Can debug issues related to [topic]
- [ ] Passed the integration test scenarios

**Documentation:**
- [ ] README is complete and accurate
- [ ] All public functions have rustdoc comments
- [ ] Examples demonstrate key usage patterns

### 🔗 Next Steps

**Week N+1: [Next Topic]**
This week's [concept] directly enables [next concept]. You'll use [specific thing you built] to [what you'll do next].

**How this connects:**
- This week's [X] → Next week's [Y]
- [Skill learned] → [Application in next week]

**Preparation for next week:**
- Review [specific concept]
- Ensure [specific task] is working
- Think about [preview of next topic]

---

## 🎓 Week Summary

**Time spent:** [X] hours

**Key accomplishments:**
- [ ] [Milestone 1]
- [ ] [Milestone 2]
- [ ] [Milestone 3]

**Challenges faced:**
- [Challenge 1]: [How you overcame it]
- [Challenge 2]: [How you overcame it]

**What I learned:**
- [Insight 1]
- [Insight 2]
- [Insight 3]

**Questions to explore:**
- [Question 1]?
- [Question 2]?

---

## 📝 License

MIT OR Apache-2.0
