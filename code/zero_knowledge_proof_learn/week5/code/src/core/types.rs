//! Common types used throughout the blockchain

/// A hash value (32 bytes)
pub type Hash = [u8; 32];

/// Block height (starts at 0 for genesis, 1 for first block, etc.)
pub type Height = u64;

/// Timestamp in seconds since Unix epoch
pub type Timestamp = u64;

/// Account balance
pub type Balance = u64;

/// Transaction nonce (for replay protection)
pub type Nonce = u64;
