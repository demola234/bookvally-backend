# ── build stage ─────────────────────────────────────────────
FROM rust:1.80-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY cmd/ cmd/

# Build release binary
RUN cargo build --release --bin api

# ── runtime stage ────────────────────────────────────────────
FROM alpine:3.20 AS runtime

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY --from=builder /app/target/release/api /app/api
COPY config/ /app/config/
COPY migrations/ /app/migrations/

ENV APP_ENV=production

EXPOSE 8080

ENTRYPOINT ["/app/api"]
