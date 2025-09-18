# Test Documentation

## Running Tests

To run all unit tests:
```bash
cargo test
```

To run tests with output:
```bash
cargo test -- --nocapture
```

To run a specific test:
```bash
cargo test test_register_success
```

## Test Coverage

### Handler Tests (12 tests)
- **Register API Tests:**
  - `test_register_success` - Successful user registration
  - `test_register_empty_email` - Validation error for empty email
  - `test_register_empty_password` - Validation error for empty password  
  - `test_register_short_password` - Validation error for short password
  - `test_register_duplicate_email` - Conflict error for duplicate email

- **Login API Tests:**
  - `test_login_success` - Successful user login
  - `test_login_empty_email` - Validation error for empty email
  - `test_login_empty_password` - Validation error for empty password
  - `test_login_user_not_found` - Error for non-existent user
  - `test_login_wrong_password` - Error for incorrect password

- **Integration Tests:**
  - `test_jwt_token_validation` - JWT token creation and validation
  - `test_password_hashing` - bcrypt password hashing verification
  - `test_register_login_flow` - Full registration -> login flow

### JWT Service Tests (4 tests)
- `test_create_and_verify_token` - Token creation and verification
- `test_verify_invalid_token` - Invalid token rejection
- `test_verify_token_with_wrong_secret` - Wrong secret rejection
- `test_token_contains_correct_claims` - Token payload validation

### Repository Tests (8 tests)
- `test_create_user` - User creation in database
- `test_find_by_email_existing` - Find user by existing email
- `test_find_by_email_not_existing` - Handle non-existent email
- `test_find_by_id_existing` - Find user by existing ID
- `test_find_by_id_not_existing` - Handle non-existent ID
- `test_create_user_with_unique_email_constraint` - Email uniqueness enforcement
- `test_user_timestamps` - Proper timestamp creation

## Test Database

Tests use in-memory SQLite databases (`sqlite::memory:`) that are isolated per test to ensure no interference between tests.

## Test Structure

- Tests use the `test_helpers::create_test_app_state()` function to create isolated test environments
- Each test gets its own database instance
- Tests cover both success and error scenarios
- Integration tests verify the complete flow from HTTP request to database

## Example Test Run Output

```
running 24 tests
test handlers::tests::test_login_success ... ok
test handlers::tests::test_register_success ... ok
test jwt::tests::test_create_and_verify_token ... ok
test repository::tests::test_create_user ... ok
...
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
