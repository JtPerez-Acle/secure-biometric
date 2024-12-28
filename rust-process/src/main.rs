mod api;
mod middleware;
mod models;
mod repositories;
mod services;
mod config;
mod rag;

use actix_web::{web, App, HttpServer};
use crate::config::database::DatabaseConfig;
use crate::middleware::{auth_middleware::AuthMiddleware, rate_limiter::RateLimiter, logger::RequestLogger};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    
    let db_config = DatabaseConfig::from_env();
    let pool = db_config.create_pool().await.expect("Failed to create database pool");
    
    let auth_service = Arc::new(services::auth_service::AuthService::new(
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        24 // token duration in hours
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .wrap(RequestLogger::new())
            .wrap(RateLimiter::new(100, 60)) // 100 requests per minute
            .configure(api::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
