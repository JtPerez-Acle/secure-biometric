use crate::models::Project;
use sqlx::PgPool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, project: &Project) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO projects (id, user_id, name, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            project.id,
            project.user_id,
            project.name,
            project.description,
            project.created_at,
            project.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Project>, RepositoryError> {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(project)
    }

    pub async fn find_by_user(&self, user_id: uuid::Uuid) -> Result<Vec<Project>, RepositoryError> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM projects
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(projects)
    }

    pub async fn update(&self, project: &Project) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE projects
            SET name = $2, description = $3, updated_at = $4
            WHERE id = $1
            "#,
            project.id,
            project.name,
            project.description,
            project.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
