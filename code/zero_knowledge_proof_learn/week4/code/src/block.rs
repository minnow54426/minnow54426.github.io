use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tx_rs::SignedTransaction;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub prev_hash: [u8; 32],
    pub txs: Vec<SignedTransaction>,
    pub height: u64,
    pub timestamp: u64,
}

impl Block {
    pub fn new(
        prev_hash: [u8; 32],
        txs: Vec<SignedTransaction>,
        height: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            prev_hash,
            txs,
            height,
            timestamp,
        }
    }
}

pub fn block_hash(block: &Block) -> [u8; 32] {
    let serialized = serde_json::to_vec(block).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    let result = hasher.finalize();

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_block_creation() {
        let _key = Keypair::generate(&mut OsRng);

        let block = Block::new(
            [0u8; 32],  // prev_hash
            vec![],     // empty transactions
            1,          // height
            1234567890, // timestamp
        );

        assert_eq!(block.prev_hash, [0u8; 32]);
        assert_eq!(block.txs.len(), 0);
        assert_eq!(block.height, 1);
        assert_eq!(block.timestamp, 1234567890);
    }

    #[test]
    fn test_block_serialization() {
        let block = Block::new([0u8; 32], vec![], 1, 1234567890);
        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"prev_hash\""));
        assert!(json.contains("\"height\":1"));
    }

    #[test]
    fn test_block_hash() {
        let block = Block::new([0u8; 32], vec![], 1, 1234567890);

        let hash1 = block_hash(&block);
        let hash2 = block_hash(&block);

        // Same block should produce same hash
        assert_eq!(hash1, hash2);

        // Different block should produce different hash
        let block2 = Block::new([1u8; 32], vec![], 1, 1234567890);
        let hash3 = block_hash(&block2);
        assert_ne!(hash1, hash3);

        // Hash should be 32 bytes
        assert_eq!(hash1.len(), 32);
    }
}
