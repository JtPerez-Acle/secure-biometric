mod security;
mod storage;
mod templates;

use actix_web::{web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting secure biometric system...");
    
    // Initialize template vault
    let vault = storage::TemplateVault::new("data/templates")
        .await
        .expect("Failed to initialize template vault");
    let vault = web::Data::new(vault);
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(vault.clone())
            // TODO: Add API routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
