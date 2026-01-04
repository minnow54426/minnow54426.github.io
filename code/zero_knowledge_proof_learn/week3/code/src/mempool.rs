//! Mempool module for managing pending transactions
//!
//! A mempool (memory pool) holds transactions that have been signed
//! but not yet included in a block. This module provides:
//! - Transaction deduplication by TxId
//! - Nonce tracking for replay protection
//! - Signature validation
//! - Basic transaction ordering

use crate::crypto::SignedTransaction;
use crate::transaction::{TxId, HashablePublicKey};
use ed25519_dalek::PublicKey;
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur during mempool operations
#[derive(Debug, Clone, PartialEq)]
pub enum MempoolError {
    /// Transaction signature is invalid
    InvalidSignature,
    /// Transaction nonce is too low (replay protection)
    InvalidNonce,
    /// Transaction already exists in mempool
    DuplicateTransaction,
    /// Transaction is malformed
    InvalidTransaction,
}

impl fmt::Display for MempoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MempoolError::InvalidSignature => write!(f, "Invalid transaction signature"),
            MempoolError::InvalidNonce => write!(f, "Invalid transaction nonce"),
            MempoolError::DuplicateTransaction => write!(f, "Transaction already exists"),
            MempoolError::InvalidTransaction => write!(f, "Invalid transaction format"),
        }
    }
}

impl std::error::Error for MempoolError {}

/// A mempool for managing pending transactions
#[derive(Debug, Default)]
pub struct Mempool {
    /// All transactions in the mempool, indexed by TxId
    transactions: HashMap<TxId, SignedTransaction>,
    /// Track the highest nonce seen for each account (for replay protection)
    nonce_tracker: HashMap<HashablePublicKey, u64>,
}

impl Mempool {
    /// Create a new empty mempool
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a signed transaction to the mempool
    ///
    /// This method performs several validation checks:
    /// 1. Verifies the transaction signature
    /// 2. Checks that the nonce is higher than any previously seen nonce for this sender
    /// 3. Ensures the transaction isn't already in the mempool
    ///
    /// # Arguments
    /// * `signed_tx` - The signed transaction to add
    ///
    /// # Returns
    /// `Ok(())` if the transaction was added successfully
    /// `Err(MempoolError)` if validation failed
    ///
    /// # Example
    /// ```rust
    /// use tx_rs::{Transaction, sign, Mempool, SignedTransaction};
    /// use ed25519_dalek::Keypair;
    /// use rand::rngs::OsRng;
    ///
    /// let mut csprng = OsRng;
    /// let keypair = Keypair::generate(&mut csprng);
    ///
    /// let tx = Transaction::new(
    ///     keypair.public,
    ///     keypair.public,
    ///     100,
    ///     1,
    /// );
    ///
    /// let signature = sign(&tx, &keypair);
    /// let signed_tx = SignedTransaction::new(tx, signature);
    ///
    /// let mut mempool = Mempool::new();
    /// mempool.add_transaction(signed_tx).expect("Failed to add transaction");
    /// ```
    pub fn add_transaction(&mut self, signed_tx: SignedTransaction) -> Result<(), MempoolError> {
        // 1. Verify signature
        if !signed_tx.verify() {
            return Err(MempoolError::InvalidSignature);
        }

        // 2. Check nonce (replay protection)
        let current_nonce = self.nonce_tracker
            .get(&signed_tx.tx.from_pubkey)
            .copied()
            .unwrap_or(0);

        if signed_tx.tx.nonce <= current_nonce {
            return Err(MempoolError::InvalidNonce);
        }

        // 3. Check for duplicates
        if self.transactions.contains_key(&signed_tx.tx_id) {
            return Err(MempoolError::DuplicateTransaction);
        }

        // 4. Add transaction and update nonce tracker
        self.transactions.insert(signed_tx.tx_id, signed_tx.clone());
        self.nonce_tracker.insert(signed_tx.tx.from_pubkey, signed_tx.tx.nonce);

        Ok(())
    }

    /// Remove a transaction from the mempool (typically after it's included in a block)
    ///
    /// # Arguments
    /// * `tx_id` - The ID of the transaction to remove
    ///
    /// # Returns
    /// The removed transaction if it existed, `None` otherwise
    pub fn remove_transaction(&mut self, tx_id: &TxId) -> Option<SignedTransaction> {
        self.transactions.remove(tx_id)
    }

    /// Get a transaction by ID
    ///
    /// # Arguments
    /// * `tx_id` - The transaction ID to look up
    ///
    /// # Returns
    /// A reference to the transaction if it exists, `None` otherwise
    pub fn get_transaction(&self, tx_id: &TxId) -> Option<&SignedTransaction> {
        self.transactions.get(tx_id)
    }

    /// Get all transactions in the mempool
    ///
    /// # Returns
    /// An iterator over all transactions in the mempool
    pub fn transactions(&self) -> impl Iterator<Item = &SignedTransaction> {
        self.transactions.values()
    }

    /// Get the number of transactions in the mempool
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Check if the mempool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Get the highest nonce seen for a specific account
    ///
    /// # Arguments
    /// * `pubkey` - The public key of the account
    ///
    /// # Returns
    /// The highest nonce seen, or 0 if no transactions from this account are in the mempool
    pub fn get_account_nonce(&self, pubkey: &PublicKey) -> u64 {
        let hashable_pubkey: HashablePublicKey = (*pubkey).into();
        self.nonce_tracker.get(&hashable_pubkey).copied().unwrap_or(0)
    }

    /// Get all transaction IDs in the mempool
    pub fn transaction_ids(&self) -> impl Iterator<Item = &TxId> {
        self.transactions.keys()
    }

    /// Clear all transactions from the mempool
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.nonce_tracker.clear();
    }

    /// Get transactions for a specific sender
    ///
    /// # Arguments
    /// * `pubkey` - The public key of the sender
    ///
    /// # Returns
    /// A vector of all transactions from this sender
    pub fn transactions_by_sender(&self, pubkey: &PublicKey) -> Vec<&SignedTransaction> {
        let hashable_pubkey: HashablePublicKey = (*pubkey).into();
        self.transactions.values()
            .filter(|tx| tx.tx.from_pubkey == hashable_pubkey)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{SignedTransaction, sign};
    use crate::transaction::Transaction;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_add_valid_transaction() {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            keypair.public,
            recipient_keypair.public,
            100,
            1,
        );

        let signature = sign(&tx, &keypair);
        let signed_tx = SignedTransaction::new(tx, signature);

        let mut mempool = Mempool::new();
        assert!(mempool.add_transaction(signed_tx).is_ok());
        assert_eq!(mempool.len(), 1);
    }

    #[test]
    fn test_reject_duplicate_transaction() {
        let mut csprng = OsRng;
        let keypair1: Keypair = Keypair::generate(&mut csprng);

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        // Create a transaction
        let tx = Transaction::new(
            keypair1.public,
            recipient_keypair.public,
            100,
            1,
        );

        // Sign it
        let signature = sign(&tx, &keypair1);
        let signed_tx = SignedTransaction::new(tx, signature);

        let mut mempool = Mempool::new();

        // Add it once - should succeed
        assert!(mempool.add_transaction(signed_tx.clone()).is_ok());
        assert_eq!(mempool.len(), 1);

        // Try to add the exact same transaction again - should fail due to nonce
        assert_eq!(
            mempool.add_transaction(signed_tx),
            Err(MempoolError::InvalidNonce)
        );
        assert_eq!(mempool.len(), 1);
    }

    #[test]
    fn test_nonce_tracking() {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let mut mempool = Mempool::new();

        // Add transaction with nonce 1
        let tx1 = Transaction::new(
            keypair.public,
            recipient_keypair.public,
            100,
            1,
        );
        let signature1 = sign(&tx1, &keypair);
        let signed_tx1 = SignedTransaction::new(tx1, signature1);
        assert!(mempool.add_transaction(signed_tx1).is_ok());

        // Try to add transaction with lower nonce (should fail)
        let tx_low = Transaction::new(
            keypair.public,
            recipient_keypair.public,
            50,
            1, // Same nonce
        );
        let signature_low = sign(&tx_low, &keypair);
        let signed_tx_low = SignedTransaction::new(tx_low, signature_low);
        assert_eq!(
            mempool.add_transaction(signed_tx_low),
            Err(MempoolError::InvalidNonce)
        );

        // Add transaction with higher nonce (should succeed)
        let tx2 = Transaction::new(
            keypair.public,
            recipient_keypair.public,
            200,
            2,
        );
        let signature2 = sign(&tx2, &keypair);
        let signed_tx2 = SignedTransaction::new(tx2, signature2);
        assert!(mempool.add_transaction(signed_tx2).is_ok());

        assert_eq!(mempool.len(), 2);
        assert_eq!(mempool.get_account_nonce(&keypair.public), 2);
    }

    #[test]
    fn test_reject_invalid_signature() {
        let mut csprng = OsRng;
        let keypair1: Keypair = Keypair::generate(&mut csprng);
        let keypair2: Keypair = Keypair::generate(&mut csprng);

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            keypair1.public,
            recipient_keypair.public,
            100,
            1,
        );

        // Sign with key1 but create transaction with key2 as sender (mismatch)
        let signature = sign(&tx, &keypair1);
        let wrong_tx = Transaction::new(
            keypair2.public, // Wrong public key
            recipient_keypair.public,
            100,
            1,
        );

        // Create signed transaction with mismatched data
        let signed_tx = SignedTransaction::new(wrong_tx, signature);

        let mut mempool = Mempool::new();
        assert_eq!(
            mempool.add_transaction(signed_tx),
            Err(MempoolError::InvalidSignature)
        );
    }

    #[test]
    fn test_remove_transaction() {
        let mut csprng = OsRng;
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let recipient_keypair: Keypair = Keypair::generate(&mut csprng);

        let tx = Transaction::new(
            keypair.public,
            recipient_keypair.public,
            100,
            1,
        );

        let signature = sign(&tx, &keypair);
        let signed_tx = SignedTransaction::new(tx.clone(), signature);
        let tx_id = signed_tx.tx_id;

        let mut mempool = Mempool::new();
        assert!(mempool.add_transaction(signed_tx).is_ok());
        assert_eq!(mempool.len(), 1);

        let removed = mempool.remove_transaction(&tx_id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().tx_id, tx_id);
        assert_eq!(mempool.len(), 0);
    }
}