use crate::models::ApiKey;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct ApiKeyRepository {
    pool: PgPool,
}

impl ApiKeyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, api_key: &ApiKey) -> Result<(), RepositoryError> {
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

    pub async fn find_by_key(&self, key: &str) -> Result<Option<ApiKey>, RepositoryError> {
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

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), RepositoryError> {
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

    pub async fn delete_expired(&self) -> Result<(), RepositoryError> {
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
