// State module placeholder

use serde::{Deserialize, Serialize};

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
}
