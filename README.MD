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
- ✅ **Role-based access control** (RBAC)
- ✅ **Structured logging** with tracing
- ✅ **Interactive API documentation** with SwaggerUI
- ✅ **Automated setup script** for easy deployment

## 🏗️ Architecture

```
src/
├── auth/           # JWT token generation and validation
├── config/         # Database and Redis configuration
├── db/            # Database operations and queries
├── errors/        # Custom error types and handling
├── handlers/      # HTTP request handlers
├── middleware/    # Authentication, CORS, and rate limiting
├── models/        # Data structures and database models
├── routes/        # API route definitions
├── services/      # Business logic (password hashing)
├── docs.rs        # OpenAPI documentation
└── bin/           # Utility binaries (seed script)
```

## 🛠️ Tech Stack

- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Authentication**: JWT (JSON Web Tokens)
- **Password Hashing**: Argon2
- **Documentation**: Utoipa (OpenAPI)
- **Error Handling**: Thiserror
- **Serialization**: Serde
- **Logging**: Tracing + Tracing Subscriber

## 📋 Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- SQLx CLI (optional, for manual migrations)

## 🚀 Quick Start

### 1. Clone and Setup

```bash
git clone <repository-url>
cd rust-auth-service

# Make setup script executable
chmod +x setup.sh

# Run automated setup
./setup.sh
```

### 2. Manual Setup (Alternative)

```bash
# Start services
docker compose up -d

# Wait for services to be ready
sleep 15

# Run migrations
sqlx migrate run

# Create initial data
cargo run --bin seed

# Start the service
cargo run
```

## 📚 API Documentation

Once the service is running, access the interactive API documentation:

- **Swagger UI**: `http://localhost:4000/swagger-ui`
- **OpenAPI JSON**: `http://localhost:4000/api-docs/openapi.json`

## 🔐 API Endpoints

### Public Endpoints

| Method | Endpoint       | Description           |
| ------ | -------------- | --------------------- |
| `GET`  | `/`            | Root                  |
| `GET`  | `/api/health`  | Health check endpoint |
| `POST` | `/api/login`   | User login            |
| `POST` | `/api/refresh` | Refresh access token  |
| `POST` | `/api/users`   | Create new user       |

### Protected Endpoints

| Method   | Endpoint          | Description    | Auth Required |
| -------- | ----------------- | -------------- | ------------- |
| `POST`   | `/api/logout`     | User logout    | ✅            |
| `GET`    | `/api/users/{id}` | Get user by ID | ✅            |
| `PATCH`  | `/api/users/{id}` | Update user    | ✅            |
| `DELETE` | `/api/users/{id}` | Delete user    | ✅            |

### Admin Endpoints

| Method | Endpoint     | Description     | Admin Role Required |
| ------ | ------------ | --------------- | ------------------- |
| `GET`  | `/api/admin` | Admin dashboard | ✅                  |

## 🔒 Authentication

The service uses JWT (JSON Web Tokens) for authentication:

1. **Login**: POST `/api/login` with email and password
2. **Access Token**: Valid for 15 minutes
3. **Refresh Token**: Valid for 7 days
4. **Authorization**: Include `Authorization: Bearer <token>` header

### Default Admin User

After running the seed script, you'll have access to:

- **Email**: `admin@example.com`
- **Password**: `admin123`
- **Role**: Admin

## 🛡️ Security Features

- **Password Hashing**: Argon2 with random salt generation
- **Token Revocation**: Blacklisted JWT tokens via Redis
- **Rate Limiting**: 10 requests per minute per IP
- **CORS**: Configurable cross-origin resource sharing
- **Input Validation**: Comprehensive request validation
- **Role-Based Access**: Fine-grained permission control

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Roles Table

```sql
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### User Roles Table

```sql
CREATE TABLE user_roles (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);
```

### Refresh Tokens Table

```sql
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token TEXT UNIQUE NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test categories
cargo test --test login_tests
cargo test --test users_tests
```

## 🔧 Configuration

### Environment Variables

| Variable       | Description                  | Default                   |
| -------------- | ---------------------------- | ------------------------- |
| `DATABASE_URL` | PostgreSQL connection string | Required                  |
| `REDIS_URL`    | Redis connection string      | Required                  |
| `JWT_SECRET`   | Secret key for JWT signing   | Required                  |
| `RUST_LOG`     | Logging level                | `rust_auth_service=debug` |

### Docker Services

- **PostgreSQL**: Main database (port 5432)
- **Redis**: Cache and token blacklisting (port 6379)
- **Redis Commander**: Web UI for Redis (port 8081)

## 🚀 Development

### Local Development

```bash
# Start services
docker compose up -d

# Run migrations
sqlx migrate run

# Create seed data
cargo run --bin seed

# Start development server
cargo run
```

### Database Migrations

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert migrations
sqlx migrate revert
```

## 🐳 Docker Development

Use the provided `compose.yml` for local development:

```bash
# Start all services
docker compose up -d

# View logs
docker compose logs -f

# Stop services
docker compose down
```

## 📁 Project Structure

```
rust-auth-service/
├── Cargo.toml              # Dependencies and project metadata
├── compose.yml             # Docker Compose configuration
├── setup.sh               # Automated setup script
├── migrations/             # Database migration files
├── src/
│   ├── auth/              # JWT authentication logic
│   ├── bin/               # Utility binaries (seed)
│   ├── config.rs          # Configuration management
│   ├── db/                # Database operations
│   ├── docs.rs            # OpenAPI documentation
│   ├── errors/            # Error handling
│   ├── handlers/          # HTTP request handlers
│   ├── lib.rs             # Library entry point
│   ├── main.rs            # Application entry point
│   ├── middleware/        # HTTP middleware
│   ├── models/            # Data models
│   ├── routes/            # API routing
│   └── services/          # Business logic services
└── tests/                 # Integration tests
```

## 🚀 Deployment

### Production Considerations

1. **Environment Variables**: Use secure, environment-specific configuration
2. **Database**: Use managed PostgreSQL service with connection pooling
3. **Redis**: Use managed Redis service or cluster
4. **HTTPS**: Always use HTTPS in production
5. **Rate Limiting**: Adjust rate limits based on your requirements
6. **Monitoring**: Add logging and monitoring for production use

### Docker Deployment

```bash
# Build production image
docker build -t rust-auth-service .

# Run production container
docker run -d \
  -p 4000:4000 \
  -e DATABASE_URL=your-db-url \
  -e REDIS_URL=your-redis-url \
  -e JWT_SECRET=your-secret \
  rust-auth-service
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For support and questions:

- Create an issue in the repository
- Check the API documentation at `/swagger-ui`
- Review the test files for usage examples

## 🔄 Changelog

### Version 1.0.0

- Initial release
- Complete authentication system
- User management with role-based access control
- JWT token system with refresh tokens
- Rate limiting and security features
- Comprehensive API documentation
- Automated setup and deployment scripts

## 🎯 Current Status

**Status**

The project includes:

- ✅ Complete authentication and authorization system
- ✅ Professional-grade security implementation
- ✅ Comprehensive testing and documentation
- ✅ Automated setup and deployment
- ✅ Production-ready architecture
