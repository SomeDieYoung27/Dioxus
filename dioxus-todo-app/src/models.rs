use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt::{Display, Formatter, Result as FmtResult};

// Todo item model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

// User model for authentication
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

// Authentication state
#[derive(Debug, Default, Clone, PartialEq)]
pub enum AuthState {
    #[default]
    Unknown,
    Authenticated(User),
    Guest,
    Failed,
}

// Form models
#[derive(Debug, Clone, Default)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TodoForm {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub priority: Priority,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl Todo {
    pub fn new(user_id: Uuid, title: String, description: Option<String>, priority: Priority) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
            priority,
        }
    }
    
    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
        self.updated_at = Utc::now();
    }
} 