use sha2::{Digest, Sha256};

/// Type alias for 32-byte hash (SHA-256 output)
pub type Hash32 = [u8; 32];

/// Merkle tree implementation with SHA-256
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Hash32,
    leaves: Vec<Hash32>,
    tree_levels: Vec<Vec<Hash32>>,
}

/// Merkle proof structure for inclusion verification
#[derive(Debug, Clone)]
pub struct MerkleProof {
    /// Sibling hashes along the authentication path
    pub siblings: Vec<Hash32>,
    /// Path bits indicating whether the sibling is on the left (false) or right (true)
    pub path_bits: Vec<bool>,
}

impl MerkleTree {
    /// Create a Merkle tree from leaf data
    pub fn from_leaves(leaves: Vec<Vec<u8>>) -> Self {
        if leaves.is_empty() {
            panic!("Cannot create Merkle tree with no leaves");
        }

        // Hash all leaves with domain separator 0x00
        let leaf_hashes: Vec<Hash32> = leaves
            .into_iter()
            .map(|leaf| Self::hash_leaf(&leaf))
            .collect();

        let mut tree_levels = vec![leaf_hashes.clone()];
        let mut current_level = leaf_hashes.clone();

        // Build tree levels until we reach the root
        while current_level.len() > 1 {
            let next_level = Self::build_next_level(&current_level);
            tree_levels.push(next_level.clone());
            current_level = next_level;
        }

        let root = current_level[0];

        MerkleTree {
            root,
            leaves: leaf_hashes,
            tree_levels,
        }
    }

    /// Get the Merkle root
    pub fn root(&self) -> Hash32 {
        self.root
    }

    /// Get the leaf hashes
    pub fn leaves(&self) -> Vec<Hash32> {
        self.leaves.clone()
    }

    /// Generate a Merkle proof for the leaf at the given index
    pub fn prove(&self, index: usize) -> MerkleProof {
        if index >= self.leaves.len() {
            panic!("Index {} out of bounds for {} leaves", index, self.leaves.len());
        }

        let mut siblings = Vec::new();
        let mut path_bits = Vec::new();
        let mut current_index = index;

        // Collect siblings from each level
        for level_idx in 0..self.tree_levels.len() - 1 {
            let level = &self.tree_levels[level_idx];

            // Determine if current node is left or right child
            let is_right_child = current_index % 2 == 1;
            path_bits.push(is_right_child);

            // Get sibling index
            let sibling_index = if is_right_child {
                current_index - 1
            } else {
                // If we're at the last node and it's alone, it's its own sibling
                if current_index == level.len() - 1 && level.len() % 2 == 1 {
                    current_index
                } else {
                    current_index + 1
                }
            };

            siblings.push(level[sibling_index]);

            // Move to parent level
            current_index /= 2;
        }

        MerkleProof {
            siblings,
            path_bits,
        }
    }

    /// Hash a leaf with domain separator 0x00
    pub fn hash_leaf(data: &[u8]) -> Hash32 {
        let mut hasher = Sha256::new();
        hasher.update([0x00]); // Domain separator for leaves
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Hash an internal node with domain separator 0x01
    pub fn hash_node(left: &Hash32, right: &Hash32) -> Hash32 {
        let mut hasher = Sha256::new();
        hasher.update([0x01]); // Domain separator for internal nodes
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().into()
    }

    /// Build the next level of the tree from the current level
    fn build_next_level(current_level: &[Hash32]) -> Vec<Hash32> {
        let mut next_level = Vec::new();

        for chunk in current_level.chunks(2) {
            match chunk {
                [left] => {
                    // Odd number of nodes, duplicate the last one
                    next_level.push(Self::hash_node(left, left));
                }
                [left, right] => {
                    next_level.push(Self::hash_node(left, right));
                }
                _ => unreachable!(),
            }
        }

        next_level
    }
}

/// Verify a Merkle proof
pub fn verify(root: Hash32, leaf: &[u8], proof: MerkleProof) -> bool {
    let mut current = MerkleTree::hash_leaf(leaf);

    for (sibling, is_right_child) in proof.siblings.into_iter().zip(proof.path_bits) {
        if is_right_child {
            current = MerkleTree::hash_node(&sibling, &current);
        } else {
            current = MerkleTree::hash_node(&current, &sibling);
        }
    }

    current == root
}

/// Convenience function for hashing internal nodes
pub fn hash_internal(left: &Hash32, right: &Hash32) -> Hash32 {
    MerkleTree::hash_node(left, right)
}

/// Convenience function for hashing leaves
pub fn hash_leaf(data: &[u8]) -> Hash32 {
    MerkleTree::hash_leaf(data)
}

/// Include the security analysis module
pub mod security;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_root() {
        let leaves = vec![
            b"leaf1".to_vec(),
            b"leaf2".to_vec(),
            b"leaf3".to_vec(),
            b"leaf4".to_vec(),
        ];

        let tree1 = MerkleTree::from_leaves(leaves.clone());
        let tree2 = MerkleTree::from_leaves(leaves);

        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_valid_proof_verifies() {
        let leaves = vec![
            b"apple".to_vec(),
            b"banana".to_vec(),
            b"cherry".to_vec(),
            b"date".to_vec(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();

        // Test proof for each leaf
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.prove(i);
            assert!(verify(root, leaf, proof));
        }
    }

    #[test]
    fn test_tampered_leaf_fails() {
        let leaves = vec![
            b"original".to_vec(),
            b"data".to_vec(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();
        let proof = tree.prove(0);

        // Try to verify with tampered leaf
        let tampered_leaf = b"tampered";
        assert!(!verify(root, tampered_leaf, proof));
    }

    #[test]
    fn test_tampered_proof_fails() {
        let leaves = vec![
            b"test".to_vec(),
            b"data".to_vec(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();
        let mut proof = tree.prove(0);

        // Tamper with the proof by changing a sibling
        proof.siblings[0] = [0u8; 32];

        assert!(!verify(root, &leaves[0], proof));
    }

    #[test]
    fn test_tampered_root_fails() {
        let leaves = vec![
            b"verify".to_vec(),
            b"test".to_vec(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let proof = tree.prove(0);
        let fake_root = [0u8; 32];

        assert!(!verify(fake_root, &leaves[0], proof));
    }

    #[test]
    fn test_odd_number_of_leaves() {
        let leaves = vec![
            b"one".to_vec(),
            b"two".to_vec(),
            b"three".to_vec(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();

        // All leaves should be verifiable
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.prove(i);
            assert!(verify(root, leaf, proof));
        }
    }

    #[test]
    fn test_single_leaf() {
        let leaves = vec![b"single".to_vec()];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();
        let proof = tree.prove(0);

        assert!(verify(root, &leaves[0], proof.clone()));
        assert_eq!(proof.siblings.len(), 0);
        assert_eq!(proof.path_bits.len(), 0);
    }

    #[test]
    fn test_large_tree() {
        let leaves: Vec<Vec<u8>> = (0..1000)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();

        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();

        // Test random leaves
        for i in [0, 123, 456, 999] {
            let proof = tree.prove(i);
            assert!(verify(root, &leaves[i], proof));
        }
    }
}