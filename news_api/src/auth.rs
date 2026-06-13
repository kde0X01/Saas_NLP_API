// Authentification related functionality
// auth.rs

use crate::errors::AppError;
use crate::models::GoogleUser;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Google ID
    pub email: String,
    pub exp: usize, // Expiration time
}

// JWT Configuration
pub struct JwtConfig {
    pub secret: String,
    pub expiration_days: i64,
}

// Load JWT configuration from environment variables
impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
            expiration_days: 7, // 7 days
        }
    }
}

// Generate a JWT for the given Google user
pub fn generate_token(google_user: &GoogleUser) -> Result<String, AppError> {
    let config: JwtConfig = JwtConfig::from_env();
    let expiration: usize = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::Unexpected("Time went backwards".to_string()))?
        .as_secs() as usize
        + (config.expiration_days as usize * 24 * 60 * 60);

    // Create the claims
    let claims = Claims {
        sub: google_user.sub.clone(),
        email: google_user.email.clone(),
        exp: expiration,
    };

    // Encode the token
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
    .map_err(|e| AppError::Unexpected(format!("JWT encoding error: {}", e)))
}

// Validate a JWT and return the claims if valid
pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let config: JwtConfig = JwtConfig::from_env();
    let validation: Validation = Validation::default();

    // Decode and validate the token
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::AuthError(format!("Invalid token: {}", e)))
}
