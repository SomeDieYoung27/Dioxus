use crate::{
    db::DB,
    errors::AppError,
    models::{AuthResponse, LoginUser, RegisterUser, User},
};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub async fn register(
    State(db): State<DB>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let user_id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    sqlx::query!(
        "INSERT INTO users (id, username, password_hash, created_at) VALUES ($1, $2, $3, $4)",
        user_id,
        payload.username,
        payload.password, // In a real app, hash the password
        created_at,
    )
    .execute(&db)
    .await?;

    // For simplicity, we'll just return a dummy token.
    // In a real app, you would generate a JWT.
    let token = "dummy-token".to_string();

    Ok((StatusCode::CREATED, Json(AuthResponse { token })))
}

pub async fn login(
    State(db): State<DB>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<AuthResponse>, AppError> {
    let user: Option<User> = sqlx::query_as!(
        User,
        r#"SELECT id as "id: Uuid", username, password_hash, created_at as "created_at: DateTime<Utc>" FROM users WHERE username = $1"#,
        payload.username
    )
    .fetch_optional(&db)
    .await?;

    if let Some(user) = user {
        // In a real app, verify the password hash
        if user.password_hash == payload.password {
            // For simplicity, we'll just return a dummy token.
            let token = "dummy-token".to_string();
            Ok(Json(AuthResponse { token }))
        } else {
            Err(AppError::InvalidCredentials)
        }
    } else {
        Err(AppError::UserNotFound)
    }
}

pub async fn logout() -> Result<StatusCode, AppError> {
    // In a real app, this would involve token invalidation
    Ok(StatusCode::OK)
} 