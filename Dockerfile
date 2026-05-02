# Stage 1: Build frontend
FROM node:20-bookworm AS frontend-builder

WORKDIR /app/frontend

# Copy frontend files
COPY apps/admin-web/package.json apps/admin-web/package-lock.json ./
RUN npm install

COPY apps/admin-web/ ./
RUN npm run build

# Stage 2: Build backend
FROM rust:1.85-bookworm AS backend-builder

WORKDIR /app

# Native build tools required by whisper-rs / whisper-rs-sys
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    clang \
    libclang-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml ./crates/core/
COPY crates/protocol/Cargo.toml ./crates/protocol/
COPY crates/config/Cargo.toml ./crates/config/
COPY crates/providers/Cargo.toml ./crates/providers/
COPY apps/backend/Cargo.toml ./apps/backend/

# Create dummy files to cache dependencies
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

# Copy built frontend dist from frontend-builder
COPY --from=frontend-builder /app/frontend/dist ./apps/admin-web/dist

# Build actual binary (now includes frontend dist path)
RUN cargo build --release -p voxalive-backend

# Stage 3: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/voxalive-backend /usr/local/bin/voxalive-backend

# Copy frontend static files
COPY --from=frontend-builder /app/frontend/dist /app/admin-web/dist

EXPOSE 8080

CMD ["voxalive-backend"]
