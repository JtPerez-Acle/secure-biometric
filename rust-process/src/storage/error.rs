use crate::security::SecurityError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Template not found: {0}")]
    NotFound(Uuid),

    #[error("Encryption error: {0}")]
    Encryption(#[from] SecurityError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] Box<bincode::ErrorKind>),

    #[error("Storage error: {0}")]
    Storage(#[from] sled::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
