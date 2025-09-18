# Authentication API - System Documentation

## Overview
This document provides detailed system architecture documentation for the Authentication API built with Rust/Axum framework. The system provides user registration and login functionality with JWT token-based authentication.

## System Architecture

### Technology Stack
- **Backend Framework**: Axum (Rust)
- **Database**: SQLite
- **Authentication**: JWT (JSON Web Tokens)
- **Password Hashing**: bcrypt
- **Documentation**: OpenAPI/Swagger

---

## Entity Relationship Diagram

```mermaid
erDiagram
    USERS {
        string id PK "UUID"
        string email UK "Unique email address"
        string password_hash "Bcrypt hashed password"
        datetime created_at "Account creation timestamp"
        datetime updated_at "Last update timestamp"
    }
    
    JWT_TOKENS ||--|| USERS : "belongs_to"
    JWT_TOKENS {
        string sub "User ID (from Users.id)"
        string email "User email"
        number exp "Expiration timestamp"
        number iat "Issued at timestamp"
    }
```

### Entity Descriptions

#### Users Table
- **id**: Primary key using UUID format
- **email**: Unique email address for user identification
- **password_hash**: Securely hashed password using bcrypt
- **created_at**: Timestamp when the account was created
- **updated_at**: Timestamp when the account was last modified

#### JWT Tokens (Virtual Entity)
- **sub**: Subject field containing the user ID
- **email**: User's email address embedded in token
- **exp**: Token expiration time (Unix timestamp)
- **iat**: Token issued at time (Unix timestamp)

---

## Sequence Diagrams

### User Registration Flow

```mermaid
sequenceDiagram
    participant Client
    participant API as Axum API
    participant Handler as Register Handler
    participant Validator as Input Validator
    participant Repo as User Repository
    participant DB as SQLite Database
    participant Hash as bcrypt Service
    participant JWT as JWT Service

    Client->>API: POST /auth/register
    Note over Client,API: {email, password}
    
    API->>Handler: route request
    Handler->>Validator: validate input
    
    alt Email or password empty
        Validator-->>Handler: validation error
        Handler-->>API: 400 Bad Request
        API-->>Client: {error: "validation_error"}
    else Password too short
        Validator-->>Handler: validation error
        Handler-->>API: 400 Bad Request
        API-->>Client: {error: "validation_error"}
    else Valid input
        Validator-->>Handler: validation passed
        
        Handler->>Repo: find_by_email(email)
        Repo->>DB: SELECT * FROM users WHERE email = ?
        DB-->>Repo: query result
        
        alt User already exists
            Repo-->>Handler: Some(user)
            Handler-->>API: 409 Conflict
            API-->>Client: {error: "email_exists"}
        else User doesn't exist
            Repo-->>Handler: None
            
            Handler->>Hash: hash(password)
            Hash-->>Handler: password_hash
            
            Handler->>Repo: create_user(email, password_hash)
            Repo->>DB: INSERT INTO users...
            DB-->>Repo: new user record
            Repo-->>Handler: User object
            
            Handler->>JWT: create_token(user_id, email)
            JWT-->>Handler: JWT token
            
            Handler-->>API: 201 Created
            API-->>Client: {token, user_id, email}
        end
    end
```

### User Login Flow

```mermaid
sequenceDiagram
    participant Client
    participant API as Axum API
    participant Handler as Login Handler
    participant Validator as Input Validator
    participant Repo as User Repository
    participant DB as SQLite Database
    participant Hash as bcrypt Service
    participant JWT as JWT Service

    Client->>API: POST /auth/login
    Note over Client,API: {email, password}
    
    API->>Handler: route request
    Handler->>Validator: validate input
    
    alt Email or password empty
        Validator-->>Handler: validation error
        Handler-->>API: 400 Bad Request
        API-->>Client: {error: "validation_error"}
    else Valid input
        Validator-->>Handler: validation passed
        
        Handler->>Repo: find_by_email(email)
        Repo->>DB: SELECT * FROM users WHERE email = ?
        DB-->>Repo: query result
        
        alt User not found
            Repo-->>Handler: None
            Handler-->>API: 401 Unauthorized
            API-->>Client: {error: "invalid_credentials"}
        else User found
            Repo-->>Handler: Some(user)
            
            Handler->>Hash: verify(password, user.password_hash)
            Hash-->>Handler: verification result
            
            alt Password incorrect
                Hash-->>Handler: false
                Handler-->>API: 401 Unauthorized
                API-->>Client: {error: "invalid_credentials"}
            else Password correct
                Hash-->>Handler: true
                
                Handler->>JWT: create_token(user_id, email)
                JWT-->>Handler: JWT token
                
                Handler-->>API: 200 OK
                API-->>Client: {token, user_id, email}
            end
        end
    end
```

### JWT Token Verification Flow (for protected routes)

```mermaid
sequenceDiagram
    participant Client
    participant API as Axum API
    participant Middleware as Auth Middleware
    participant JWT as JWT Service
    participant Repo as User Repository
    participant DB as SQLite Database
    participant Handler as Protected Handler

    Client->>API: Request to protected endpoint
    Note over Client,API: Authorization: Bearer <token>
    
    API->>Middleware: extract token from header
    
    alt No token provided
        Middleware-->>API: 401 Unauthorized
        API-->>Client: {error: "missing_token"}
    else Token provided
        Middleware->>JWT: verify_token(token)
        
        alt Invalid/expired token
            JWT-->>Middleware: verification failed
            Middleware-->>API: 401 Unauthorized
            API-->>Client: {error: "invalid_token"}
        else Valid token
            JWT-->>Middleware: Claims{user_id, email}
            
            Middleware->>Repo: find_by_id(user_id)
            Repo->>DB: SELECT * FROM users WHERE id = ?
            DB-->>Repo: user record
            
            alt User not found
                Repo-->>Middleware: None
                Middleware-->>API: 401 Unauthorized
                API-->>Client: {error: "user_not_found"}
            else User found
                Repo-->>Middleware: Some(user)
                Middleware->>Handler: proceed with user context
                Handler-->>API: response
                API-->>Client: protected resource
            end
        end
    end
```

---

## API Endpoints

### Authentication Endpoints

| Method | Endpoint | Description | Request Body | Response |
|--------|----------|-------------|--------------|----------|
| POST | `/auth/register` | Register new user | `RegisterRequest` | `AuthResponse` |
| POST | `/auth/login` | Login user | `LoginRequest` | `AuthResponse` |

### Utility Endpoints

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/` | Health check | System info JSON |
| GET | `/api-docs/openapi.json` | OpenAPI spec | OpenAPI JSON |
| GET | `/swagger-ui` | Swagger UI | HTML page |

---

## Data Models

### Request Models

```rust
// Registration request
{
  "email": "user@example.com",
  "password": "securepassword123"
}

// Login request
{
  "email": "user@example.com", 
  "password": "securepassword123"
}
```

### Response Models

```rust
// Successful authentication response
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com"
}

// Error response
{
  "error": "validation_error",
  "message": "Email and password are required"
}
```

---

## Security Features

### Password Security
- **Hashing Algorithm**: bcrypt with default cost (12)
- **Minimum Length**: 6 characters
- **Storage**: Only hashed passwords stored, never plain text

### JWT Security
- **Algorithm**: HS256 (HMAC with SHA-256)
- **Claims**: User ID, email, expiration, issued at
- **Secret Key**: Configurable (should be environment variable in production)

### Input Validation
- **Email Format**: Basic email validation
- **Password Requirements**: Minimum 6 characters
- **Sanitization**: Input trimming and validation

### Database Security
- **Unique Constraints**: Email addresses must be unique
- **Parameterized Queries**: Protection against SQL injection
- **Connection Pooling**: Managed SQLite connections

---

## Error Handling

### HTTP Status Codes

| Status Code | Description | Example Scenarios |
|-------------|-------------|-------------------|
| 200 | OK | Successful login |
| 201 | Created | Successful registration |
| 400 | Bad Request | Invalid input data |
| 401 | Unauthorized | Invalid credentials, expired token |
| 409 | Conflict | Email already exists |
| 500 | Internal Server Error | Database errors, hashing failures |

### Error Types

| Error Code | Description | HTTP Status |
|------------|-------------|-------------|
| `validation_error` | Input validation failed | 400 |
| `email_exists` | Email already registered | 409 |
| `invalid_credentials` | Wrong email or password | 401 |
| `database_error` | Database operation failed | 500 |
| `hash_error` | Password hashing failed | 500 |
| `token_error` | JWT generation/verification failed | 500 |

---

## Development Notes

### Testing Coverage
- Unit tests for all handlers
- Repository layer tests
- JWT service tests
- Integration tests for complete flows

### Performance Considerations
- Connection pooling for database
- Async/await for non-blocking operations
- Efficient password hashing with bcrypt

### Production Readiness
- Environment-based configuration needed
- Logging and monitoring to be added
- Rate limiting for authentication endpoints
- HTTPS enforcement
- Database migration system
