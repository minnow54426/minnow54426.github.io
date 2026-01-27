//! Artifact structures for ZK-SNARK proofs
//!
//! This module provides data structures for serializing and deserializing
//! ZK-SNARK proof artifacts (witnesses, proofs, public inputs, verification keys)
//! in a standardized JSON format.

use serde::{Deserialize, Serialize};
use crate::error::CircuitType;

/// Common metadata for all artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub circuit_type: CircuitType,
    pub version: String,
    pub timestamp: u64,
    pub description: Option<String>,
}

impl Metadata {
    pub fn new(circuit_type: CircuitType) -> Self {
        Self {
            circuit_type,
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: None,
        }
    }
}

/// Identity circuit witness data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityWitness {
    pub preimage: String,
}

/// Membership circuit witness data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipWitness {
    pub leaf: String,
    pub path: Vec<String>,
    pub path_indices: Vec<bool>,
}

/// Privacy circuit witness data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyWitness {
    pub value: u64,
}

/// Witness data enum for all circuit types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WitnessData {
    Identity(IdentityWitness),
    Membership(MembershipWitness),
    Privacy(PrivacyWitness),
}

/// Witness wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessWrapper {
    pub metadata: Metadata,
    pub witness: WitnessData,
}

/// Public inputs for identity circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityPublicInputs {
    pub hash: String,
}

/// Public inputs for membership circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipPublicInputs {
    pub root: String,
}

/// Public inputs for privacy circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPublicInputs {
    pub min: u64,
    pub max: u64,
}

/// Public inputs data enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PublicInputsData {
    Identity(IdentityPublicInputs),
    Membership(MembershipPublicInputs),
    Privacy(PrivacyPublicInputs),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_metadata_serialization() {
        let meta = Metadata {
            circuit_type: CircuitType::Identity,
            version: "0.1.0".to_string(),
            timestamp: 1706361600,
            description: Some("Test proof".to_string()),
        };

        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("identity"));

        let decoded: Metadata = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.circuit_type, CircuitType::Identity);
    }

    #[test]
    fn test_identity_witness_serialization() {
        let witness = WitnessData::Identity(IdentityWitness {
            preimage: "secret123".to_string(),
        });

        let json = serde_json::to_string(&witness).unwrap();
        assert!(json.contains("Identity"));

        let decoded: WitnessData = serde_json::from_str(&json).unwrap();
        match decoded {
            WitnessData::Identity(data) => {
                assert_eq!(data.preimage, "secret123");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_witness_wrapper_serialization() {
        let wrapper = WitnessWrapper {
            metadata: Metadata {
                circuit_type: CircuitType::Identity,
                version: "0.1.0".to_string(),
                timestamp: 1706361600,
                description: None,
            },
            witness: WitnessData::Identity(IdentityWitness {
                preimage: "secret".to_string(),
            }),
        };

        let json = serde_json::to_string_pretty(&wrapper).unwrap();
        println!("{}", json);

        let decoded: WitnessWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.metadata.circuit_type, CircuitType::Identity);
    }
}
