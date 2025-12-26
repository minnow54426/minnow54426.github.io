//! Security property definitions and verification
//!
//! This module defines the formal security properties that a Merkle tree
//! should satisfy and provides methods to verify them.

use crate::{MerkleTree, Hash32, verify};

/// Core security properties for Merkle trees
#[derive(Debug, Clone)]
pub struct SecurityProperties {
    /// Collision resistance: computationally infeasible to find x≠y with H(x)=H(y)
    pub collision_resistance: bool,
    /// Binding: root commits to specific leaf set
    pub binding_property: bool,
    /// Soundness: valid proof convinces verifier
    pub soundness: bool,
    /// Completeness: legitimate inclusion can be proven
    pub completeness: bool,
    /// Domain separation: leaf vs internal node hashes are distinguishable
    pub domain_separation: bool,
}

impl SecurityProperties {
    /// Create new security properties with default values
    pub fn new() -> Self {
        Self {
            collision_resistance: false,
            binding_property: false,
            soundness: false,
            completeness: false,
            domain_separation: false,
        }
    }

    /// Verify all security properties for a given Merkle tree
    pub fn verify_all(tree: &MerkleTree, leaves: &[Vec<u8>]) -> Self {
        let mut properties = Self::new();

        properties.collision_resistance = Self::verify_collision_resistance(tree, leaves);
        properties.binding_property = Self::verify_binding_property(tree, leaves);
        properties.soundness = Self::verify_soundness(tree, leaves);
        properties.completeness = Self::verify_completeness(tree, leaves);
        properties.domain_separation = Self::verify_domain_separation();

        properties
    }

    /// Verify collision resistance property
    fn verify_collision_resistance(tree: &MerkleTree, leaves: &[Vec<u8>]) -> bool {
        // Basic check: different inputs should produce different outputs
        let root1 = tree.root();

        // Slightly modify input
        let mut modified_leaves = leaves.to_vec();
        if !modified_leaves.is_empty() {
            modified_leaves[0].push(1);
        }

        let tree2 = MerkleTree::from_leaves(modified_leaves);
        let root2 = tree2.root();

        root1 != root2
    }

    /// Verify binding property
    fn verify_binding_property(tree: &MerkleTree, leaves: &[Vec<u8>]) -> bool {
        let root = tree.root();

        // Try to create a different tree with the same root
        // This should be computationally infeasible, so we just do basic checks
        for i in 0..leaves.len().min(5) {
            let mut alt_leaves = leaves.to_vec();
            alt_leaves[i] = format!("modified_{}", i).into_bytes();

            let alt_tree = MerkleTree::from_leaves(alt_leaves);
            if alt_tree.root() == root {
                return false; // Binding property violated
            }
        }

        true
    }

    /// Verify soundness property
    fn verify_soundness(tree: &MerkleTree, leaves: &[Vec<u8>]) -> bool {
        let root = tree.root();

        // Valid proofs should verify
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.prove(i);
            if !verify(root, leaf, proof) {
                return false; // Soundness violated
            }
        }

        true
    }

    /// Verify completeness property
    fn verify_completeness(tree: &MerkleTree, leaves: &[Vec<u8>]) -> bool {
        let root = tree.root();

        // Every legitimate leaf should be provable
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.prove(i);
            if !verify(root, leaf, proof) {
                return false; // Completeness violated
            }
        }

        true
    }

    /// Verify domain separation property
    fn verify_domain_separation() -> bool {
        use crate::{hash_leaf, hash_internal};

        let data = b"test_data";

        // Leaf hash
        let leaf_hash = hash_leaf(data);

        // Internal hash with same data twice
        let internal_hash = hash_internal(&leaf_hash, &leaf_hash);

        // These should be different due to domain separation
        leaf_hash != internal_hash
    }

    /// Get overall security score (0.0 to 1.0)
    pub fn security_score(&self) -> f64 {
        let properties = [
            self.collision_resistance,
            self.binding_property,
            self.soundness,
            self.completeness,
            self.domain_separation,
        ];

        let satisfied = properties.iter().filter(|&&p| p).count();
        satisfied as f64 / properties.len() as f64
    }

    /// Check if all properties are satisfied
    pub fn all_satisfied(&self) -> bool {
        self.collision_resistance
            && self.binding_property
            && self.soundness
            && self.completeness
            && self.domain_separation
    }
}

impl Default for SecurityProperties {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_properties_creation() {
        let props = SecurityProperties::new();
        assert!(!props.collision_resistance);
        assert!(!props.binding_property);
        assert_eq!(props.security_score(), 0.0);
        assert!(!props.all_satisfied());
    }

    #[test]
    fn test_security_properties_verification() {
        let leaves = vec![
            b"leaf1".to_vec(),
            b"leaf2".to_vec(),
            b"leaf3".to_vec(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());
        let props = SecurityProperties::verify_all(&tree, &leaves);

        // Most properties should be satisfied for a correct implementation
        assert!(props.soundness);
        assert!(props.completeness);
        assert!(props.domain_separation);
        assert!(props.security_score() > 0.5);
    }

    #[test]
    fn test_domain_separation() {
        assert!(SecurityProperties::verify_domain_separation());
    }
}