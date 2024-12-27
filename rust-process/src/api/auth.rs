use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::models::User;
use crate::services::auth_service::AuthService;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn login(
    auth_service: web::Data<Arc<AuthService>>,
    credentials: web::Json<LoginRequest>,
) -> HttpResponse {
    // TODO: Implement login logic
    HttpResponse::Ok().finish()
}

pub async fn register(
    auth_service: web::Data<Arc<AuthService>>,
    credentials: web::Json<RegisterRequest>,
) -> HttpResponse {
    // TODO: Implement registration logic
    HttpResponse::Ok().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth/login")
            .route(web::post().to(login))
    )
    .service(
        web::resource("/auth/register")
            .route(web::post().to(register))
    );
}
