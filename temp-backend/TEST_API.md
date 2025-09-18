# Test Authentication API

## Test with curl commands

### 1. Test the main endpoint
```bash
curl http://localhost:3000/
```

### 2. Register a new user
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'
```

### 3. Login with the same user
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'
```

### 4. Get OpenAPI specification
```bash
curl http://localhost:3000/api-docs/openapi.json
```

### 5. Access Swagger UI
Open in browser: http://localhost:3000/swagger-ui

## Test with httpie (if you have it installed)

### Register
```bash
http POST localhost:3000/auth/register email=user@test.com password=mypassword
```

### Login
```bash
http POST localhost:3000/auth/login email=user@test.com password=mypassword
```

## Response Examples

### Successful Registration Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user_id": "uuid-here",
  "email": "test@example.com"
}
```

### Error Response:
```json
{
  "error": "validation_error",
  "message": "Email and password are required"
}
```
