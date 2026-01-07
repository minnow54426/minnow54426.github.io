# toychain-rs Architecture

## Module Dependency Graph

```
┌─────────────────────────────────────────┐
│              lib.rs                     │
│         (Public API Exports)            │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│           core/mod.rs                   │
└──────────────┬──────────────────────────┘
               │
      ┌────────┴────────┐
      │                 │
      ▼                 ▼
┌──────────┐      ┌──────────┐
│ types.rs │      │ block.rs │
│ (Hash,   │◄─────┤ (Block,  │
│  Height, │      │  hash)   │
│  etc.)   │      └──────────┘
└──────────┘             │
      │                 │
      │                 ▼
      │          ┌──────────┐
      │          │  tx.rs   │
      │          │ (re-exports│
      │          │  tx-rs)  │
      │          └──────────┘
      │
      ▼
┌──────────┐      ┌──────────┐
│ state.rs │◄─────┤ chain.rs │
│(Account, │      │(Blockchain│
│ State)   │─────►│ + Forks) │
└──────────┘      └──────────┘
      ▲                 │
      └─────────────────┘
         (State updates)
```

## Data Flow

### Adding a Block

```
Block
  │
  ├─► block.hash() → Hash
  │
  └─► blockchain.add_block(block)
        │
        ├─► blocks.insert(hash, block)
        │
        └─► update_tip()
              │
              ├─► Compare heights
              │
              └─► Update tip if higher
```

### Fork Resolution

```
Block1a (height 1)
  └─► Tip = 1a

Block1b (height 1)
  └─► Tip = 1a (tie, first wins)

Block2a (height 2, extends 1a)
  └─► Tip = 2a (2 > 1)

Block2b (height 2, extends 1b)
Block3b (height 3, extends 2b)
  └─► Tip = 3b (3 > 2, REORG!)
```

### Canonical chain Reconstruction

```
get_canonical_chain()
  │
  ├─► Start at tip
  │
  ├─► Follow prev_hash backwards
  │     Until genesis (prev_hash == 0)
  │
  └─► Reverse list
        Result: [genesis, ..., tip]
```

## Key Design Decisions

### 1. HashMap Storage
**Decision**: Store all blocks in `HashMap<Hash, Block>`
**Rationale**:
- O(1) lookup by hash
- Keeps fork history
- Simple implementation

### 2. Longest-Chain Rule
**Decision**: Tip = block with maximum height
**Rationale**:
- Standard Bitcoin-like consensus
- Simple to understand
- Deterministic (ties broken by arrival order)

### 3. Separate Tip Tracking
**Decision**: Store `tip: Option<Hash>` separately from blocks
**Rationale**:
- O(1) tip access
- Easy to update
- Clear separation of data vs. metadata

### 4. Lazy Chain Reconstruction
**Decision**: `get_canonical_chain()` builds chain on-demand
**Rationale**:
- No redundant storage
- Always returns current state
- Simple implementation

## Trade-offs

### Simplicity vs. Features
We chose simplicity over production features:
- ❌ No proof-of-work
- ❌ No difficulty adjustment
- ❌ No total difficulty (just height)
- ❌ No uncle blocks
- ✅ Easy to understand
- ✅ Good for learning

### Performance vs. Clarity
We chose code clarity over micro-optimizations:
- ❌ Cloning blocks in `get_canonical_chain()`
- ❌ Multiple hash lookups
- ✅ Readable code
- ✅ Safe Rust (no unsafe)

## Extension Ideas

To make this more production-like:
1. Add proof-of-work validation
2. Track total difficulty instead of height
3. Implement GHOST fork-choice rule
4. Add block validation rules
5. Persist blocks to disk (sled database)
6. Add networking (libp2p)
