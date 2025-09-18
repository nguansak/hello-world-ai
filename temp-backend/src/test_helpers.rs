use crate::{database::create_tables, jwt::JwtService, repository::UserRepository, AppState};
use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::sync::Arc;

pub async fn create_test_pool() -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    
    create_tables(&pool).await?;
    Ok(pool)
}

pub async fn create_test_app_state() -> Result<AppState> {
    let pool = create_test_pool().await?;
    let user_repo = Arc::new(UserRepository::new(pool));
    let jwt_service = Arc::new(JwtService::new("test-secret-key"));
    
    Ok(AppState {
        user_repo,
        jwt_service,
    })
}
