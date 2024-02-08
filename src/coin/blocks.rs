use crate::coin::errors::CoinError;
use crate::coin::transactions::Transaction;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

/// The maximum number of transactions in a block.
pub const MAX_TRANSACTIONS: usize = 1;

/// The difficulty of the block.
pub const BLOCK_DIFFICULTY: usize = 5;

/// Represents a block.
///
/// # Fields
///
/// * `previous_hash` - The hash of the previous block.
/// * `transactions` - The transactions in the block.
/// * `timestamp` - The time in milliseconds since the Unix epoch.
/// * `nonce` - The nonce of the block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub previous_hash: Option<String>,
    pub transactions: Vec<Transaction>,
    pub timestamp: u128,
    pub nonce: u64,
}

impl Block {
    /// Creates a new block.
    ///
    /// # Arguments
    ///
    /// * `previous_hash` - The hash of the previous block, if any.
    /// * `transactions` - The transactions in the block.
    ///
    /// # Returns
    ///
    /// * `Block` - The new block.
    #[must_use]
    pub fn new(previous_hash: Option<String>, transactions: Vec<Transaction>) -> Self {
        Self {
            previous_hash,
            transactions,
            timestamp: crate::utils::time::current_time_millis(),
            nonce: rand::random(),
        }
    }

    /// Hashes the block.
    ///
    /// # Returns
    ///
    /// * `Result<String, CoinError>` - The hash of the block.
    ///
    /// # Errors
    ///
    /// * If the block cannot be serialized to JSON.
    pub fn hash(&self) -> Result<String, CoinError> {
        let json = serde_json::to_string(&self)?;

        let mut hasher = Sha3_256::new();
        hasher.update(json);

        let result = hasher.finalize();

        Ok(format!("{result:x}"))
    }

    /// Mines the block.
    ///
    /// # Arguments
    ///
    /// * `difficulty` - The difficulty of the block.
    ///
    /// # Returns
    ///
    /// * `String` - The solution to the block.
    #[must_use]
    pub fn mine(&self, difficulty: usize) -> String {
        let prefix = "0".repeat(difficulty);
        let mut solution = 1;

        loop {
            let digest = md5::compute(format!("{}", self.nonce + solution));

            let attempt = format!("{digest:x}");
            if attempt.starts_with(&prefix) {
                return attempt;
            }

            solution += 1;
        }
    }

    /// Checks if the block is full.
    ///
    /// # Returns
    ///
    /// * `bool` - The result of comparing the length of the transactions to [`MAX_TRANSACTIONS`].
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.transactions.len() >= MAX_TRANSACTIONS
    }

    /// Adds a transaction to the block.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The transaction to add.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - The result of the operation.
    ///
    /// # Errors
    ///
    /// * If the block is too old.
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), CoinError> {
        if self.is_full() {
            return Err(CoinError::BlockFull(MAX_TRANSACTIONS));
        }

        self.transactions.push(transaction);

        Ok(())
    }
}

impl Default for Block {
    /// Creates a default block.
    ///
    /// # Returns
    ///
    /// * `Block` - The default block.
    fn default() -> Self {
        let genesis_transaction = Transaction::default();

        Self::new(None, vec![genesis_transaction])
    }
}
