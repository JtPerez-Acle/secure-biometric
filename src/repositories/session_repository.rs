use crate::models::Session;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, session: &Session) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO sessions (id, user_id, created_at, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            session.id,
            session.user_id,
            session.created_at,
            session.expires_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Session>, RepositoryError> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, created_at, expires_at
            FROM sessions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
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
            DELETE FROM sessions
            WHERE expires_at < NOW()
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
