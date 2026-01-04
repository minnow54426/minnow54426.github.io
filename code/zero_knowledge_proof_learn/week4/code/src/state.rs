// State module placeholder

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ed25519_dalek::PublicKey;
use tx_rs::{Transaction, SignedTransaction, sign};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

impl Account {
    pub fn new(balance: u64, nonce: u64) -> Self {
        Self { balance, nonce }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    accounts: HashMap<[u8; 32], Account>,
}

impl State {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn get_account(&self, pubkey: &PublicKey) -> Option<&Account> {
        self.accounts.get(pubkey.as_bytes())
    }

    pub fn set_account(&mut self, pubkey: PublicKey, account: Account) {
        self.accounts.insert(*pubkey.as_bytes(), account);
    }

    pub fn apply_tx(&mut self, signed_tx: &SignedTransaction) -> Result<()> {
        // Verify signature
        if !signed_tx.verify() {
            return Err(anyhow::anyhow!("Invalid signature"));
        }

        let tx = &signed_tx.tx;

        // Check sender has sufficient balance
        let sender_account = self.get_account(&tx.from_pubkey.0)
            .ok_or_else(|| anyhow::anyhow!("Sender account not found"))?;

        if sender_account.balance < tx.amount {
            return Err(anyhow::anyhow!("Insufficient balance"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_account_creation() {
        let account = Account::new(100, 0);
        assert_eq!(account.balance, 100);
        assert_eq!(account.nonce, 0);
    }

    #[test]
    fn test_account_serialization() {
        let account = Account::new(100, 5);
        let json = serde_json::to_string(&account).unwrap();
        assert_eq!(json, r#"{"balance":100,"nonce":5}"#);
    }

    #[test]
    fn test_state_get_and_set() {
        use ed25519_dalek::SecretKey;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let key = SecretKey::generate(&mut OsRng);
        let pubkey: PublicKey = (&key).into();

        // Get non-existent account
        let account = state.get_account(&pubkey);
        assert_eq!(account, None);

        // Set account
        state.set_account(pubkey, Account::new(100, 0));

        // Get account
        let account = state.get_account(&pubkey);
        assert_eq!(account, Some(&Account::new(100, 0)));
    }

    #[test]
    fn test_apply_tx_invalid_signature() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Fund Alice
        let pubkey1: PublicKey = alice_key.public;
        state.set_account(pubkey1, Account::new(100, 0));

        // Create transaction from Alice to Bob
        let pubkey2: PublicKey = bob_key.public;
        let tx = Transaction::new(
            pubkey1,
            pubkey2,
            50,
            0,
        );

        // Sign with WRONG key (Bob signs instead of Alice)
        let signature = sign(&tx, &bob_key);
        let signed_tx = SignedTransaction::new(tx, signature);

        // Try to apply - should fail
        let result = state.apply_tx(&signed_tx);
        assert!(result.is_err());
    }

    #[test]
    fn test_apply_tx_insufficient_balance() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Fund Alice with only 10 tokens
        state.set_account(alice_key.public, Account::new(10, 0));

        // Try to send 50 tokens
        let tx = Transaction::new(
            alice_key.public,
            bob_key.public,
            50,
            0,
        );

        let signature = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, signature);

        // Should fail - insufficient balance
        let result = state.apply_tx(&signed_tx);
        assert!(result.is_err());
    }
}
