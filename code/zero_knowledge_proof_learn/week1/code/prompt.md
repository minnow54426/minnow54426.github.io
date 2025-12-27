# Week 1 — Rust foundations for crypto & protocol code
### Role in the whole picture
Everything else depends on writing correct Rust that manipulates bytes, errors, and serialization safely.

### Learn
- Ownership/borrowing: enough to write clean APIs without fighting the compiler
- Modules/crates, `pub` visibility, traits (basic)
- Error handling: `Result`, `anyhow`, `thiserror`
- Serialization: deterministic binary encoding (prefer `bincode`)
- Testing: unit tests, table-driven tests

### Materials
- The Rust Book: https://doc.rust-lang.org/book/
  - Focus: Ch 3–5, 7–9, 11
- Rust by Example (spot-check topics): https://doc.rust-lang.org/rust-by-example/
- `anyhow`: https://docs.rs/anyhow/
- `thiserror`: https://docs.rs/thiserror/
- `serde`: https://serde.rs/
- `bincode`: https://docs.rs/bincode/

### Coding goals (deliverables)
Create repo `rust-protocol-basics`:
- `bytes` module:
  - hex encode/decode helpers (`hex` crate OK)
  - `to_bytes()` for your structs via `bincode`
- `hash` module:
  - `sha256(data: &[u8]) -> [u8; 32]`
- `types` module:
  - define `Hash32([u8; 32])` newtype + display in hex

### Checks (done when)
- `cargo test` passes
- `cargo fmt` + `cargo clippy` clean enough (few/no warnings)
- README shows: serialize a struct → hash it → print hash

### Extra (optional, if time)
- Tiny CLI: `hash "hello"` prints SHA-256

Do above tasks in the following order
1. create a rust proj using cargo
2. create examples corresponding above topics
3. write detailed tests and run to make sure all tests can pass
4. write a readme.md to summary
The code and tests shall attached with detailed comments