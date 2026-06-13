// errors.rs

use actix_web::{ResponseError, http::StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(rusqlite::Error),
    // Add other specific error types as needed
    NotFound,
    // Authentication-related errors
    AuthError(String),
    // TokenValidationError(ClaimsVerificationError),
    Unexpected(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(err) => write!(f, "Database error: {}", err),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            // AppError::TokenValidationError(err) => write!(f, "Token validation error: {}", err),
            AppError::Unexpected(msg) => write!(f, "Unexpected error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            // AppError::TokenValidationError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
