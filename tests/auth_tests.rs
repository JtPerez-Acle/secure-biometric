use crate::models::User;
use crate::services::auth_service::{AuthService, AuthError};
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_token_creation_and_validation() {
    let auth_service = AuthService::new("test_secret".to_string(), 24);
    
    let user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test token creation
    let token = auth_service.create_token(&user).unwrap();
    
    // Test token validation
    let user_id = auth_service.validate_token(&token).unwrap();
    assert_eq!(user_id, user.id);

    // Test invalid token
    let invalid_token = "invalid.token.string";
    let result = auth_service.validate_token(invalid_token);
    assert!(matches!(result, Err(AuthError::InvalidToken)));
}

#[tokio::test]
async fn test_expired_token() {
    let auth_service = AuthService::new("test_secret".to_string(), -1); // Negative duration for expired token
    
    let user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let token = auth_service.create_token(&user).unwrap();
    let result = auth_service.validate_token(&token);
    assert!(matches!(result, Err(AuthError::InvalidToken)));
}
