# Week 3 — Signatures + transactions (authorization)
### Role in the whole picture
A blockchain is a state machine driven by *authorized* messages. Signatures answer: “who is allowed to move funds/change state?”

### Learn
- Signing vs hashing (what each guarantees)
- Transaction fields, canonical encoding
- Nonce/replay protection intuition

### Materials
- Transactions (Ethereum docs, conceptual): https://ethereum.org/en/developers/docs/transactions/
- Digital signatures primer: https://cryptobook.nakov.com/digital-signatures
- `ed25519-dalek`: https://docs.rs/ed25519-dalek/

### Coding goals
Create crate `tx-rs`:
- `Transaction { from_pubkey, to_pubkey, amount: u64, nonce: u64 }`
- `TxId = Hash32(sha256(serialize(tx)))`
- `SignedTransaction { tx, sig }`
- `sign(tx, sk) -> sig`
- `verify(signed_tx) -> bool`

### Checks
- Tests:
  - sign then verify passes
  - modifying any field breaks signature
- README: explain nonce and why deterministic encoding matters

### Extra (optional)
- Add “mempool” vector and basic dedup by TxId