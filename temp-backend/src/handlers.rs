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
