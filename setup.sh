#!/bin/bash

echo "🚀 Setup basic Rust Auth Service..."

if [ ! -f .env ]; then
    echo "📝 Creating .env..."
    cat > .env << EOF
POSTGRES_DB=auth_db
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_HOST=localhost
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:5432/${POSTGRES_DB}
JWT_SECRET=1234
REDIS_URL=redis://localhost:6379
RUST_LOG=rust_auth_service=debug,tower_http=debug,sqlx=debug
EOF
fi

echo "🐳 Init Docker Compose"
# Try V2 first, fallback to V1
if docker compose up -d 2>/dev/null; then
    echo "Using Docker Compose V2"
else
    echo "Using Docker Compose V1"
    docker-compose up -d
fi

echo "⏳ Waiting for services to be ready..."
sleep 15

echo "🔍 Checking services..."
until docker compose exec -T postgres pg_isready -U postgres; do
    echo "Waiting for PostgreSQL..."
    sleep 2
done

echo "🔄 Running migrations..."
source .env && sqlx migrate run

echo "🌱 Creating initial data..."
set -a
source .env
set +a
cargo run --bin seed -- --nocapture

echo "✅ Setup complete!"
echo "Execute: cargo run"
