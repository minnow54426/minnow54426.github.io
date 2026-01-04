# Type Structure Visualization

## Transaction Type Structure

```
┌─────────────────────────────────────────────────────────┐
│ ed25519_dalek v1.0 API                                 │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Keypair {                                             │
│      public: PublicKey    ← Field name is ".public"    │
│      secret: SecretKey                              │
│  }                                                     │
│                                                         │
│  PublicKey([u8; 32])    ← Does NOT implement Hash     │
│                                                         │
└─────────────────────────────────────────────────────────┘
                          │
                          │ Cannot use as HashMap key
                          ▼
┌─────────────────────────────────────────────────────────┐
│ tx-rs Library (week3)                                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  HashablePublicKey(pub PublicKey)  ← Tuple wrapper     │
│  ├─ Implements Hash trait                              │
│  └─ Implements Eq trait                                 │
│                                                         │
│  Transaction {                                         │
│      from_pubkey: HashablePublicKey  ← Wrapper type    │
│      to_pubkey: HashablePublicKey                     │
│      amount: u64                                       │
│      nonce: u64                                        │
│  }                                                     │
│                                                         │
└─────────────────────────────────────────────────────────┘
                          │
                          │ Access pattern
                          ▼
┌─────────────────────────────────────────────────────────┐
│ Access Patterns                                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. Get public key from Keypair:                        │
│     ✅ let pubkey = keypair.public;                    │
│     ❌ let pubkey = keypair.public_key;  // No field   │
│                                                         │
│  2. Get PublicKey from HashablePublicKey:               │
│     ✅ let pubkey = tx.from_pubkey.0;                  │
│     ❌ let pubkey = tx.from_pubkey;   // Type mismatch │
│                                                         │
│  3. HashablePublicKey → PublicKey conversion:           │
│     Transaction::new() accepts PublicKey                │
│     Converts internally: HashablePublicKey::from()      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Data Flow Example

```
Step 1: Generate Keypair
────────────────────────────────
let alice_key = Keypair::generate(&mut OsRng);
//    alice_key: Keypair
//    └─ .public: PublicKey ← Access via .public

Step 2: Create Transaction
──────────────────────────
let tx = Transaction::new(
    alice_key.public,    // PublicKey
    bob_key.public,      // PublicKey
    100, 0
);
// Internally converts to:
// tx.from_pubkey: HashablePublicKey(alice_key.public)
//                  └──────────────┬──────────────┘
//                         Tuple field .0 is the PublicKey

Step 3: Access Sender's Public Key
──────────────────────────────────
let sender_pubkey: PublicKey = tx.from_pubkey.0;
//                              ^^^ Access tuple field

Step 4: Use with State
──────────────────────
state.get_account(&tx.from_pubkey.0)
//                     ^^^^^^^^^^^^^^^ Must unwrap to PublicKey
```

## Why .0 is Necessary

```
HashablePublicKey is a TUPLE STRUCT:

pub struct HashablePublicKey(pub PublicKey);
                              ^^^^
                              This creates a tuple struct with one field

Access pattern:
    wrapper.0    →    inner value

    HashablePublicKey.0    →    PublicKey
```

## Type Compatibility Matrix

| Operation | Type | Compatible? | Notes |
|-----------|------|-------------|-------|
| `keypair.public` | `PublicKey` | ✅ | ed25519-dalek v1.0 API |
| `keypair.public_key` | N/A | ❌ | Field doesn't exist |
| `tx.from_pubkey` | `HashablePublicKey` | ✅ | Type is wrapper |
| `tx.from_pubkey as PublicKey` | N/A | ❌ | Cannot cast wrapper |
| `tx.from_pubkey.0` | `PublicKey` | ✅ | Tuple field access |
| `HashMap<PublicKey, _>` | N/A | ❌ | PublicKey doesn't implement Hash |
| `HashMap<HashablePublicKey, _>` | N/A | ✅ | Wrapper implements Hash |

## Compilation Error Examples

```rust
// ERROR 1: Type mismatch
let pubkey: PublicKey = tx.from_pubkey;
//    expected struct `tx_rs::HashablePublicKey`
//    found struct `ed25519_dalek::PublicKey`

// ERROR 2: Missing field
let pubkey = keypair.public_key;
//    no field named `public_key` on type `Keypair`

// ERROR 3: Cannot use as HashMap key
let mut map: HashMap<PublicKey, Account> = HashMap::new();
//    the trait `Hash` is not implemented for `PublicKey`
```
