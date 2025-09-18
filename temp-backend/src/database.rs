use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use anyhow::Result;

pub async fn create_pool() -> Result<SqlitePool> {
    // Create database file if it doesn't exist
    let database_url = "sqlite:./app.db";
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn create_tables(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            first_name TEXT,
            last_name TEXT,
            phone TEXT,
            membership_id TEXT,
            membership_level TEXT NOT NULL DEFAULT 'Bronze',
            points INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
