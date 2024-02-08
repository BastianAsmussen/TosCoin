use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

/// Represents a transaction.
///
/// # Fields
///
/// * `amount` - The amount of the transaction.
/// * `sender` - The sender of the transaction.
/// * `receiver` - The receiver of the transaction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    /// The amount of the transaction.
    pub amount: f64,
    /// The public key of the sender.
    pub sender: VerifyingKey,
    /// The public key of the receiver.
    pub receiver: VerifyingKey,
}

impl Transaction {
    /// Creates a new transaction.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of the transaction.
    /// * `sender` - The public key of the sender.
    /// * `receiver` - The public key of the receiver.
    ///
    /// # Returns
    ///
    /// * `Transaction` - The new transaction.
    #[must_use]
    pub const fn new(amount: f64, sender: VerifyingKey, receiver: VerifyingKey) -> Self {
        Self {
            amount,
            sender,
            receiver,
        }
    }
}

impl Default for Transaction {
    /// Creates a default transaction.
    ///
    /// # Returns
    ///
    /// * `Transaction` - The default transaction.
    #[allow(clippy::unwrap_used)]
    fn default() -> Self {
        Self {
            amount: 128.0,
            sender: VerifyingKey::from_bytes(&[0; 32]).unwrap(),
            receiver: VerifyingKey::from_bytes(&[0; 32]).unwrap(),
        }
    }
}
