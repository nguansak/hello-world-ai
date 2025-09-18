use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use anyhow::{Result, anyhow};

use crate::models::Claims;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn create_token(&self, user_id: &str, email: &str) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to create token: {}", e))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| anyhow!("Invalid token: {}", e))?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_and_verify_token() {
        let jwt_service = JwtService::new("test-secret-key");
        let user_id = "test-user-id";
        let email = "test@example.com";

        // Create token
        let token = jwt_service.create_token(user_id, email).unwrap();
        assert!(!token.is_empty());

        // Verify token
        let claims = jwt_service.verify_token(&token).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert!(claims.exp > Utc::now().timestamp() as usize);
        assert!(claims.iat <= Utc::now().timestamp() as usize);
    }

    #[test]
    fn test_verify_invalid_token() {
        let jwt_service = JwtService::new("test-secret-key");
        let invalid_token = "invalid.token.here";

        let result = jwt_service.verify_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_token_with_wrong_secret() {
        let jwt_service1 = JwtService::new("secret1");
        let jwt_service2 = JwtService::new("secret2");

        // Create token with first service
        let token = jwt_service1.create_token("user-id", "test@example.com").unwrap();

        // Try to verify with second service (different secret)
        let result = jwt_service2.verify_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let jwt_service = JwtService::new("test-secret");
        let user_id = "user-123";
        let email = "user@example.com";

        let token = jwt_service.create_token(user_id, email).unwrap();
        let claims = jwt_service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        
        // Check that expiration is ~24 hours from now
        let now = Utc::now().timestamp() as usize;
        let expected_exp = now + (24 * 60 * 60); // 24 hours
        assert!(claims.exp > now);
        assert!(claims.exp <= expected_exp + 60); // Allow 1 minute variance
    }
}
