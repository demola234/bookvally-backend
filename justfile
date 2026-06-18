# Book Vally — dev task runner
# Install just: cargo install just

set dotenv-load := true
set dotenv-filename := ".dev.env"

# default
default:
    @just --list

# infrastructure

# Start all local services (postgres, redis, kafka, otel-collector)
up:
    docker compose up -d
    @echo "Waiting for postgres..."
    @until docker compose exec postgres pg_isready -U bookvally > /dev/null 2>&1; do sleep 1; done
    @echo "Services ready."

# Stop all local services
down:
    docker compose down

# Wipe all local data volumes
nuke:
    docker compose down -v

# Tail service logs
logs service="":
    docker compose logs -f {{ service }}

# database

# Run all pending migrations
migrate:
    sqlx migrate run --database-url "$DATABASE_URL"

# Roll back the last migration
migrate-revert:
    sqlx migrate revert --database-url "$DATABASE_URL"

# Show migration status
migrate-info:
    sqlx migrate info --database-url "$DATABASE_URL"

# Prepare sqlx offline query cache (run after changing queries)
prepare:
    cargo sqlx prepare --workspace -- --all-targets

# Drop and recreate the database, then re-run all migrations
db-reset:
    sqlx database drop -y --database-url "$DATABASE_URL"
    sqlx database create --database-url "$DATABASE_URL"
    just migrate

# development

# Run the API server with auto-reload (requires cargo-watch)
dev:
    cargo watch -q -c -x "run --bin api"

# Run the background worker with auto-reload
dev-worker:
    cargo watch -q -c -x "run --bin worker"

# build & check

# Compile all crates (no binary output)
check:
    cargo check --workspace

# Build release binaries
build:
    cargo build --release --workspace

# Run all tests
test:
    cargo test --workspace

# Lint with clippy
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Format all code
fmt:
    cargo fmt --all

# Format check (CI)
fmt-check:
    cargo fmt --all -- --check

# setup

# First-time setup: copy .dev.env, start services, run migrations
setup:
    @cp -n .dev.env.example .dev.env || true
    @echo "Edit .env if needed, then run: just up && just migrate"

# Generate openapi.json without starting the server
gen-schema:
    cargo run --bin gen-schema > openapi.json
    @echo "Written to openapi.json"

# Install required dev tools
install-tools:
    cargo install sqlx-cli --no-default-features --features rustls,postgres
    cargo install cargo-watch
    cargo install cargo-nextest
