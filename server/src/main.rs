pub mod errors;
pub mod db;
pub mod handlers;
pub mod models;

use axum::{
    routing::get,
    Router,
};
use handlers::{ 
    todos::{ all_todos, create_todo, delete_todo, update_todo, get_todo }, 
    auth::{ login, register, logout } 
};
use db::init_db;
use tower_http::cors::CorsLayer;
use axum::routing::{post, put, delete};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_pool = init_db().await.expect("failed to initialize database");

    let app = Router::new()
        // authentication
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/logout", post(logout))
        // todos
        .route("/api/todos", get(all_todos).post(create_todo))
        .route("/api/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .layer(CorsLayer::very_permissive())
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 