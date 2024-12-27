use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::services::auth_service::AuthService;
use crate::error::AppError;
use std::sync::Arc;

/// Middleware for handling authentication
/// 
/// Validates JWT tokens in incoming requests
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    /// Validates the incoming request by checking the JWT token
    /// 
    /// # Arguments
    /// * `req` - The incoming service request
    /// 
    /// # Returns
    /// `Result<ServiceRequest, Error>` - The validated request or an error
    pub async fn validate(&self, req: ServiceRequest) -> Result<ServiceRequest, Error> {
        let bearer = BearerAuth::extract(&req)
            .await
            .map_err(|_| AppError::Authentication("Invalid token format".to_string()))?;

        let token = bearer.token();
        self.auth_service
            .validate_token(token)
            .map_err(|_| AppError::Authentication("Invalid or expired token".to_string()))?;

        Ok(req)
    }
}
