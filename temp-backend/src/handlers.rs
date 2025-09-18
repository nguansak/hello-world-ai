use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{
    models::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest},
    AppState,
};

/// Register a new user
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 409, description = "Email already exists", body = ErrorResponse)
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, ResponseJson<AuthResponse>), (StatusCode, ResponseJson<ErrorResponse>)> {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                error: "validation_error".to_string(),
                message: "Email and password are required".to_string(),
            }),
        ));
    }

    if payload.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                error: "validation_error".to_string(),
                message: "Password must be at least 6 characters long".to_string(),
            }),
        ));
    }

    // Check if user already exists
    match state.user_repo.find_by_email(&payload.email).await {
        Ok(Some(_)) => {
            return Err((
                StatusCode::CONFLICT,
                ResponseJson(ErrorResponse {
                    error: "email_exists".to_string(),
                    message: "Email already exists".to_string(),
                }),
            ));
        }
        Ok(None) => {}
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "database_error".to_string(),
                    message: "Failed to check existing user".to_string(),
                }),
            ));
        }
    }

    // Hash password
    let password_hash = match hash(payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "hash_error".to_string(),
                    message: "Failed to hash password".to_string(),
                }),
            ));
        }
    };

    // Create user
    let user = match state.user_repo.create_user(&payload.email, &password_hash).await {
        Ok(user) => user,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "database_error".to_string(),
                    message: "Failed to create user".to_string(),
                }),
            ));
        }
    };

    // Generate JWT token
    let token = match state.jwt_service.create_token(&user.id, &user.email) {
        Ok(token) => token,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "token_error".to_string(),
                    message: "Failed to generate token".to_string(),
                }),
            ));
        }
    };

    Ok((
        StatusCode::CREATED,
        ResponseJson(AuthResponse {
            token,
            user_id: user.id,
            email: user.email,
        }),
    ))
}

/// Login user
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<AuthResponse>, (StatusCode, ResponseJson<ErrorResponse>)> {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                error: "validation_error".to_string(),
                message: "Email and password are required".to_string(),
            }),
        ));
    }

    // Find user by email
    let user = match state.user_repo.find_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                ResponseJson(ErrorResponse {
                    error: "invalid_credentials".to_string(),
                    message: "Invalid email or password".to_string(),
                }),
            ));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "database_error".to_string(),
                    message: "Failed to find user".to_string(),
                }),
            ));
        }
    };

    // Verify password
    let is_valid = match verify(&payload.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "verification_error".to_string(),
                    message: "Failed to verify password".to_string(),
                }),
            ));
        }
    };

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            ResponseJson(ErrorResponse {
                error: "invalid_credentials".to_string(),
                message: "Invalid email or password".to_string(),
            }),
        ));
    }

    // Generate JWT token
    let token = match state.jwt_service.create_token(&user.id, &user.email) {
        Ok(token) => token,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    error: "token_error".to_string(),
                    message: "Failed to generate token".to_string(),
                }),
            ));
        }
    };

    Ok(ResponseJson(AuthResponse {
        token,
        user_id: user.id,
        email: user.email,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{LoginRequest, RegisterRequest},
        test_helpers::create_test_app_state,
    };
    use axum::{
        extract::{Json, State},
        http::StatusCode,
    };

    #[tokio::test]
    async fn test_register_success() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = register(State(app_state), Json(request)).await;
        
        assert!(result.is_ok());
        let (status, response) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(response.email, "test@example.com");
        assert!(!response.token.is_empty());
        assert!(!response.user_id.is_empty());
    }

    #[tokio::test]
    async fn test_register_empty_email() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = RegisterRequest {
            email: "".to_string(),
            password: "password123".to_string(),
        };

        let result = register(State(app_state), Json(request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error, "validation_error");
        assert_eq!(response.message, "Email and password are required");
    }

    #[tokio::test]
    async fn test_register_empty_password() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "".to_string(),
        };

        let result = register(State(app_state), Json(request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error, "validation_error");
        assert_eq!(response.message, "Email and password are required");
    }

    #[tokio::test]
    async fn test_register_short_password() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "123".to_string(),
        };

        let result = register(State(app_state), Json(request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error, "validation_error");
        assert_eq!(response.message, "Password must be at least 6 characters long");
    }

    #[tokio::test]
    async fn test_register_duplicate_email() {
        let app_state = create_test_app_state().await.unwrap();
        
        // First registration
        let request1 = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let _ = register(State(app_state.clone()), Json(request1)).await.unwrap();

        // Second registration with same email
        let request2 = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password456".to_string(),
        };
        let result = register(State(app_state), Json(request2)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(response.error, "email_exists");
        assert_eq!(response.message, "Email already exists");
    }

    #[tokio::test]
    async fn test_login_success() {
        let app_state = create_test_app_state().await.unwrap();
        
        // First register a user
        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let _ = register(State(app_state.clone()), Json(register_request)).await.unwrap();

        // Then login
        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let result = login(State(app_state), Json(login_request)).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.email, "test@example.com");
        assert!(!response.token.is_empty());
        assert!(!response.user_id.is_empty());
    }

    #[tokio::test]
    async fn test_login_empty_email() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = LoginRequest {
            email: "".to_string(),
            password: "password123".to_string(),
        };

        let result = login(State(app_state), Json(request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error, "validation_error");
        assert_eq!(response.message, "Email and password are required");
    }

    #[tokio::test]
    async fn test_login_empty_password() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "".to_string(),
        };

        let result = login(State(app_state), Json(request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.error, "validation_error");
        assert_eq!(response.message, "Email and password are required");
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let app_state = create_test_app_state().await.unwrap();
        
        let request = LoginRequest {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login(State(app_state), Json(request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(response.error, "invalid_credentials");
        assert_eq!(response.message, "Invalid email or password");
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let app_state = create_test_app_state().await.unwrap();
        
        // First register a user
        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let _ = register(State(app_state.clone()), Json(register_request)).await.unwrap();

        // Then login with wrong password
        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };
        let result = login(State(app_state), Json(login_request)).await;
        
        assert!(result.is_err());
        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(response.error, "invalid_credentials");
        assert_eq!(response.message, "Invalid email or password");
    }

    #[tokio::test]
    async fn test_jwt_token_validation() {
        let app_state = create_test_app_state().await.unwrap();
        
        // Register and get token
        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let (_, register_response) = register(State(app_state.clone()), Json(register_request)).await.unwrap();

        // Verify token can be decoded
        let claims = app_state.jwt_service.verify_token(&register_response.token);
        assert!(claims.is_ok());
        
        let claims = claims.unwrap();
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.sub, register_response.user_id);
    }

    #[tokio::test]
    async fn test_password_hashing() {
        let app_state = create_test_app_state().await.unwrap();
        
        // Register a user
        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let (_, register_response) = register(State(app_state.clone()), Json(register_request)).await.unwrap();

        // Verify password is hashed in database
        let user = app_state.user_repo.find_by_email("test@example.com").await.unwrap().unwrap();
        assert_ne!(user.password_hash, "password123"); // Should be hashed, not plain text
        assert!(user.password_hash.starts_with("$2b$")); // bcrypt hash format
    }

    #[tokio::test]
    async fn test_register_login_flow() {
        let app_state = create_test_app_state().await.unwrap();
        
        // Register
        let register_request = RegisterRequest {
            email: "user@example.com".to_string(),
            password: "mypassword123".to_string(),
        };
        let (register_status, register_response) = register(State(app_state.clone()), Json(register_request)).await.unwrap();
        
        assert_eq!(register_status, StatusCode::CREATED);
        assert_eq!(register_response.email, "user@example.com");
        
        // Login
        let login_request = LoginRequest {
            email: "user@example.com".to_string(),
            password: "mypassword123".to_string(),
        };
        let login_response = login(State(app_state), Json(login_request)).await.unwrap();
        
        assert_eq!(login_response.email, "user@example.com");
        assert_eq!(login_response.user_id, register_response.user_id); // Same user ID
        // Different tokens (new token generated on login)
        assert_ne!(login_response.token, register_response.token);
    }
}
