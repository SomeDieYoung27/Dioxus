use crate::models::{Todo, User};
use gloo_storage::{LocalStorage, Storage};
use uuid::Uuid;

const TODOS_STORAGE_KEY: &str = "dioxus_todos";
const USER_STORAGE_KEY: &str = "dioxus_user";

// Todo storage utilities
pub fn save_todos(todos: &[Todo]) -> Result<(), String> {
    LocalStorage::set(TODOS_STORAGE_KEY, todos)
        .map_err(|e| format!("Failed to save todos: {}", e))
}

pub fn load_todos() -> Result<Vec<Todo>, String> {
    LocalStorage::get(TODOS_STORAGE_KEY)
        .unwrap_or_else(|_| Ok(Vec::new()))
        .map_err(|e| format!("Failed to load todos: {}", e))
}

pub fn clear_todos() -> Result<(), String> {
    LocalStorage::delete(TODOS_STORAGE_KEY);
    Ok(())
}

// User authentication utilities
pub fn save_user(user: &User) -> Result<(), String> {
    LocalStorage::set(USER_STORAGE_KEY, user)
        .map_err(|e| format!("Failed to save user: {}", e))
}

pub fn load_user() -> Option<User> {
    LocalStorage::get(USER_STORAGE_KEY).ok()
}

pub fn clear_user() {
    LocalStorage::delete(USER_STORAGE_KEY);
}

// Mock authentication function (in real app, this would call an API)
pub fn authenticate_user(username: &str, password: &str) -> Result<User, String> {
    // Simple mock authentication
    if username.len() >= 3 && password.len() >= 6 {
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: format!("{}@example.com", username),
            created_at: chrono::Utc::now(),
        };
        save_user(&user)?;
        Ok(user)
    } else {
        Err("Invalid credentials. Username must be at least 3 characters and password at least 6 characters.".to_string())
    }
}

// Validation utilities
pub fn validate_todo_title(title: &str) -> Result<(), String> {
    if title.trim().is_empty() {
        Err("Title cannot be empty".to_string())
    } else if title.len() > 100 {
        Err("Title cannot be longer than 100 characters".to_string())
    } else {
        Ok(())
    }
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
} 