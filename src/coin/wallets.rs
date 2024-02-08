use crate::coin::chains::CHAIN;
use crate::coin::errors::CoinError;
use crate::coin::transactions::Transaction;
use anyhow::Result;
use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

/// Represents a wallet.
///
/// # Fields
///
/// * `public_key` - The public key of the wallet.
/// * `private_key` - The private key of the wallet.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
}

impl Wallet {
    /// Creates a new wallet.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key of the wallet.
    /// * `private_key` - The private key of the wallet.
    ///
    /// # Returns
    ///
    /// * `Wallet` - The new wallet.
    #[must_use]
    pub const fn new(public_key: VerifyingKey, private_key: SigningKey) -> Self {
        Self {
            public_key,
            private_key,
        }
    }

    /// Sends a transaction.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to send.
    /// * `receiver` - The public key of the receiver.
    ///
    /// # Returns
    ///
    /// * `Result<Option<String>, CoinError>` - The result of the operation.
    ///
    /// # Errors
    ///
    /// * If the transaction cannot be serialized to JSON.
    /// * If the transaction cannot be added to the chain.
    pub fn send(
        &mut self,
        amount: f64,
        receiver: VerifyingKey,
    ) -> Result<Option<String>, CoinError> {
        // Create a new transaction.
        let transaction = Transaction::new(amount, self.public_key, receiver);
        let json = serde_json::to_string(&transaction)?;

        // Sign the transaction.
        let signature: Signature = self.private_key.sign(json.as_ref());

        // Add the transaction to the chain.
        let solution = CHAIN
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .add_transaction(transaction, &self.public_key, &signature)?;

        Ok(solution)
    }
}

impl Default for Wallet {
    fn default() -> Self {
        let signing_key: SigningKey = SigningKey::generate(&mut OsRng);

        let public_key = signing_key.verifying_key();
        let private_key = signing_key;

        Self {
            public_key,
            private_key,
        }
    }
}
