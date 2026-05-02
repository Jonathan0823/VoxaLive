# VoxaLive

VoxaLive is a Rust-based realtime voice/avatar AI backend.

The system receives input from text, voice, or live comments, processes it through an AI pipeline, and sends avatar control output to VTube Studio.

This repository is being structured as a monorepo with:

- Rust backend runtime
- React + Vite admin web UI
- Shared Rust crates for core logic, protocol, config, and providers
- SDD documentation for AI-agent-assisted development

## Status

VoxaLive is currently in MVP planning and implementation.

The first MVP focuses on:

- Rust backend
- VTube Studio integration
- LLM provider selection
- TTS/STT pipeline
- Runtime configuration
- Simple admin web UI
- Secure secret handling

Raw API, Web3D, mobile clients, desktop clients, and custom SDKs are future work.

## MVP Scope

Supported in the first MVP:

- VTube Studio as the only frontend output adapter
- Unified backend pipeline
- Admin API for configuration
- Admin web UI built with React + Vite + TypeScript
- LLM providers: Gemini, OpenRouter, Ollama
- TTS providers: Piper and Qwen, depending on environment support
- STT provider: faster-whisper, depending on environment support

Not included in the first MVP:

- Web3D frontend
- Raw API frontend adapter
- Mobile app
- Desktop app
- Multi-tenant user management
- Billing or payment system
- Plugin marketplace

## Architecture

The backend uses Hexagonal Architecture / Ports and Adapters.

The core backend defines ports for external systems. Provider implementations live outside the core as adapters.

~~~mermaid
flowchart TD
    Input[Text / Voice / Live Comment] --> Pipeline[Core Pipeline]

    Pipeline --> STT[STT Port]
    Pipeline --> LLM[LLM Port]
    Pipeline --> TTS[TTS Port]
    Pipeline --> Frontend[Frontend Adapter Port]

    STT --> Whisper[Whisper Adapter]
    LLM --> Gemini[Gemini Adapter]
    LLM --> OpenRouter[OpenRouter Adapter]
    LLM --> Ollama[Ollama Adapter]
    TTS --> Piper[Piper Adapter]
    TTS --> Qwen[Qwen Adapter]
    Frontend --> VTS[VTube Studio Adapter]
~~~

Main principle:

~~~text
Core logic depends on ports.
Adapters implement ports.
Routes call services/use cases.
Provider SDKs do not live inside route handlers.
~~~

## Repository Structure

Target monorepo structure:

~~~text
voxalive/
├─ apps/
│  ├─ backend/
│  └─ admin-web/
├─ crates/
│  ├─ core/
│  ├─ protocol/
│  ├─ config/
│  └─ providers/
├─ docs/
├─ scripts/
├─ Cargo.toml
├─ package.json
├─ pnpm-workspace.yaml
└─ AGENTS.md
~~~

## Backend

The backend is responsible for:

- HTTP routes
- WebSocket routes
- pipeline orchestration
- provider selection
- runtime config
- secret handling
- VTube Studio integration
- static admin UI serving

Main backend routes:

~~~text
GET    /api/health
GET    /api/config
PATCH  /api/config
GET    /api/secrets/status
PUT    /api/secrets
POST   /api/test/llm
POST   /api/test/tts
POST   /api/test/vts
GET    /ws/unified?frontend=vts
~~~

Reserved for future:

~~~text
GET /ws/unified?frontend=raw
GET /ws/unified?frontend=web3d
~~~

Unsupported frontend values should return `FRONTEND_NOT_SUPPORTED`.

## Admin Web

The admin web UI is a local configuration and testing console.

Frontend stack:

~~~text
React
Vite
TypeScript
~~~

The admin web UI is used to:

- view backend status
- select LLM provider
- configure TTS/STT
- configure VTube Studio connection
- check secret status
- test LLM/TTS/VTube Studio connection

The admin web UI is not a full avatar frontend.

## Configuration

Create a `.env` file from `.env.example`.

Minimal example:

~~~env
BIND_ADDR=0.0.0.0:8080
ADMIN_UI_ENABLED=true
ADMIN_TOKEN=change-me

LLM_PROVIDER=gemini
LLM_MODEL=gemini-2.0-flash-exp
GEMINI_API_KEY=

OPENROUTER_API_KEY=
OLLAMA_URL=http://localhost:11434

TTS_PROVIDER=piper
TTS_MODE=cpu
STT_DEVICE=cpu

VTS_URL=ws://localhost:8001
~~~

Secret rules:

- Raw secrets must never be returned by the API.
- Secret status may only show configured or not configured.
- Frontend must not store secrets in localStorage.
- Logs must not contain raw secrets.

## Production Build

### Build Frontend

```bash
pnpm --filter admin-web build
```

The frontend build output will be in `apps/admin-web/dist`.

### Static File Serving

The backend serves the admin UI from the path specified by the `STATIC_DIR` environment variable (defaults to `apps/admin-web/dist`).

Set the environment variable before running the backend:

```bash
export STATIC_DIR=/path/to/apps/admin-web/dist
cargo run -p voxalive-backend
```

In production (Docker), the `STATIC_DIR` is set to `/app/admin-web/dist`.

### Docker Build

```bash
docker build -t voxalive-backend .
docker run -p 8080:8080 voxalive-backend
```

The admin UI will be available at `http://localhost:8080/admin/index.html`.

### Release Binary

```bash
cargo build --release -p voxalive-backend
./target/release/voxalive-backend
```

## Development

### Environment Setup

Use a single root `.env` file (gitignored) for local development:

~~~bash
cp .env.example .env
# edit .env with your local values
~~~

The backend loads `.env` automatically via `dotenvy`.
The frontend (Vite) reads `VITE_*` vars from the repo root via `envDir`.

### Native build requirements

The STT adapter uses `whisper-rs`, which needs native build tools:

~~~bash
cmake
clang / libclang
~~~

On Debian/Ubuntu, for example:

~~~bash
sudo apt-get install cmake clang libclang-dev
~~~

Install frontend dependencies:

~~~bash
pnpm install
~~~

Run backend:

~~~bash
cargo run -p voxalive-backend
~~~

Run admin web:

~~~bash
pnpm --filter admin-web dev
~~~

Build backend:

~~~bash
cargo build --release -p voxalive-backend
~~~

Build admin web:

~~~bash
pnpm --filter admin-web build
~~~

## VTube Studio Setup

1. Open VTube Studio.
2. Enable the VTube Studio plugin API.
3. Confirm the API WebSocket port, usually `8001`.
4. Set `VTS_URL=ws://localhost:8001`.
5. Start VoxaLive backend.
6. Use the admin web UI to test the VTube Studio connection.

## Documentation

Important docs:

~~~text
AGENTS.md
docs/00_PROJECT_BRIEF.md
docs/01_PRODUCT_SPEC.md
docs/03_ARCHITECTURE.md
docs/04_BACKEND_DESIGN.md
docs/05_MONOREPO_STRUCTURE.md
docs/07_API_CONTRACT.md
docs/08_WS_PROTOCOL.md
docs/09_CONFIG_AND_SECRETS.md
docs/12_IMPLEMENTATION_PLAN.md
docs/14_ACCEPTANCE_CRITERIA.md
~~~

AI agents should read `AGENTS.md` first before making changes.

## Development Principle

Work in small phases:

~~~text
1. Add/update docs
2. Inspect current repo
3. Migrate backend into monorepo
4. Add shared crates
5. Move protocol types
6. Add admin API
7. Add admin web
8. Implement VTube Studio adapter
~~~

Do not refactor all components at once.

## License

MIT
