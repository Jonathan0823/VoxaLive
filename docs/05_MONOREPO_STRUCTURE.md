# 05 Monorepo Structure

## Goal

The repository should be migrated from backend-only into a monorepo.

The monorepo should contain:

- Rust backend runtime,
- simple admin web UI,
- shared Rust crates,
- documentation for AI agents.

## Target Structure

~~~text
voxalive/
├─ apps/
│  ├─ backend/
│  └─ admin-web/
├─ crates/
│  ├─ core/
│  ├─ protocol/
│  ├─ config/
│  ├─ providers/
│  └─ runtime/
├─ docs/
├─ scripts/
├─ Cargo.toml
├─ Cargo.lock
├─ Dockerfile
├─ docker-compose.yml
├─ .env.example
├─ README.md
└─ AGENTS.md
~~~

## Workspace Relationship

~~~mermaid
flowchart TD
    Root[VoxaLive Monorepo] --> Apps[apps/]
    Root --> Crates[crates/]
    Root --> Docs[docs/]
    Root --> Scripts[scripts/]

    Apps --> Backend[apps/backend]
    Apps --> AdminWeb[apps/admin-web]

    Crates --> Core[crates/core]
    Crates --> Protocol[crates/protocol]
    Crates --> Config[crates/config]
    Crates --> Providers[crates/providers]
    Crates --> Runtime[crates/runtime optional]
~~~

## Rust Workspace

Root `Cargo.toml` should define a Cargo workspace.

~~~toml
[workspace]
resolver = "2"
members = [
  "apps/backend",
  "crates/core",
  "crates/protocol",
  "crates/config",
  "crates/providers"
]

[workspace.package]
edition = "2021"
version = "0.1.0"
license = "MIT"

[workspace.dependencies]
anyhow = "1"
tokio = { version = "1", features = ["full"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
~~~

Add `crates/runtime` later when required.

## Frontend App

`apps/admin-web` is a standalone Vite React TypeScript app.

Development:

~~~text
cd apps/admin-web && npm run dev
~~~

Build:

~~~text
cd apps/admin-web && npm run build
~~~

## Backend Crate Dependency Graph

~~~mermaid
flowchart TD
    Backend[apps/backend] --> Core[crates/core]
    Backend --> Protocol[crates/protocol]
    Backend --> Config[crates/config]
    Backend --> Providers[crates/providers]

    Providers --> Core
    Config --> Core
    Protocol --> Core

    Runtime[crates/runtime optional] --> Core
    Backend --> Runtime
~~~

## Development Flow

~~~mermaid
flowchart LR
    Dev[Developer] --> BackendCmd[cargo run -p voxalive-backend]
    Dev --> FrontendCmd[cd apps/admin-web && npm run dev]

    BackendCmd --> Backend[Backend localhost:8080]
    FrontendCmd --> AdminWeb[Admin Web localhost:5173]
    AdminWeb --> Backend
~~~

## Production Build Flow

~~~mermaid
flowchart TD
    BuildWeb[cd apps/admin-web && npm run build] --> WebDist[apps/admin-web/dist]
    BuildBackend[cargo build --release -p voxalive-backend] --> BackendBin[Backend Binary]
    WebDist --> StaticBundle[Copied or served as static admin assets]
    BackendBin --> Runtime[Production Runtime]
    StaticBundle --> Runtime
    Runtime --> AdminRoute[/admin]
    Runtime --> ApiRoutes[/api/*]
    Runtime --> WsRoute[/ws/unified]
~~~

## Migration Strategy

~~~mermaid
flowchart TD
    A[Start from backend-only repo] --> B[Create apps/backend]
    B --> C[Move existing src and Cargo.toml]
    C --> D[Create root Cargo workspace]
    D --> E[Verify backend compiles]
    E --> F[Create crates/core protocol config providers]
    F --> G[Move code gradually]
    G --> H[Create apps/admin-web]
    H --> I[Connect admin UI to backend API]
    I --> J[Serve admin static files in production]
~~~

## Migration Rules

1. Do not rewrite the backend during folder migration.
2. First goal is compile parity.
3. Move code into crates only after backend still runs.
4. Public API shape should be defined in docs before implementation.
5. Do not move provider code into route handlers.
6. Keep admin frontend small.
7. Use docs as source of truth.
