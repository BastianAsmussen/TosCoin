use crate::coin::blocks::{Block, BLOCK_DIFFICULTY};
use crate::coin::errors::CoinError;
use crate::coin::transactions::Transaction;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

lazy_static! {
    /// The blockchain.
    pub static ref CHAIN: Arc<Mutex<Chain>> = Arc::new(Mutex::new(Chain::default()));
}

/// Represents a chain of blocks.
///
/// # Fields
///
/// * `blocks` - The blocks in the chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    /// Adds a block to the chain.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to add.
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Gets a reference to the last block in the chain.
    ///
    /// # Returns
    ///
    /// * `Result<&Block, CoinError>` - The last block in the chain.
    ///
    /// # Errors
    ///
    /// * If the chain has no genesis block.
    pub fn last_block(&self) -> Result<&Block, CoinError> {
        self.blocks.last().ok_or(CoinError::NoGenesisBlock)
    }

    /// Gets a mutable reference to the last block in the chain.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Block, CoinError>` - A mutable reference to the last block in the chain.
    ///
    /// # Errors
    ///
    /// * If the chain has no genesis block.
    pub fn last_block_mut(&mut self) -> Result<&mut Block, CoinError> {
        self.blocks.last_mut().ok_or(CoinError::NoGenesisBlock)
    }

    /// Adds a transaction to the chain.
    ///
    /// # Arguments
    ///
    /// * `transaction` - The transaction to add.
    /// * `public_key` - The public key of the sender.
    /// * `signature` - The signature of the transaction.
    ///
    /// # Returns
    ///
    /// * `Result<Option<String>, CoinError>` - The solution to the block, if any.
    ///
    /// # Errors
    ///
    /// * If the chain has no genesis block.
    /// * If the block is full.
    /// * If the transaction cannot be serialized to JSON.
    /// * If the signature is invalid.
    /// * If the hash of the previous block cannot be calculated.
    pub fn add_transaction(
        &mut self,
        transaction: Transaction,
        public_key: &VerifyingKey,
        signature: &Signature,
    ) -> Result<Option<String>, CoinError> {
        // Verify the signature of the transaction.
        let transaction_json = serde_json::to_string(&transaction)?;
        public_key.verify(transaction_json.as_ref(), signature)?;

        // Add the transaction to the chain, creating a new block if necessary.
        let last_block = self.last_block_mut()?;
        if !last_block.is_full() {
            last_block.add_transaction(transaction)?;

            return Ok(None);
        }

        let new_block = Block::new(Some(last_block.hash()?), vec![transaction]);
        let solution = new_block.mine(BLOCK_DIFFICULTY);

        self.add_block(new_block);

        Ok(Some(solution))
    }
}

impl Default for Chain {
    fn default() -> Self {
        let genesis_block = Block::default();

        Self {
            blocks: vec![genesis_block],
        }
    }
}
