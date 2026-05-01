# AGENTS.md

## Project

This repository is VoxaLive, a Rust-powered realtime avatar and voice AI backend with a simple admin web UI.

## Current Direction

The project is being migrated from backend-only into a monorepo.

The monorepo contains:
- Rust backend as the core runtime
- Admin web UI for settings, provider selection, and testing
- Shared protocol/config crates

External clients such as Web3D avatar, VTube Studio integration, mobile apps, and SDKs are not part of this MVP.

## Rules for AI Agent

1. Do not implement features before reading docs/PRODUCT_SPEC.md.
2. Do not change the WebSocket protocol without updating docs/WS_PROTOCOL.md.
3. Do not expose raw API keys to frontend responses.
4. Do not hardcode secrets.
5. Keep backend business logic independent from HTTP handlers.
6. Keep admin frontend simple and configuration-focused.
7. Prefer small incremental commits.
8. If a change affects architecture, update docs/ARCHITECTURE.md.
9. If a change affects API response format, update docs/API_CONTRACT.md.
10. If uncertain, preserve existing backend behavior.

## Development Commands

Backend:
cargo run -p voxalive-backend

Frontend:
pnpm --filter admin-web dev

Build:
pnpm build:web
cargo build --release -p voxalive-backend
