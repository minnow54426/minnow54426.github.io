use tx_rs::SignedTransaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub prev_hash: [u8; 32],
    pub txs: Vec<SignedTransaction>,
    pub height: u64,
    pub timestamp: u64,
}

impl Block {
    pub fn new(prev_hash: [u8; 32], txs: Vec<SignedTransaction>, height: u64, timestamp: u64) -> Self {
        Self {
            prev_hash,
            txs,
            height,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            [0u8; 32], // prev_hash
            vec![],     // empty transactions
            1,         // height
            1234567890, // timestamp
        );

        assert_eq!(block.prev_hash, [0u8; 32]);
        assert_eq!(block.txs.len(), 0);
        assert_eq!(block.height, 1);
        assert_eq!(block.timestamp, 1234567890);
    }
}
