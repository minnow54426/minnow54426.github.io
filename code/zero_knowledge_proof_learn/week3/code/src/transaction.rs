//! Transaction module defining core transaction structure and serialization
//!
//! Transactions are the fundamental data structure in blockchain systems.
//! They represent transfer of value from one account to another and must be
//! signed by the sender to be valid.

use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Wrapper around PublicKey that implements Hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashablePublicKey(pub PublicKey);

impl Hash for HashablePublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_bytes().hash(state);
    }
}

impl From<PublicKey> for HashablePublicKey {
    fn from(pubkey: PublicKey) -> Self {
        HashablePublicKey(pubkey)
    }
}

impl From<&PublicKey> for HashablePublicKey {
    fn from(pubkey: &PublicKey) -> Self {
        HashablePublicKey(*pubkey)
    }
}

/// A transaction represents a value transfer between two accounts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Public key of the sender (who signs this transaction)
    pub from_pubkey: HashablePublicKey,
    /// Public key of the recipient
    pub to_pubkey: HashablePublicKey,
    /// Amount to transfer (in smallest currency units)
    pub amount: u64,
    /// Nonce to prevent replay attacks
    pub nonce: u64,
}

/// Transaction identifier - SHA256 hash of serialized transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxId(pub [u8; 32]);

impl Transaction {
    /// Create a new transaction
    pub fn new(
        from_pubkey: PublicKey,
        to_pubkey: PublicKey,
        amount: u64,
        nonce: u64,
    ) -> Self {
        Self {
            from_pubkey: from_pubkey.into(),
            to_pubkey: to_pubkey.into(),
            amount,
            nonce,
        }
    }

    /// Serialize transaction to canonical byte representation
    ///
    /// This is crucial for signature verification - the same transaction
    /// must always serialize to the same bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.from_pubkey.0.as_bytes());
        bytes.extend_from_slice(self.to_pubkey.0.as_bytes());
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes.extend_from_slice(&self.nonce.to_le_bytes());
        bytes
    }

    /// Compute transaction ID by hashing the serialized transaction
    pub fn compute_id(&self) -> TxId {
        let mut hasher = Sha256::new();
        hasher.update(self.serialize());
        TxId(hasher.finalize().into())
    }
}

impl TxId {
    /// Create TxId from transaction
    pub fn from_tx(tx: &Transaction) -> Self {
        tx.compute_id()
    }

    /// Get TxId as hex string for display
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Create TxId from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self, hex::FromHexError> {
        let bytes = hex::decode(hex_str)?;
        if bytes.len() != 32 {
            return Err(hex::FromHexError::InvalidStringLength);
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(TxId(array))
    }
}

impl fmt::Display for TxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_transaction_serialization() {
        let mut csprng = OsRng;
        let keypair1: Keypair = Keypair::generate(&mut csprng);
        let keypair2: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            keypair1.public,
            keypair2.public,
            100,
            1,
        );

        let serialized1 = tx.serialize();
        let serialized2 = tx.serialize();

        assert_eq!(serialized1, serialized2, "Serialization must be deterministic");
    }

    #[test]
    fn test_tx_id_deterministic() {
        let mut csprng = OsRng;
        let keypair1: Keypair = Keypair::generate(&mut csprng);
        let keypair2: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            keypair1.public,
            keypair2.public,
            100,
            1,
        );

        let id1 = tx.compute_id();
        let id2 = tx.compute_id();

        assert_eq!(id1, id2, "TxId must be deterministic");
    }

    #[test]
    fn test_tx_id_unique() {
        let mut csprng = OsRng;
        let keypair1: Keypair = Keypair::generate(&mut csprng);
        let keypair2: Keypair = Keypair::generate(&mut csprng);

        let tx1 = Transaction::new(
            keypair1.public,
            keypair2.public,
            100,
            1,
        );

        let tx2 = Transaction::new(
            keypair1.public,
            keypair2.public,
            200, // Different amount
            1,
        );

        let id1 = tx1.compute_id();
        let id2 = tx2.compute_id();

        assert_ne!(id1, id2, "Different transactions should have different TxIds");
    }
}