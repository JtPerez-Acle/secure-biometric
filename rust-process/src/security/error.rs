use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Key generation error: {0}")]
    KeyGeneration(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Integrity check failed: {0}")]
    IntegrityError(String),
}
