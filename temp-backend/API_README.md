# Authentication API

A simple Rust backend with authentication using SQLite, JWT tokens, and Swagger documentation.

## Features

- User registration with email and password
- User login with JWT token generation
- Password hashing using bcrypt
- SQLite database for data persistence
- Swagger UI for API documentation
- CORS support

## API Endpoints

### Authentication

#### POST /auth/register
Register a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response (201 Created):**
```json
{
  "token": "jwt_token_here",
  "user_id": "uuid",
  "email": "user@example.com"
}
```

#### POST /auth/login
Login with existing credentials.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response (200 OK):**
```json
{
  "token": "jwt_token_here",
  "user_id": "uuid",
  "email": "user@example.com"
}
```

### Documentation

#### GET /swagger-ui
Access the Swagger UI for interactive API documentation.

#### GET /api-docs/openapi.json
Get the OpenAPI specification in JSON format.

## Running the Application

1. Install Rust and Cargo
2. Clone this repository
3. Run the application:
   ```bash
   cargo run
   ```
4. The server will start on `http://localhost:3000`
5. Access Swagger UI at `http://localhost:3000/swagger-ui`

## Environment Variables

You should change the JWT secret in production. Consider using environment variables:

```rust
// In production, use:
let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
let jwt_service = Arc::new(JwtService::new(&jwt_secret));
```

## Database

The application uses SQLite with a file named `app.db` in the project root. The database schema is automatically created on startup.

## Security Considerations

- Passwords are hashed using bcrypt
- JWT tokens expire after 24 hours
- Input validation is performed on all endpoints
- CORS is configured for cross-origin requests

## Dependencies

- `axum` - Web framework
- `sqlx` - Database toolkit
- `bcrypt` - Password hashing
- `jsonwebtoken` - JWT token handling
- `utoipa` - OpenAPI documentation
- `serde` - Serialization/deserialization
