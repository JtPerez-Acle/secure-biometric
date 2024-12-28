use crate::models::User;
use crate::services::auth_service::{AuthService, AuthError};
use crate::middleware::auth_middleware::AuthMiddleware;
use actix_web::test::TestRequest;
use chrono::Utc;
use std::sync::Arc;
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

#[tokio::test]
async fn test_auth_middleware() {
    let auth_service = Arc::new(AuthService::new("test_secret".to_string(), 24));
    let middleware = AuthMiddleware::new(auth_service.clone());
    
    let user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Create a valid token
    let token = auth_service.create_token(&user).unwrap();

    // Test valid token
    let req = TestRequest::default()
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_srv_request();
    let result = middleware.validate(req).await;
    assert!(result.is_ok());

    // Test invalid token
    let req = TestRequest::default()
        .insert_header(("Authorization", "Bearer invalid.token"))
        .to_srv_request();
    let result = middleware.validate(req).await;
    assert!(result.is_err());

    // Test missing token
    let req = TestRequest::default().to_srv_request();
    let result = middleware.validate(req).await;
    assert!(result.is_err());
}
