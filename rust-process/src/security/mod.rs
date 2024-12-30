mod encryption;
mod error;
mod key_manager;

pub use encryption::{EncryptedData, EncryptionEngine};
pub use error::SecurityError;
pub use key_manager::KeyManager;

pub type Result<T> = std::result::Result<T, SecurityError>;
