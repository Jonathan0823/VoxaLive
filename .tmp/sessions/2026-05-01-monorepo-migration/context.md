# Task Context: VoxaLive Monorepo Migration

Session ID: 2026-05-01-monorepo-migration
Created: 2026-05-01T12:00:00Z
Status: in_progress

## Current Request
Migrate VoxaLive from backend-only scaffold to complete monorepo structure with shared crates and admin API endpoints, following the implementation plan in docs/12_IMPLEMENTATION_PLAN.md. Execute the 10 subtasks defined in .tmp/tasks/monorepo-migration/ in parallel batches as approved.

## Context Files (Standards to Follow)
These are the project-specific standards and documentation files discovered during Stage 1:
- docs/00_PRODUCT_BRIEF.md
- docs/01_PRODUCT_SPEC.md
- docs/03_ARCHITECTURE.md
- docs/04_BACKEND_DESIGN.md
- docs/05_MONOREPO_STRUCTURE.md
- docs/07_API_CONTRACT.md
- docs/08_WS_PROTOCOL.md
- docs/09_CONFIG_AND_SECRETS.md
- docs/12_IMPLEMENTATION_PLAN.md
- docs/14_ACCEPTANCE_CRITERIA.md
- AGENTS.md (project rules for AI agents)

Note: code-quality.md from ~/.config/opencode/context/core/standards/ was not found; proceeding with project docs as primary standards.

## Reference Files (Source Material to Look At)
Project files relevant to this task:
- Cargo.toml (root workspace)
- apps/backend/Cargo.toml
- apps/backend/src/main.rs
- crates/core/Cargo.toml
- crates/core/src/lib.rs
- crates/core/src/domain/mod.rs
- crates/core/src/ports/mod.rs
- crates/core/src/service/mod.rs
- README.md

## External Docs Fetched
None (no external libraries requiring documentation fetch at this stage)

## Components
Functional units to implement as defined in the approved proposal:
1. `apps/admin-web` — Vite React TypeScript admin UI scaffold
2. `crates/protocol` — REST & WebSocket DTOs matching API contract docs
3. `crates/config` — Config models, validation, secret handling per config docs
4. `crates/providers` — Provider adapter scaffolding with module structure
5. `crates/core` — Update existing with port traits and domain types
6. `apps/backend` — Axum route handlers for admin API endpoints

## Constraints
Technical constraints and rules from AGENTS.md and architecture docs:
1. Do not implement features before reading docs/PRODUCT_SPEC.md
2. Do not change WebSocket protocol without updating docs/WS_PROTOCOL.md
3. Do not expose raw API keys to frontend responses
4. Do not hardcode secrets
5. Keep backend business logic independent from HTTP handlers
6. Keep admin frontend simple and configuration-focused
7. Prefer small incremental commits
8. If change affects architecture, update docs/ARCHITECTURE.md
9. If change affects API response format, update docs/API_CONTRACT.md
10. Preserve existing backend behavior during migration
11. Core crates must not depend on Axum, provider SDKs, or frontend implementations
12. Public API/WebSocket DTOs must live in crates/protocol
13. Provider logic must not be inside route handlers
14. Secrets never flow back to frontend as raw values

## Exit Criteria
- [ ] Monorepo structure matches docs/05_MONOREPO_STRUCTURE.md
- [ ] Root Cargo.toml workspace includes all 5 members (apps/backend, crates/core, crates/protocol, crates/config, crates/providers)
- [ ] package.json and pnpm-workspace.yaml exist at root
- [ ] apps/admin-web scaffold created and can run in dev mode
- [ ] crates/protocol has API and WS DTOs matching docs/07_API_CONTRACT.md and docs/08_WS_PROTOCOL.md
- [ ] crates/config has config models and secret handling matching docs/09_CONFIG_AND_SECRETS.md
- [ ] crates/providers has module structure for all adapter types
- [ ] crates/core has port traits (LlmProvider, TtsProvider, SttProvider, etc.)
- [ ] apps/backend has route handlers for all admin API endpoints
- [ ] GET /api/health works without admin token
- [ ] GET/PATCH /api/config requires admin token, no raw secrets returned
- [ ] GET /api/secrets/status and PUT /api/secrets implemented with secret safety
- [ ] POST /api/test/llm and /tts implemented
- [ ] cargo check passes at root workspace level
- [ ] All acceptance criteria from docs/14_ACCEPTANCE_CRITERIA.md are met
