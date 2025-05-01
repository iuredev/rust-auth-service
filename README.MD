# Rust Authentication Service

## Overview

This project is a robust authentication service built with Rust. It provides secure user authentication functionality backed by a PostgreSQL database.

## Features

- Secure user authentication (In Progress)
- PostgreSQL database integration with SQLx
- Containerized setup with Docker Compose
- Easy local development environment setup

## Prerequisites

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install)

## Getting Started

### Setting up the Database

The project uses PostgreSQL as its database. You can start it using Docker Compose:

```bash
docker-compose up -d postgres
```

This will start a PostgreSQL container with the following configuration:
You can configure as you want, but the default configuration that I used is as follows: (Locally)

- **Container name**: rust_auth_pg
- **Username**: postgres
- **Password**: postgres
- **Database name**: auth_db
- **Port**: 5432 (mapped to host)

### Database Connection String

For your Rust application, use the following connection string:

```
postgres://postgres:postgres@localhost:5432/auth_db
```

## Project Structure

- `src/` - Rust source code
- `tests/` - Rust test files
- `migrations/` - Database migration files
- `compose.yml` - Docker Compose configuration

## Development

To run the full stack locally:

```bash
docker-compose up -d
```

To run only the database:

```bash
docker-compose up -d postgres
```

To stop all services:

```bash
docker-compose down
```

## Running the Rust Application

```bash
cargo run
```

## Data Persistence

Database data is persisted using a Docker volume named `pgdata`, which ensures your data remains intact between container restarts.

## Configuration

The PostgreSQL database is configured with the following environment variables:

- `POSTGRES_USER`: postgres
- `POSTGRES_PASSWORD`: postgres
- `POSTGRES_DB`: auth_db

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
