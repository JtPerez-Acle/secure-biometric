use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
}

pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            connection_string: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .map(|s| s.parse().unwrap_or(10))
                .unwrap_or(10),
        }
    }

    pub async fn create_pool(&self) -> Result<PgPool, DatabaseError> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.connection_string)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
    }
}
