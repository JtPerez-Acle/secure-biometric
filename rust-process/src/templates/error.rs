use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Invalid template data: {0}")]
    InvalidData(String),

    #[error("Invalid template format: {0}")]
    InvalidFormat(String),

    #[error("Template validation failed: {0}")]
    ValidationFailed(String),

    #[error("Template quality below threshold: {0}")]
    QualityBelowThreshold(f32),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
