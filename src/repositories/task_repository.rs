use crate::models::Task;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, task: &Task) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO tasks (id, user_id, title, description, completed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            task.id,
            task.user_id,
            task.title,
            task.description,
            task.completed,
            task.created_at,
            task.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Task>, RepositoryError> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT id, user_id, title, description, completed, created_at, updated_at
            FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    pub async fn find_by_user(&self, user_id: uuid::Uuid) -> Result<Vec<Task>, RepositoryError> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT id, user_id, title, description, completed, created_at, updated_at
            FROM tasks
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    pub async fn update(&self, task: &Task) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE tasks
            SET title = $2, description = $3, completed = $4, updated_at = $5
            WHERE id = $1
            "#,
            task.id,
            task.title,
            task.description,
            task.completed,
            task.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
