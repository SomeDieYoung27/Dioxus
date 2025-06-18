use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
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
#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    Loading,
    Authenticated(User),
    Unauthenticated,
}

// Form models
#[derive(Debug, Clone, Default)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct TodoForm {
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