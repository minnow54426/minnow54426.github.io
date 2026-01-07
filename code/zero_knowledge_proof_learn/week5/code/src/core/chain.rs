//! Blockchain management with fork support
//!
//! This module provides the Blockchain struct which stores all blocks
//! (including forks) and implements the longest-chain fork-choice rule.

use anyhow::Result;
use std::collections::HashMap;

use super::block::Block;
use super::state::State;
use super::types::Hash;

/// Blockchain stores all blocks and tracks the canonical chain tip
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// All blocks indexed by their hash
    blocks: HashMap<Hash, Block>,

    /// Current canonical chain tip (hash of the tip block)
    tip: Option<Hash>,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    /// Create a new empty blockchain
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            tip: None,
        }
    }

    /// Get the number of blocks in storage
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Check if blockchain is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Get the current canonical chain tip
    pub fn get_tip(&self) -> Option<&Hash> {
        self.tip.as_ref()
    }

    /// Get a block by its hash
    pub fn get_block(&self, hash: &Hash) -> Option<&Block> {
        self.blocks.get(hash)
    }

    /// Add a block to the blockchain
    ///
    /// This implements a simple "longest chain" fork-choice rule:
    /// - If the new block extends the current tip and has higher height, update tip
    /// - If heights are equal, keep current tip (first-to-arrive wins)
    /// - All blocks are stored regardless of fork choice
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        let block_hash = block.hash();

        // Store the block
        self.blocks.insert(block_hash, block.clone());

        // Update tip using fork-choice rule
        self.update_tip(&block);

        Ok(())
    }

    /// Update the tip using longest-chain fork-choice rule
    fn update_tip(&mut self, new_block: &Block) {
        let new_block_hash = new_block.hash();

        match self.tip {
            None => {
                // First block - becomes tip
                self.tip = Some(new_block_hash);
            }
            Some(current_tip_hash) => {
                // Get current tip block
                if let Some(current_tip) = self.get_block(&current_tip_hash) {
                    // Compare heights
                    if new_block.height > current_tip.height {
                        // New block is higher - switch to it
                        self.tip = Some(new_block_hash);
                    }
                    // If heights are equal, keep current tip (no reorg)
                }
            }
        }
    }

    /// Get the canonical chain from genesis to tip
    /// Returns blocks in order from genesis to tip
    pub fn get_canonical_chain(&self) -> Vec<Block> {
        let mut chain = Vec::new();

        // Start from tip and work backwards
        let mut current_hash_opt = self.tip.as_ref().map(|h| *h);

        while let Some(current_hash) = current_hash_opt {
            if let Some(block) = self.get_block(&current_hash) {
                chain.push(block.clone());

                // Move to parent
                if block.is_genesis() {
                    break;
                } else {
                    current_hash_opt = Some(block.prev_hash);
                }
            } else {
                break;
            }
        }

        // Reverse to get genesis -> tip order
        chain.reverse();
        chain
    }
}

/// Apply a block to the state (legacy function from Week 4)
///
/// This applies transactions sequentially and updates state atomically.
/// If any transaction fails, the entire block application fails.
pub fn apply_block(state: &mut State, block: &Block) -> Result<()> {
    for signed_tx in &block.txs {
        state.apply_tx(signed_tx)?;
    }
    Ok(())
}

#[cfg(test)]
mod blockchain_tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.len(), 0);
        assert!(blockchain.get_tip().is_none());
    }

    #[test]
    fn test_add_genesis_block() {
        let mut blockchain = Blockchain::new();
        let genesis = Block::new([0u8; 32], vec![], 0, 1234567890);

        let genesis_hash = genesis.hash();
        blockchain.add_block(genesis).unwrap();

        assert_eq!(blockchain.len(), 1);
        assert_eq!(blockchain.get_tip(), Some(&genesis_hash));
    }

    #[test]
    fn test_add_block_creates_fork() {
        let mut blockchain = Blockchain::new();

        // Add genesis
        let genesis = Block::new([0u8; 32], vec![], 0, 1000);
        let genesis_hash = genesis.hash();
        blockchain.add_block(genesis).unwrap();

        // Add block 1a (height 1)
        let block1a = Block::new(genesis_hash, vec![], 1, 2000);
        let hash1a = block1a.hash();
        blockchain.add_block(block1a).unwrap();

        assert_eq!(blockchain.get_tip(), Some(&hash1a));

        // Add block 1b (another block at height 1 - FORK!)
        let block1b = Block::new(genesis_hash, vec![], 1, 2001);
        let hash1b = block1b.hash();
        blockchain.add_block(block1b).unwrap();

        // Both blocks should exist
        assert!(blockchain.get_block(&hash1a).is_some());
        assert!(blockchain.get_block(&hash1b).is_some());

        // Tip should still be 1a (first one wins when heights tie)
        assert_eq!(blockchain.get_tip(), Some(&hash1a));
    }

    #[test]
    fn test_longest_chain_fork_choice() {
        let mut blockchain = Blockchain::new();

        // Genesis -> 1a -> 2a -> 3a (height 3)
        //        ↘ 1b -> 2b     (height 2)

        let genesis = Block::new([0u8; 32], vec![], 0, 1000);
        let genesis_hash = genesis.hash();
        blockchain.add_block(genesis).unwrap();

        // Branch A
        let block1a = Block::new(genesis_hash, vec![], 1, 2000);
        let hash1a = block1a.hash();
        blockchain.add_block(block1a).unwrap();

        let block2a = Block::new(hash1a, vec![], 2, 3000);
        let hash2a = block2a.hash();
        blockchain.add_block(block2a).unwrap();

        let block3a = Block::new(hash2a, vec![], 3, 4000);
        let hash3a = block3a.hash();
        blockchain.add_block(block3a).unwrap();

        assert_eq!(blockchain.get_tip(), Some(&hash3a));

        // Branch B (competitor)
        let block1b = Block::new(genesis_hash, vec![], 1, 2001);
        let hash1b = block1b.hash();
        blockchain.add_block(block1b).unwrap();

        let block2b = Block::new(hash1b, vec![], 2, 3001);
        let hash2b = block2b.hash();
        blockchain.add_block(block2b).unwrap();

        // Tip should still be hash3a (height 3 > height 2)
        assert_eq!(blockchain.get_tip(), Some(&hash3a));

        // Extend branch B to height 4
        let block3b = Block::new(hash2b, vec![], 3, 4001);
        let hash3b = block3b.hash();
        blockchain.add_block(block3b).unwrap();

        // Still 3a (heights tie, first wins)
        assert_eq!(blockchain.get_tip(), Some(&hash3a));

        let block4b = Block::new(hash3b, vec![], 4, 5000);
        let hash4b = block4b.hash();
        blockchain.add_block(block4b).unwrap();

        // Now 4b wins (height 4 > height 3)
        assert_eq!(blockchain.get_tip(), Some(&hash4b));
    }
}
