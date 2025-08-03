# Rust Authentication Service

## Overview

This project is a robust authentication service built with Rust. It provides secure user authentication functionality backed by a PostgreSQL database and Redis for session management.

## Features

- ✅ **Secure user authentication** with JWT tokens
- ✅ **Password hashing** using Argon2
- ✅ **Access and refresh tokens** with automatic rotation
- ✅ **PostgreSQL database** integration with SQLx
- ✅ **Redis integration** for token revocation and caching
- ✅ **Rate limiting** middleware
- ✅ **CORS support** for web applications
- ✅ **Comprehensive test suite** with unit and integration tests
- ✅ **Containerized setup** with Docker Compose
- ✅ **Database migrations** with SQLx CLI

## Prerequisites

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install)

## Getting Started

### 1. Environment Setup

Create a `.env` file in the project root:

```bash
# Database Configuration
DATABASE_URL=postgres://postgres:postgres@localhost:5432/auth_db
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/auth_test_db

# Redis Configuration
REDIS_URL=redis://localhost:6379

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-here-make-it-long-and-random-at-least-32-characters

# PostgreSQL Configuration
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=auth_db
```

### 2. Setting up the Database

The project uses PostgreSQL as its database. You can start it using Docker Compose:

```bash
docker-compose up -d postgres
```

This will start a PostgreSQL container with the following configuration:

- **Container name**: rust_auth_pg
- **Username**: postgres
- **Password**: postgres
- **Database name**: auth_db
- **Port**: 5432 (mapped to host)

### 3. Running Database Migrations

```bash
# Install SQLx CLI if not already installed
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run
```

### 4. Starting the Full Stack

```bash
# Start all services (PostgreSQL, Redis, Redis Commander)
docker-compose up -d

# Or start only the database
docker-compose up -d postgres
```

## API Endpoints

### Authentication

- `POST /api/login` - User login
- `POST /api/logout` - User logout (requires authentication)
- `POST /api/refresh` - Refresh access token

### User Management

- `POST /api/users` - Create new user
- `GET /api/users/{user_id}` - Get user by ID
- `PATCH /api/users/{user_id}` - Update user
- `DELETE /api/users/{user_id}` - Delete user

## Development

### Running the Application

```bash
cargo run
```

The server will start on `http://localhost:4000`

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test login_tests
cargo test --test users_tests

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

The project includes comprehensive tests:

- **Unit tests**: Password hashing, JWT generation, data validation
- **Integration tests**: Full authentication flow, database operations
- **Helper functions**: Reusable test utilities

### Project Structure

```
├── src/
│   ├── auth/           # JWT authentication logic
│   ├── db/             # Database operations
│   ├── errors/         # Custom error types
│   ├── handlers/       # HTTP request handlers
│   ├── middleware/     # CORS, rate limiting, auth middleware
│   ├── models/         # Data structures
│   ├── routes/         # API route definitions
│   ├── services/       # Business logic (password hashing)
│   └── config.rs       # Configuration management
├── tests/
│   ├── helpers.rs      # Test utilities
│   ├── login_tests.rs  # Authentication tests
│   ├── users_tests.rs  # User model tests
│   └── main_tests.rs   # Basic functionality tests
├── migrations/         # Database schema migrations
├── compose.yml         # Docker Compose configuration
└── Cargo.toml          # Rust dependencies
```

## Security Features

- **Password Security**: Argon2 hashing with salt
- **JWT Tokens**: Secure token generation with expiration
- **Rate Limiting**: Protection against brute force attacks
- **CORS**: Configurable cross-origin resource sharing
- **Input Validation**: Comprehensive data validation
- **Token Revocation**: Redis-based token blacklisting

## Configuration

### Environment Variables

| Variable            | Description                  | Required |
| ------------------- | ---------------------------- | -------- |
| `DATABASE_URL`      | PostgreSQL connection string | Yes      |
| `REDIS_URL`         | Redis connection string      | Yes      |
| `JWT_SECRET`        | Secret key for JWT signing   | Yes      |
| `POSTGRES_USER`     | Database username            | Yes      |
| `POSTGRES_PASSWORD` | Database password            | Yes      |
| `POSTGRES_DB`       | Database name                | Yes      |

### Docker Services

- **PostgreSQL**: Main database
- **Redis**: Session storage and caching
- **Redis Commander**: Web UI for Redis management (http://localhost:8081)

## Data Persistence

- **Database**: PostgreSQL data persisted in Docker volume `pgdata`
- **Cache**: Redis data persisted in Docker volume `redisdata`
- **Migrations**: Version-controlled database schema changes

## Testing

### Test Environment Setup

For tests, the application automatically uses test-specific environment variables:

```rust
// In tests, set environment variables
unsafe {
    std::env::set_var("JWT_SECRET", "test-secret-key-for-testing-only");
}
```

### Running Different Test Types

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test "*"

# Specific test
cargo test should_hash_password_correctly
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

[MIT License](LICENSE)
