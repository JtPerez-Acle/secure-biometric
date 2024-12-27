use crate::models::{User, Task};
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.username,
            user.email,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, RepositoryError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
