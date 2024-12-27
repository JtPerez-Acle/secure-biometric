use crate::models::ApiKey;
use sqlx::PgPool;
use crate::error::{AppError, AppResult};

/// Repository for managing API keys in the database

pub struct ApiKeyRepository {
    pool: PgPool,
}

impl ApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Creates a new API key in the database
    /// 
    /// # Arguments
    /// * `api_key` - The API key to create
    /// 
    /// # Returns
    /// `AppResult<()>` - Result indicating success or failure
    pub async fn create(&self, api_key: &ApiKey) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO api_keys (id, user_id, key, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            api_key.id,
            api_key.user_id,
            api_key.key,
            api_key.created_at,
            api_key.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Finds an API key by its key value
    /// 
    /// # Arguments
    /// * `key` - The API key to search for
    /// 
    /// # Returns
    /// `AppResult<Option<ApiKey>>` - The found API key or None if not found
    pub async fn find_by_key(&self, key: &str) -> AppResult<Option<ApiKey>> {
        let api_key = sqlx::query_as!(
            ApiKey,
            r#"
            SELECT id, user_id, key, created_at, expires_at
            FROM api_keys
            WHERE key = $1
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(api_key)
    }

    /// Deletes an API key by its ID
    /// 
    /// # Arguments
    /// * `id` - The ID of the API key to delete
    /// 
    /// # Returns
    /// `AppResult<()>` - Result indicating success or failure
    pub async fn delete(&self, id: uuid::Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM api_keys
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Deletes all expired API keys
    /// 
    /// # Returns
    /// `AppResult<()>` - Result indicating success or failure
    pub async fn delete_expired(&self) -> AppResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM api_keys
            WHERE expires_at < NOW()
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
