use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Authentication failed")]
    AuthenticationError,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Item not found")]
    NotFound,
    
    #[error("Internal Server Error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error".to_string()),
            AppError::AuthenticationError => (StatusCode::UNAUTHORIZED, "Authentication Failed".to_string()),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found".to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid Credentials".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };

        (status, error_message).into_response()
    }
} 