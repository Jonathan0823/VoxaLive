# 12 Implementation Plan

## Purpose

This document defines the recommended implementation order for AI agents and developers.

The goal is to migrate safely from backend-only to monorepo without breaking existing backend behavior.

## Overall Implementation Flow

~~~mermaid
flowchart TD
    Phase0[Phase 0 Documentation] --> Phase1[Phase 1 Monorepo Migration]
    Phase1 --> Phase2[Phase 2 Backend Admin API]
    Phase2 --> Phase3[Phase 3 Admin Web MVP]
    Phase3 --> Phase4[Phase 4 WebSocket Protocol Cleanup]
    Phase4 --> Phase5[Phase 5 Production Build]
    Phase5 --> Phase6[Phase 6 Testing and Cleanup]
~~~

## Phase 0: Documentation

Goal:

- create SDD handoff docs,
- define architecture,
- define contracts,
- prevent uncontrolled AI coding.

Tasks:

- create `AGENTS.md`
- create project brief
- create product spec
- create architecture doc
- create backend design doc
- create monorepo structure doc
- create API contract
- create WebSocket protocol
- create config and secrets doc
- create implementation plan
- create acceptance criteria

Exit criteria:

- all required docs exist,
- Mermaid diagrams are used for technical flows,
- backend architecture decision is clear.

## Phase 1: Monorepo Migration

Goal:

Move existing backend into monorepo structure without major refactor.

~~~mermaid
flowchart TD
    A[Create apps/backend] --> B[Move existing Rust backend files]
    B --> C[Create root Cargo.toml workspace]
    C --> D[Update package names]
    D --> E[Run cargo check]
    E --> F[Fix path/dependency issues]
    F --> G[Confirm existing backend runs]
~~~

Tasks:

- create `apps/backend`
- move existing `src/` into `apps/backend/src`
- move existing backend `Cargo.toml` into `apps/backend/Cargo.toml`
- create root `Cargo.toml`
- configure workspace members
- run `cargo check`
- run backend manually
- avoid behavior changes

Exit criteria:

- backend runs from `apps/backend`,
- root workspace works,
- existing routes still work.

## Phase 2: Shared Crates

Goal:

Add shared crates without forcing a big refactor.

~~~mermaid
flowchart TD
    A[Create crates/core] --> B[Create crates/protocol]
    B --> C[Create crates/config]
    C --> D[Create crates/providers]
    D --> E[Wire dependencies]
    E --> F[Move small safe types first]
~~~

Tasks:

- create `crates/core`
- create `crates/protocol`
- create `crates/config`
- create `crates/providers`
- add workspace members
- add minimal `lib.rs` for each crate
- move protocol DTOs first
- move config models second
- define core ports third
- move provider implementations last

Exit criteria:

- workspace still compiles,
- protocol types are reusable,
- no provider logic inside route handlers.

## Phase 3: Backend Admin API

Goal:

Expose admin API for simple frontend.

~~~mermaid
flowchart TD
    Health[GET /api/health] --> ConfigGet[GET /api/config]
    ConfigGet --> ConfigPatch[PATCH /api/config]
    ConfigPatch --> SecretStatus[GET /api/secrets/status]
    SecretStatus --> SecretPut[PUT /api/secrets]
    SecretPut --> TestLLM[POST /api/test/llm]
    TestLLM --> TestTTS[POST /api/test/tts]
~~~

Tasks:

- implement `GET /api/health`
- implement `GET /api/config`
- implement `PATCH /api/config`
- implement `GET /api/secrets/status`
- implement `PUT /api/secrets`
- implement `POST /api/test/llm`
- implement `POST /api/test/tts`
- implement admin auth middleware
- ensure standard API response shape
- ensure secrets are not exposed

Exit criteria:

- admin APIs work,
- protected APIs require token,
- secrets are never returned raw.

## Phase 4: Admin Web MVP

Goal:

Create simple frontend for settings and provider testing.

~~~mermaid
flowchart TD
    Create[Create apps/admin-web] --> Dashboard[Dashboard Page]
    Dashboard --> Providers[Providers Page]
    Providers --> Voice[Voice Page]
    Voice --> LiveInputs[Live Inputs Page]
    LiveInputs --> TestConsole[Test Console]
    TestConsole --> APIClient[Connect to Backend API]
~~~

Tasks:

- create Vite React TypeScript app
- add API client
- add dashboard page
- add provider settings page
- add voice settings page
- add live inputs page
- add test console page
- read health endpoint
- read/update config
- show secret status
- update secrets safely

Exit criteria:

- admin web runs locally,
- admin web can call backend,
- admin web does not store raw secrets in localStorage,
- admin web does not call third-party provider APIs directly.

## Phase 5: WebSocket Protocol Cleanup

Goal:

Make WebSocket protocol stable and typed.

~~~mermaid
flowchart TD
    Define[Define DTOs in crates/protocol] --> Decode[Use DTOs in WS route]
    Decode --> TextFlow[Test input.text flow]
    TextFlow --> AudioFlow[Test audio metadata flow]
    AudioFlow --> ErrorFlow[Test error flow]
    ErrorFlow --> AdapterFlow[Test frontend adapter selection]
~~~

Tasks:

- define client message DTOs,
- define server message DTOs,
- define protocol version constant,
- update WS handler to use protocol types,
- preserve existing `/ws/unified` behavior,
- add error frames,
- verify raw frontend flow.

Exit criteria:

- WS DTOs live in `crates/protocol`,
- WS messages include version field,
- text input flow works,
- error flow follows protocol.

## Phase 6: Production Build

Goal:

Build frontend and serve it through backend.

~~~mermaid
flowchart TD
    BuildWeb[pnpm build:web] --> Dist[apps/admin-web/dist]
    Dist --> BackendStatic[Backend Static Serving]
    BuildBackend[cargo build --release] --> Binary[Release Binary]
    BackendStatic --> Runtime[Production Runtime]
    Binary --> Runtime
    Runtime --> Admin[/admin]
    Runtime --> API[/api/*]
    Runtime --> WS[/ws/unified]
~~~

Tasks:

- add frontend build script,
- add static serving route,
- configure production static path,
- update Dockerfile,
- update README,
- test `/admin` route in production mode.

Exit criteria:

- frontend can be built,
- backend can serve admin UI,
- release binary works.

## Phase 7: Testing and Cleanup

Goal:

Validate implementation and remove migration leftovers.

Tasks:

- run `cargo check`,
- run backend tests,
- run frontend build,
- test health/config/secrets APIs,
- test WebSocket text input,
- test provider selection,
- update docs if implementation differs,
- clean unused files.

Exit criteria:

- acceptance criteria pass,
- docs match implementation,
- no known secret leaks,
- repo is ready for next feature work.

## AI Agent Working Rules

1. Work phase by phase.
2. Do not skip acceptance criteria.
3. Do not rewrite unrelated code.
4. Keep commits small.
5. Preserve existing behavior.
6. Update docs when implementation differs.
7. Ask for human decision only if architecture decision is required.
