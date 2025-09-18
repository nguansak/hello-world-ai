# GitHub Copilot Instructions for Hello World AI

## Architecture Overview

This is a full-stack web application with separated frontend and backend:

- **Frontend**: `temp-frontend/` - React + Vite + Tailwind CSS + Storybook
- **Backend**: `temp-backend/` - Rust (Axum) + SQLite + JWT authentication

### Key Design Patterns

**Frontend Component Development**:
- Components in `src/components/` with co-located `.stories.jsx`, `.test.jsx`, and `.specs.md` files
- Tailwind CSS for styling with utility-first approach
- Storybook for isolated component development and documentation
- Thai language documentation in Storybook stories for team collaboration

**Backend Architecture**:
- Modular Rust structure: `handlers`, `models`, `repository`, `database`, `jwt` modules
- Axum web framework with structured routing in `lib.rs`
- SQLite database with SQLx for type-safe queries
- JWT-based authentication with bcrypt password hashing
- OpenAPI/Swagger documentation auto-generated via `utoipa`

## Development Workflows

### Frontend Development
```bash
# Start frontend dev server with HMR
cd temp-frontend && npm run dev

# Component development with Storybook
npm run storybook  # Runs on localhost:6006

# Build for production
npm run build
```

### Backend Development
```bash
# Start backend API server
cd temp-backend && cargo run  # Runs on localhost:3000

# Access API documentation
# Swagger UI: http://localhost:3000/swagger-ui
# OpenAPI spec: http://localhost:3000/api-docs/openapi.json
```

### Project Structure Conventions

**Frontend Components**:
```
src/components/ComponentName.jsx
src/components/ComponentName.stories.jsx  # Storybook stories
src/components/ComponentName.test.jsx     # Tests
src/components/ComponentName.specs.md     # Specifications
```

**Backend Modules**:
```
src/main.rs           # Entry point
src/lib.rs            # App creation and routing
src/handlers.rs       # HTTP request handlers
src/models.rs         # Data models with Serde + SQLx + OpenAPI
src/repository.rs     # Database operations
src/database.rs       # Database setup and migrations
src/jwt.rs           # JWT token management
```

## Technology-Specific Guidelines

### React + Vite
- Use Vite's fast HMR for development
- Import CSS through `index.css` for Tailwind
- Prefer functional components with hooks
- Component props should be documented in Storybook stories

### Tailwind CSS
- Configured via `@tailwindcss/vite` plugin in `vite.config.js`
- Use utility classes for responsive design (`grid grid-cols-1 md:grid-cols-2`)
- Color scheme: Purple primary (`purple-600`), gray neutrals, semantic colors

### Rust Backend
- Use `#[derive(ToSchema)]` on models for OpenAPI documentation
- Authentication via JWT tokens in `Authorization: Bearer <token>` header
- Database operations return `Result<T, sqlx::Error>`
- CORS configured for cross-origin frontend requests

### Storybook Integration
- Stories include Thai language documentation
- Use multiple story variants: Default, Compact, WithContainer, DarkTheme, etc.
- Test responsive behavior with different container sizes
- Include real-world usage examples in stories

## Authentication & API Patterns

**JWT Authentication Flow**:
1. `POST /auth/register` - Create account with email/password
2. `POST /auth/login` - Login returns JWT token
3. Include `Authorization: Bearer <token>` for protected routes
4. `GET /profile` and `PUT /profile` require authentication

**API Response Patterns**:
```rust
// Success responses
AuthResponse { token, user_id, email }
UserProfile { id, email, first_name, ... }

// Error responses
ErrorResponse { error: "auth_error", message: "Invalid credentials" }
```

## Database & State Management

**User Model Structure**:
- UUID primary keys for users
- Membership system with levels (Bronze, Silver, Gold, Platinum)
- Points-based system for user engagement
- Optional profile fields (first_name, last_name, phone, membership_id)

**Database Operations**:
- SQLite with file `app.db` in backend root
- Auto-migration on app startup via `create_tables()`
- Use SQLx compile-time checked queries
- Password hashing with bcrypt before storage

## Development Notes

- **Path Conventions**: Backend should eventually move from `temp-backend/` to `myapp-backend/`
- **Local Development**: Always check if dev server is running before accessing localhost
- **Component Testing**: Use Storybook for visual testing and component isolation
- **API Testing**: Swagger UI provides interactive API testing interface
- **Security**: JWT secret should be environment variable in production

## Common Integration Points

**Frontend ↔ Backend Communication**:
- Frontend connects to `localhost:3000` API
- CORS configured for development cross-origin requests
- Authentication state managed via JWT tokens
- API responses follow consistent JSON structure

**Component Development**:
- Build components in isolation using Storybook
- Test various states and responsive behaviors
- Document usage patterns in `.specs.md` files
- Include accessibility considerations in component design
