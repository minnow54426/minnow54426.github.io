//! Cryptographic module for signing and verifying transactions
//!
//! This module handles the core cryptographic operations needed for
//! blockchain transactions: signing transactions with private keys
//! and verifying signatures using public keys.

use crate::transaction::{Transaction, TxId};
use ed25519_dalek::{Signature, Keypair, Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Errors that can occur during cryptographic operations
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoError {
    /// Invalid signature format
    InvalidSignature,
    /// Signature verification failed
    VerificationFailed,
    /// Transaction malformed for signing
    InvalidTransaction,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidSignature => write!(f, "Invalid signature format"),
            CryptoError::VerificationFailed => write!(f, "Signature verification failed"),
            CryptoError::InvalidTransaction => write!(f, "Invalid transaction for signing"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// A signed transaction containing the transaction data and its signature
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTransaction {
    /// The original transaction
    pub tx: Transaction,
    /// The signature authorizing this transaction
    pub signature: Signature,
    /// Pre-computed transaction ID for efficiency
    pub tx_id: TxId,
}

impl SignedTransaction {
    /// Create a new signed transaction
    ///
    /// This constructor automatically computes the transaction ID.
    pub fn new(tx: Transaction, signature: Signature) -> Self {
        let tx_id = TxId::from_tx(&tx);
        Self { tx, signature, tx_id }
    }

    /// Verify that the signature is valid for this transaction
    ///
    /// Returns `true` if the signature is valid, `false` otherwise
    pub fn verify(&self) -> bool {
        let tx_bytes = self.tx.serialize();

        // Verify that the signature was created by the owner of from_pubkey
        self.tx.from_pubkey.0
            .verify(&tx_bytes, &self.signature)
            .is_ok()
    }

    /// Get the transaction ID
    pub fn id(&self) -> TxId {
        self.tx_id
    }

    /// Get a reference to the inner transaction
    pub fn transaction(&self) -> &Transaction {
        &self.tx
    }

    /// Get a reference to the signature
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

/// Sign a transaction with a secret key
///
/// # Arguments
/// * `tx` - The transaction to sign
/// * `keypair` - The keypair to sign with
///
/// # Returns
/// A signature that can be verified with the corresponding public key
///
/// # Example
/// ```rust
/// use tx_rs::{Transaction, sign};
/// use ed25519_dalek::Keypair;
/// use rand::rngs::OsRng;
///
/// let mut csprng = OsRng;
/// let keypair = Keypair::generate(&mut csprng);
/// // let secret_key = keypair.secret; // Not needed
///
/// let tx = Transaction::new(
///     keypair.public,
///     keypair.public, // Self-transfer for example
///     100,
///     1,
/// );
///
/// let signature = sign(&tx, &keypair);
/// ```
pub fn sign(tx: &Transaction, keypair: &Keypair) -> Signature {
    let tx_bytes = tx.serialize();
    keypair.sign(&tx_bytes)
}

/// Verify a signed transaction
///
/// This is a convenience function that creates a SignedTransaction
/// and verifies it in one step.
///
/// # Arguments
/// * `tx` - The transaction that was supposedly signed
/// * `signature` - The signature to verify
///
/// # Returns
/// `true` if the signature is valid, `false` otherwise
pub fn verify(tx: &Transaction, signature: &Signature) -> bool {
    let tx_bytes = tx.serialize();
    tx.from_pubkey.0.verify(&tx_bytes, signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_sign_and_verify() {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public;

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            public_key,
            recipient_keypair.public,
            100,
            1,
        );

        // Sign the transaction
        let signature = sign(&tx, &keypair);

        // Verify the signature
        assert!(verify(&tx, &signature));
    }

    #[test]
    fn test_signed_transaction() {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public;

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            public_key,
            recipient_keypair.public,
            100,
            1,
        );

        let signature = sign(&tx, &keypair);
        let signed_tx = SignedTransaction::new(tx.clone(), signature);

        assert!(signed_tx.verify());
        assert_eq!(signed_tx.id(), TxId::from_tx(&tx));
    }

    #[test]
    fn test_modified_transaction_fails_verification() {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public;

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let mut tx = Transaction::new(
            public_key,
            recipient_keypair.public,
            100,
            1,
        );

        let signature = sign(&tx, &keypair);

        // Modify the transaction after signing
        tx.amount = 200;

        // Verification should fail
        assert!(!verify(&tx, &signature));
    }

    #[test]
    fn test_wrong_key_fails_verification() {
        let mut csprng = OsRng;
        let keypair1: Keypair = Keypair::generate(&mut csprng);
        let keypair2: Keypair = Keypair::generate(&mut csprng);
        let public_key1 = keypair1.public;

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            public_key1,
            recipient_keypair.public,
            100,
            1,
        );

        // Sign with key1 but try to verify with key2's public key
        let signature = sign(&tx, &keypair1);
        let tx_wrong_sender = Transaction::new(
            keypair2.public, // Wrong public key
            recipient_keypair.public,
            100,
            1,
        );

        // Verification should fail
        assert!(!verify(&tx_wrong_sender, &signature));
    }
}