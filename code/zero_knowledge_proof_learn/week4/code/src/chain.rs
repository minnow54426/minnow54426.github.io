use anyhow::Result;
use crate::state::State;
use crate::block::Block;

pub fn apply_block(state: &mut State, block: &Block) -> Result<()> {
    for signed_tx in &block.txs {
        state.apply_tx(signed_tx)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{State, Account};
    use crate::block::Block;
    use tx_rs::{Transaction, SignedTransaction, sign};
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_apply_block_with_valid_txs() {
        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Setup accounts
        state.set_account(alice_key.public, Account::new(100, 0));
        state.set_account(bob_key.public, Account::new(50, 0));

        // Create transaction
        let tx = Transaction::new(
            alice_key.public,
            bob_key.public,
            30,
            0,
        );
        let sig = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, sig);

        // Create block
        let block = Block::new(
            [0u8; 32],
            vec![signed_tx],
            1,
            1234567890,
        );

        // Apply block
        apply_block(&mut state, &block).unwrap();

        // Verify state
        let alice_account = state.get_account(&alice_key.public).unwrap();
        assert_eq!(alice_account.balance, 70);
        assert_eq!(alice_account.nonce, 1);

        let bob_account = state.get_account(&bob_key.public).unwrap();
        assert_eq!(bob_account.balance, 80);
    }

    #[test]
    fn test_apply_block_with_invalid_tx() {
        let mut state = State::new();
        let alice_key = Keypair::generate(&mut OsRng);
        let bob_key = Keypair::generate(&mut OsRng);

        // Alice has insufficient balance
        state.set_account(alice_key.public, Account::new(10, 0));

        // Try to send 100
        let tx = Transaction::new(
            alice_key.public,
            bob_key.public,
            100,
            0,
        );
        let sig = sign(&tx, &alice_key);
        let signed_tx = SignedTransaction::new(tx, sig);

        let block = Block::new(
            [0u8; 32],
            vec![signed_tx],
            1,
            1234567890,
        );

        // Should fail
        let result = apply_block(&mut state, &block);
        assert!(result.is_err());

        // State should be unchanged
        let alice_account = state.get_account(&alice_key.public).unwrap();
        assert_eq!(alice_account.balance, 10);
        assert_eq!(alice_account.nonce, 0);
    }
}
