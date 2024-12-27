use crate::config::database::DatabaseConfig;
use crate::models::{User, Task, Project, Session, ApiKey};
use crate::repositories::{user_repository::UserRepository, task_repository::TaskRepository, 
                         project_repository::ProjectRepository, session_repository::SessionRepository,
                         api_key_repository::ApiKeyRepository};
use chrono::Utc;
use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn setup_test_db() -> PgPool {
    dotenvy::dotenv().ok();
    // Set test database URL if not already set
    if env::var("DATABASE_URL").is_err() {
        env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/test_db");
    }
    let config = DatabaseConfig::from_env();
    let pool = config.create_pool().await.unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
        
    pool
}

#[tokio::test]
async fn test_task_repository() {
    let pool = setup_test_db().await;
    let user_repo = UserRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool.clone());

    // Clean up before test
    sqlx::query!("DELETE FROM tasks").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM users").execute(&pool).await.unwrap();

    // Create a user first
    let user = User {
        id: Uuid::new_v4(),
        username: "taskuser".to_string(),
        email: "task@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    user_repo.create(&user).await.unwrap();

    let task = Task {
        id: Uuid::new_v4(),
        user_id: user.id,
        title: "Test Task".to_string(),
        description: Some("Test Description".to_string()),
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    task_repo.create(&task).await.unwrap();

    // Test find by id
    let found_task = task_repo.find_by_id(task.id).await.unwrap();
    assert!(found_task.is_some());
    let found_task = found_task.unwrap();
    assert_eq!(found_task.title, "Test Task");
    assert_eq!(found_task.description, Some("Test Description".to_string()));

    // Test find by user
    let user_tasks = task_repo.find_by_user(user.id).await.unwrap();
    assert_eq!(user_tasks.len(), 1);

    // Test update
    let mut updated_task = task;
    updated_task.title = "Updated Task".to_string();
    task_repo.update(&updated_task).await.unwrap();
    let found_task = task_repo.find_by_id(updated_task.id).await.unwrap().unwrap();
    assert_eq!(found_task.title, "Updated Task");

    // Test delete
    task_repo.delete(updated_task.id).await.unwrap();
    let found_task = task_repo.find_by_id(updated_task.id).await.unwrap();
    assert!(found_task.is_none());
}

#[tokio::test]
async fn test_project_repository() {
    let pool = setup_test_db().await;
    let user_repo = UserRepository::new(pool.clone());
    let project_repo = ProjectRepository::new(pool.clone());

    // Clean up before test
    sqlx::query!("DELETE FROM projects").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM users").execute(&pool).await.unwrap();

    // Create a user first
    let user = User {
        id: Uuid::new_v4(),
        username: "projectuser".to_string(),
        email: "project@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    user_repo.create(&user).await.unwrap();

    let project = Project {
        id: Uuid::new_v4(),
        user_id: user.id,
        name: "Test Project".to_string(),
        description: Some("Test Description".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    project_repo.create(&project).await.unwrap();

    // Test find by id
    let found_project = project_repo.find_by_id(project.id).await.unwrap();
    assert!(found_project.is_some());
    let found_project = found_project.unwrap();
    assert_eq!(found_project.name, "Test Project");
    assert_eq!(found_project.description, Some("Test Description".to_string()));

    // Test find by user
    let user_projects = project_repo.find_by_user(user.id).await.unwrap();
    assert_eq!(user_projects.len(), 1);

    // Test update
    let mut updated_project = project;
    updated_project.name = "Updated Project".to_string();
    project_repo.update(&updated_project).await.unwrap();
    let found_project = project_repo.find_by_id(updated_project.id).await.unwrap().unwrap();
    assert_eq!(found_project.name, "Updated Project");

    // Test delete
    project_repo.delete(updated_project.id).await.unwrap();
    let found_project = project_repo.find_by_id(updated_project.id).await.unwrap();
    assert!(found_project.is_none());
}

#[tokio::test]
async fn test_session_repository() {
    let pool = setup_test_db().await;
    let user_repo = UserRepository::new(pool.clone());
    let session_repo = SessionRepository::new(pool.clone());

    // Clean up before test
    sqlx::query!("DELETE FROM sessions").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM users").execute(&pool).await.unwrap();

    // Create a user first
    let user = User {
        id: Uuid::new_v4(),
        username: "sessionuser".to_string(),
        email: "session@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    user_repo.create(&user).await.unwrap();

    let session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
    };

    // Test create
    session_repo.create(&session).await.unwrap();

    // Test find by id
    let found_session = session_repo.find_by_id(session.id).await.unwrap();
    assert!(found_session.is_some());
    let found_session = found_session.unwrap();
    assert_eq!(found_session.user_id, user.id);

    // Test delete
    session_repo.delete(session.id).await.unwrap();
    let found_session = session_repo.find_by_id(session.id).await.unwrap();
    assert!(found_session.is_none());
}

#[tokio::test]
async fn test_api_key_repository() {
    let pool = setup_test_db().await;
    let user_repo = UserRepository::new(pool.clone());
    let api_key_repo = ApiKeyRepository::new(pool.clone());

    // Clean up before test
    sqlx::query!("DELETE FROM api_keys").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM users").execute(&pool).await.unwrap();

    // Create a user first
    let user = User {
        id: Uuid::new_v4(),
        username: "apikeyuser".to_string(),
        email: "apikey@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    user_repo.create(&user).await.unwrap();

    let api_key = ApiKey {
        id: Uuid::new_v4(),
        user_id: user.id,
        key: "test-key".to_string(),
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
    };

    // Test create
    api_key_repo.create(&api_key).await.unwrap();

    // Test find by key
    let found_api_key = api_key_repo.find_by_key(&api_key.key).await.unwrap();
    assert!(found_api_key.is_some());
    let found_api_key = found_api_key.unwrap();
    assert_eq!(found_api_key.user_id, user.id);

    // Test delete
    api_key_repo.delete(api_key.id).await.unwrap();
    let found_api_key = api_key_repo.find_by_key(&api_key.key).await.unwrap();
    assert!(found_api_key.is_none());
}

#[tokio::test]
async fn test_user_repository() {
    let pool = setup_test_db().await;
    let repo = UserRepository::new(pool.clone());

    // Clean up before test
    sqlx::query!("DELETE FROM users").execute(&pool).await.unwrap();

    let user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test create
    repo.create(&user).await.unwrap();

    // Test find
    let found_user = repo.find_by_id(user.id).await.unwrap();
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.username, "testuser");
    assert_eq!(found_user.email, "test@example.com");
}
