use crate::{
    db::DB,
    errors::AppError,
    models::{CreateTodo, Todo, UpdateTodo, Priority},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// A dummy user ID for now. In a real app, this would come from an authenticated session.
const DUMMY_USER_ID: &str = "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11";

pub async fn all_todos(State(db): State<DB>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as!(
        Todo,
        r#"SELECT id as "id: Uuid", user_id as "user_id: Uuid", title, description, completed, priority as "priority: Priority", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM todos WHERE user_id = $1"#,
        DUMMY_USER_ID
    )
    .fetch_all(&db)
    .await?;
    Ok(Json(todos))
}

pub async fn get_todo(
    State(db): State<DB>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, AppError> {
    let id_str = id.to_string();
    let todo = sqlx::query_as!(
        Todo,
        r#"SELECT id as "id: Uuid", user_id as "user_id: Uuid", title, description, completed, priority as "priority: Priority", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM todos WHERE id = $1 AND user_id = $2"#,
        id_str,
        DUMMY_USER_ID
    )
    .fetch_one(&db)
    .await
    .map_err(|_| AppError::NotFound)?;
    Ok(Json(todo))
}

pub async fn create_todo(
    State(db): State<DB>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let id = Uuid::new_v4();
    let id_str = id.to_string();
    let now = Utc::now();
    let priority_str = match payload.priority {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
    };

    sqlx::query!(
        "INSERT INTO todos (id, user_id, title, description, priority, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        id_str,
        DUMMY_USER_ID,
        payload.title,
        payload.description,
        priority_str,
        now,
        now
    )
    .execute(&db)
    .await?;

    let todo = sqlx::query_as!(
        Todo,
        r#"SELECT id as "id: Uuid", user_id as "user_id: Uuid", title, description, completed, priority as "priority: Priority", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM todos WHERE id = $1"#,
        id_str
    )
    .fetch_one(&db)
    .await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo(
    State(db): State<DB>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let id_str = id.to_string();
    let todo = sqlx::query_as!(
        Todo,
        r#"SELECT id as "id: Uuid", user_id as "user_id: Uuid", title, description, completed, priority as "priority: Priority", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM todos WHERE id = $1 AND user_id = $2"#,
        id_str,
        DUMMY_USER_ID
    )
    .fetch_one(&db)
    .await
    .map_err(|_| AppError::NotFound)?;

    let title = payload.title.unwrap_or(todo.title);
    let description = payload.description.or(todo.description);
    let completed = payload.completed.unwrap_or(todo.completed);
    let priority = payload.priority.unwrap_or(todo.priority);
    let priority_str = match priority {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
    };
    let now = Utc::now();

    sqlx::query!(
        "UPDATE todos SET title = $1, description = $2, completed = $3, priority = $4, updated_at = $5 WHERE id = $6",
        title,
        description,
        completed,
        priority_str,
        now,
        id_str
    )
    .execute(&db)
    .await?;

    let updated_todo = sqlx::query_as!(
        Todo,
        r#"SELECT id as "id: Uuid", user_id as "user_id: Uuid", title, description, completed, priority as "priority: Priority", created_at as "created_at: DateTime<Utc>", updated_at as "updated_at: DateTime<Utc>" FROM todos WHERE id = $1"#,
        id_str
    )
    .fetch_one(&db)
    .await?;

    Ok(Json(updated_todo))
}

pub async fn delete_todo(
    State(db): State<DB>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let id_str = id.to_string();
    let rows_affected = sqlx::query!(
        "DELETE FROM todos WHERE id = $1 AND user_id = $2",
        id_str,
        DUMMY_USER_ID
    )
    .execute(&db)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
} 