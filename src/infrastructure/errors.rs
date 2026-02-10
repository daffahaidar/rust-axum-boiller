use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Token creation error")]
    TokenCreationError,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Password hashing error")]
    PasswordHashingError,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Forbidden")]
    Forbidden,
    #[error("Cannot delete your own account")]
    CannotDeleteSelf,
    #[error("OAuth error: {0}")]
    OAuthError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AppError::EmailAlreadyExists => (StatusCode::CONFLICT, "Email already exists".to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()),
            AppError::TokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::PasswordHashingError => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::CannotDeleteSelf => (StatusCode::BAD_REQUEST, "Cannot delete your own account".to_string()),
            AppError::OAuthError(msg) => {
                tracing::error!("OAuth error: {}", msg);
                (StatusCode::BAD_REQUEST, format!("OAuth error: {}", msg))
            }
        };

        let body = Json(json!({
            "meta": {
                "status": "error",
                "message": message
            },
            "results": null
        }));

        (status, body).into_response()
    }
}
