use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde::Serialize;

/// Unified error type for the application
#[derive(Error, Debug, Serialize)]
pub enum AppError {
    /// Authentication related errors
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    /// Authorization related errors
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    /// Database related errors
    #[error("Database error: {0}")]
    Database(String),
    
    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// Rate limiting errors
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Authentication(_) => HttpResponse::Unauthorized().json(self),
            AppError::Authorization(_) => HttpResponse::Forbidden().json(self),
            AppError::Database(_) => HttpResponse::InternalServerError().json(self),
            AppError::Validation(_) => HttpResponse::BadRequest().json(self),
            AppError::NotFound(_) => HttpResponse::NotFound().json(self),
            AppError::RateLimitExceeded => HttpResponse::TooManyRequests().json(self),
            AppError::Internal(_) => HttpResponse::InternalServerError().json(self),
        }
    }
}

/// Convenience type for Result<T, AppError>
pub type AppResult<T> = Result<T, AppError>;
