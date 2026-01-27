//! Serialization structures for ZK-SNARK keys and proofs
//!
//! This module provides wrapper structures for serializing proving keys,
//! verifying keys, and proofs with metadata. These are placeholder structures
//! that will be expanded when integrating with Week 8's Groth16 implementation.

use serde::{Deserialize, Serialize};
use crate::artifacts::Metadata;

/// Proving key wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvingKeyWrapper {
    pub metadata: Metadata,
    // TODO: Add actual key data after integrating with Week 8
    // This will contain the serialized proving key bytes or field elements
    #[serde(skip)]
    pub key_data: Vec<u8>,
}

/// Verifying key wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyingKeyWrapper {
    pub metadata: Metadata,
    // TODO: Add actual key data after integrating with Week 8
    #[serde(skip)]
    pub key_data: Vec<u8>,
}

/// Proof wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofWrapper {
    pub metadata: Metadata,
    // TODO: Add actual proof data after integrating with Week 8
    #[serde(skip)]
    pub proof_data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_wrapper_metadata() {
        let wrapper = ProvingKeyWrapper {
            metadata: crate::artifacts::Metadata::new(
                crate::error::CircuitType::Identity
            ),
            key_data: vec![1, 2, 3],
        };

        assert_eq!(wrapper.metadata.circuit_type, crate::error::CircuitType::Identity);
    }

    #[test]
    fn test_verifying_key_wrapper() {
        let wrapper = VerifyingKeyWrapper {
            metadata: crate::artifacts::Metadata::new(
                crate::error::CircuitType::Membership
            ),
            key_data: vec![4, 5, 6],
        };

        assert_eq!(wrapper.metadata.circuit_type, crate::error::CircuitType::Membership);
    }

    #[test]
    fn test_proof_wrapper() {
        let wrapper = ProofWrapper {
            metadata: crate::artifacts::Metadata::new(
                crate::error::CircuitType::Privacy
            ),
            proof_data: vec![7, 8, 9],
        };

        assert_eq!(wrapper.metadata.circuit_type, crate::error::CircuitType::Privacy);
    }

    #[test]
    fn test_proving_key_serialization() {
        let wrapper = ProvingKeyWrapper {
            metadata: crate::artifacts::Metadata::new(
                crate::error::CircuitType::Identity
            ),
            key_data: vec![1, 2, 3],
        };

        let json = serde_json::to_string(&wrapper).unwrap();
        assert!(json.contains("identity"));

        let decoded: ProvingKeyWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.metadata.circuit_type, crate::error::CircuitType::Identity);
    }

    #[test]
    fn test_verifying_key_serialization() {
        let wrapper = VerifyingKeyWrapper {
            metadata: crate::artifacts::Metadata::new(
                crate::error::CircuitType::Membership
            ),
            key_data: vec![4, 5, 6],
        };

        let json = serde_json::to_string(&wrapper).unwrap();
        assert!(json.contains("membership"));

        let decoded: VerifyingKeyWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.metadata.circuit_type, crate::error::CircuitType::Membership);
    }

    #[test]
    fn test_proof_serialization() {
        let wrapper = ProofWrapper {
            metadata: crate::artifacts::Metadata::new(
                crate::error::CircuitType::Privacy
            ),
            proof_data: vec![7, 8, 9],
        };

        let json = serde_json::to_string(&wrapper).unwrap();
        assert!(json.contains("privacy"));

        let decoded: ProofWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.metadata.circuit_type, crate::error::CircuitType::Privacy);
    }
}
