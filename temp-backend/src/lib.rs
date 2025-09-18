pub mod database;
pub mod handlers;
pub mod jwt;
pub mod models;
pub mod repository;

#[cfg(test)]
mod test_helpers;

use axum::{
    http::Method,
    routing::{get, post, put},
    Router,
    response::Html,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;

use crate::{
    database::{create_pool, create_tables},
    handlers::{login, register, get_profile, update_profile},
    jwt::JwtService,
    models::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest, UserProfile, UpdateProfileRequest},
    repository::UserRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<UserRepository>,
    pub jwt_service: Arc<JwtService>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::register,
        handlers::login,
        handlers::get_profile,
        handlers::update_profile,
    ),
    components(
        schemas(RegisterRequest, LoginRequest, AuthResponse, ErrorResponse, UserProfile, UpdateProfileRequest)
    ),
    tags(
        (name = "auth", description = "Authentication API"),
        (name = "profile", description = "User Profile API")
    ),
    info(
        title = "User Management API",
        description = "A complete user management API with authentication and profiles",
        version = "1.0.0"
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

use utoipa::Modify;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

pub async fn create_app() -> Result<Router, Box<dyn std::error::Error>> {
    // Initialize database
    let pool = create_pool().await?;
    create_tables(&pool).await?;

    // Initialize services
    let user_repo = Arc::new(UserRepository::new(pool));
    let jwt_service = Arc::new(JwtService::new("your-secret-key-change-this-in-production"));

    let app_state = AppState {
        user_repo,
        jwt_service,
    };

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers(Any)
        .allow_origin(Any);

    // Create routes
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/profile", get(get_profile))
        .route("/profile", put(update_profile))
        .route("/api-docs/openapi.json", get(|| async {
            axum::Json(ApiDoc::openapi())
        }))
        .route("/swagger-ui", get(swagger_ui))
        .layer(cors)
        .with_state(app_state);

    Ok(app)
}

async fn hello_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "message": "User Management API is running!",
        "endpoints": {
            "register": "POST /auth/register",
            "login": "POST /auth/login",
            "get_profile": "GET /profile",
            "update_profile": "PUT /profile",
            "api_docs": "GET /api-docs/openapi.json",
            "swagger_ui": "GET /swagger-ui"
        }
    }))
}

async fn swagger_ui() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>User Management API Documentation</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui.css" />
</head>
<body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@4.15.5/swagger-ui-bundle.js" crossorigin></script>
<script>
  window.onload = () => {
    window.ui = SwaggerUIBundle({
      url: '/api-docs/openapi.json',
      dom_id: '#swagger-ui',
      presets: [
        SwaggerUIBundle.presets.apis,
        SwaggerUIBundle.presets.standalone,
      ],
    });
  };
</script>
</body>
</html>
"#)
}
