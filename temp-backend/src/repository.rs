use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::User;

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, email: &str, password_hash: &str) -> Result<User> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
        )
        .bind(&id)
        .bind(email)
        .bind(password_hash)
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::create_test_pool;

    #[tokio::test]
    async fn test_create_user() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let email = "test@example.com";
        let password_hash = "hashed_password";

        let user = repo.create_user(email, password_hash).await.unwrap();

        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
        assert!(!user.id.is_empty());
        assert!(user.created_at <= chrono::Utc::now());
        assert!(user.updated_at <= chrono::Utc::now());
    }

    #[tokio::test]
    async fn test_find_by_email_existing() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let email = "test@example.com";
        let password_hash = "hashed_password";

        // Create user first
        let created_user = repo.create_user(email, password_hash).await.unwrap();

        // Find by email
        let found_user = repo.find_by_email(email).await.unwrap();
        assert!(found_user.is_some());
        
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.email, created_user.email);
        assert_eq!(found_user.password_hash, created_user.password_hash);
    }

    #[tokio::test]
    async fn test_find_by_email_not_existing() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let result = repo.find_by_email("nonexistent@example.com").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_find_by_id_existing() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let email = "test@example.com";
        let password_hash = "hashed_password";

        // Create user first
        let created_user = repo.create_user(email, password_hash).await.unwrap();

        // Find by ID
        let found_user = repo.find_by_id(&created_user.id).await.unwrap();
        assert!(found_user.is_some());
        
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.email, created_user.email);
        assert_eq!(found_user.password_hash, created_user.password_hash);
    }

    #[tokio::test]
    async fn test_find_by_id_not_existing() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let result = repo.find_by_id("non-existent-id").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_create_user_with_unique_email_constraint() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let email = "test@example.com";
        let password_hash1 = "password1";
        let password_hash2 = "password2";

        // Create first user
        let _user1 = repo.create_user(email, password_hash1).await.unwrap();

        // Try to create second user with same email - should fail
        let result = repo.create_user(email, password_hash2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_user_timestamps() {
        let pool = create_test_pool().await.unwrap();
        let repo = UserRepository::new(pool);

        let before_creation = chrono::Utc::now();
        let user = repo.create_user("test@example.com", "password").await.unwrap();
        let after_creation = chrono::Utc::now();

        assert!(user.created_at >= before_creation);
        assert!(user.created_at <= after_creation);
        assert!(user.updated_at >= before_creation);
        assert!(user.updated_at <= after_creation);
        assert_eq!(user.created_at, user.updated_at); // Should be same on creation
    }
}
