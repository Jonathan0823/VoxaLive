# Multi-stage Dockerfile for VoxaLive backend
FROM rust:1.85-bookworm AS builder

WORKDIR /app

# Copy Cargo files first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/protocol/Cargo.toml ./crates/protocol/
COPY crates/config/Cargo.toml ./crates/config/
COPY crates/providers/Cargo.toml ./crates/providers/
COPY apps/backend/Cargo.toml ./apps/backend/

# Create dummy lib.rs files to cache dependencies
RUN mkdir -p crates/core/src crates/protocol/src crates/config/src crates/providers/src apps/backend/src && \
    echo "// dummy" > crates/core/src/lib.rs && \
    echo "// dummy" > crates/protocol/src/lib.rs && \
    echo "// dummy" > crates/config/src/lib.rs && \
    echo "// dummy" > crates/providers/src/lib.rs && \
    echo "fn main() {}" > apps/backend/src/main.rs && \
    cargo build --release -p voxalive-backend && \
    rm -rf crates/core/src crates/protocol/src crates/config/src crates/providers/src apps/backend/src

# Copy full source
COPY . .

# Build actual binary
RUN cargo build --release -p voxalive-backend

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/voxalive-backend /usr/local/bin/voxalive-backend

EXPOSE 8080

CMD ["voxalive-backend"]
