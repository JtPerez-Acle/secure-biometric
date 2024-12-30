mod error;
mod vault;

pub use error::StorageError;
pub use vault::TemplateVault;

pub type Result<T> = std::result::Result<T, StorageError>;
