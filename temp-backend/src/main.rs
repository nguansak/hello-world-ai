mod database;
mod handlers;
mod jwt;
mod models;
mod repository;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
    response::Html,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;

use crate::{
    database::{create_pool, create_tables},
    handlers::{login, register},
    jwt::JwtService,
    models::{AuthResponse, ErrorResponse, LoginRequest, RegisterRequest},
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
    ),
    components(
        schemas(RegisterRequest, LoginRequest, AuthResponse, ErrorResponse)
    ),
    tags(
        (name = "auth", description = "Authentication API")
    ),
    info(
        title = "Authentication API",
        description = "A simple authentication API with JWT tokens",
        version = "1.0.0"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    // Create routes
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/api-docs/openapi.json", get(|| async {
            axum::Json(ApiDoc::openapi())
        }))
        .route("/swagger-ui", get(swagger_ui))
        .layer(cors)
        .with_state(app_state);

    println!("Server starting on http://localhost:3000");
    println!("API documentation available at http://localhost:3000/api-docs/openapi.json");
    println!("Swagger UI available at http://localhost:3000/swagger-ui");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "message": "Authentication API is running!",
        "endpoints": {
            "register": "POST /auth/register",
            "login": "POST /auth/login",
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
  <title>Authentication API Documentation</title>
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
