use thiserror::Error;

/// Represents an error with the coin.
///
/// # Fields
///
/// * `NoGenesisBlock` - The chain has no genesis block.
/// * `BlockFull` - The block is full.
/// * `SerializationError` - The transaction cannot be serialized to JSON.
/// * `InvalidSignature` - The signature is invalid.
/// * `HashNotFound` - The hash was not found.
#[derive(Debug, Error)]
pub enum CoinError {
    #[error("The chain has no genesis block!")]
    NoGenesisBlock,
    #[error("The block cannot contain more than {0} transactions!")]
    BlockFull(usize),
    #[error("The transaction cannot be serialized to JSON!")]
    SerializationError(#[from] serde_json::Error),
    #[error("Invalid signature!")]
    InvalidSignature(#[from] ed25519_dalek::SignatureError),
    #[error("Hash not found!")]
    HashNotFound,
}
