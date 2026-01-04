// State module placeholder

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ed25519_dalek::PublicKey;

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
}
