use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use dotenvy::dotenv;
use tracing::info;

pub type DB = SqlitePool;

pub async fn init_db() -> Result<DB, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    info!("Connecting to database: {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    info!("Running migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Migrations complete");

    Ok(pool)
} 