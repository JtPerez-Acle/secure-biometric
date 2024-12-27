use crate::config::database::DatabaseConfig;
use crate::models::User;
use crate::repositories::user_repository::UserRepository;
use chrono::Utc;
use dotenvy::dotenv;
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_db() -> PgPool {
    dotenv().ok();
    let config = DatabaseConfig::from_env();
    config.create_pool().await.unwrap()
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
