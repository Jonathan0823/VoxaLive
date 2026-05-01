# 14 Acceptance Criteria

## Purpose

This document defines when the SDD monorepo migration and MVP backend/admin setup is considered complete.

## Documentation Criteria

Documentation is accepted when:

- `AGENTS.md` exists.
- `docs/00_PROJECT_BRIEF.md` exists.
- `docs/01_PRODUCT_SPEC.md` exists.
- `docs/03_ARCHITECTURE.md` exists.
- `docs/04_BACKEND_DESIGN.md` exists.
- `docs/05_MONOREPO_STRUCTURE.md` exists.
- `docs/07_API_CONTRACT.md` exists.
- `docs/08_WS_PROTOCOL.md` exists.
- `docs/09_CONFIG_AND_SECRETS.md` exists.
- `docs/12_IMPLEMENTATION_PLAN.md` exists.
- `docs/14_ACCEPTANCE_CRITERIA.md` exists.
- Technical flows use Mermaid JS.
- Folder trees may use plain text.

## Monorepo Criteria

Monorepo setup is accepted when:

- root `Cargo.toml` exists as workspace.
- `apps/backend` exists.
- `apps/admin-web` exists.
- `crates/core` exists.
- `crates/protocol` exists.
- `crates/config` exists.
- `crates/providers` exists.
- backend can run from workspace.
- frontend can run from workspace.
- migration does not break existing backend behavior.

## Backend Structure Criteria

Backend structure is accepted when:

- HTTP routes live under `apps/backend/src/routes`.
- app state lives in `apps/backend/src/state.rs`.
- middleware lives under `apps/backend/src/middleware`.
- provider logic is not implemented inside route handlers.
- public API DTOs live in `crates/protocol`.
- WebSocket DTOs live in `crates/protocol`.
- core ports live in `crates/core`.
- config logic lives in `crates/config`.
- provider adapters live in `crates/providers`.

## API Criteria

Admin API is accepted when:

- `GET /api/health` works without admin token.
- `GET /api/config` requires admin token.
- `PATCH /api/config` requires admin token.
- `GET /api/secrets/status` requires admin token.
- `PUT /api/secrets` requires admin token.
- `POST /api/test/llm` requires admin token.
- `POST /api/test/tts` requires admin token.
- `POST /api/test/vts` requires admin token.
- API responses use standard success/error shape.
- invalid config is rejected before applying.
- route handlers call services, not provider SDKs.

## Secret Safety Criteria

Secret handling is accepted when:

- raw API keys are never returned from API responses.
- secret status returns only configured/not configured.
- raw secrets are not logged.
- Authorization header is not logged.
- frontend does not store secrets in localStorage.
- secret update endpoint does not echo secret values back.

## WebSocket Criteria

WebSocket protocol is accepted when:

- `/ws/unified?frontend=vts` is recognized.
- JSON messages include `v`.
- protocol version starts at `1`.
- text input message is accepted.
- error response follows protocol.
- frontend-specific formatting happens through adapters.
- WebSocket DTOs come from `crates/protocol`.

## Admin Frontend Criteria

Admin frontend is accepted when:

- app exists under `apps/admin-web`.
- app can run in development mode.
- dashboard can read health endpoint.
- provider page can read config.
- provider page can update config.
- secret status can be displayed without raw secret values.
- test console can call provider test endpoints, including VTube Studio.
- frontend calls backend API only.
- frontend does not call Gemini/OpenRouter/Ollama directly.

## Production Criteria

Production setup is accepted when:

- admin frontend can be built.
- backend can serve built admin frontend.
- backend release build works.
- `/admin` serves admin UI in production mode.
- `/api/*` remains available.
- `/ws/unified` remains available.

## Final Acceptance Checklist

~~~text
[ ] Documentation pack exists
[ ] Mermaid diagrams used for technical flows
[ ] Root Cargo workspace works
[ ] Backend runs from apps/backend
[ ] Admin frontend exists
[ ] Shared crates exist
[ ] Admin API works
[ ] Secret handling is safe
[ ] WebSocket protocol is stable
[ ] Existing backend behavior is preserved
[ ] Production build path is documented
~~~

## Definition of Done

The work is done when:

1. All required docs exist.
2. Backend runs in the new monorepo structure.
3. Admin API is available.
4. Admin frontend can call backend API.
5. Secrets are not leaked.
6. WebSocket behavior is preserved.
7. Acceptance checklist is complete.
