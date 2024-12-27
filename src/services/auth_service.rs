use crate::models::{User, Task};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user id
    pub exp: usize, // expiration time
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Token creation error")]
    TokenCreationError,
    #[error("Invalid token")]
    InvalidToken,
}

pub struct AuthService {
    secret: String,
    token_duration: i64, // in hours
}

impl AuthService {
    pub fn new(secret: String, token_duration: i64) -> Self {
        Self {
            secret,
            token_duration,
        }
    }

    pub fn create_token(&self, user: &User) -> Result<String, AuthError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(self.token_duration))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id,
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| AuthError::TokenCreationError)
    }

    pub fn validate_token(&self, token: &str) -> Result<Uuid, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims.sub)
    }
}
