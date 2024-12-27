use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::services::auth_service::AuthService;
use std::sync::Arc;

pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn validate(&self, req: ServiceRequest) -> Result<ServiceRequest, Error> {
        let bearer = BearerAuth::extract(&req)
            .await
            .map_err(|_| ErrorUnauthorized("Invalid token"))?;

        let token = bearer.token();
        self.auth_service
            .validate_token(token)
            .map_err(|_| ErrorUnauthorized("Invalid token"))?;

        Ok(req)
    }
}
