# 04 Backend Design

## Architecture Style

The backend uses **Hexagonal Architecture / Ports and Adapters**.

The backend core must not depend directly on external providers, HTTP framework details, WebSocket implementation details, or storage implementation details.

External systems are accessed through ports defined by the core, then implemented as adapters.

## Responsibilities

The backend is responsible for:

- HTTP routes
- WebSocket routes
- Pipeline orchestration
- Provider selection
- Runtime config
- Secret handling
- Provider health check
- Admin API
- Static admin UI serving
- Graceful shutdown

The backend is not responsible for:

- Full Web3D rendering
- Mobile app UI
- VTube Studio UI
- Multi-tenant user management
- Payment or billing

## Main Routes

- `GET /api/health`
- `GET /api/config`
- `PATCH /api/config`
- `GET /api/secrets/status`
- `PUT /api/secrets`
- `POST /api/test/llm`
- `POST /api/test/tts`
- `GET /ws/unified`

## Runtime Flow

~~~mermaid
flowchart TD
    Voice[Voice WS] --> InputAdapter[Input Adapter]
    Text[Text API / WS] --> InputAdapter
    Youtube[YouTube Live] --> InputAdapter
    TikTok[TikTok Live] --> InputAdapter

    InputAdapter --> Queue[Pipeline Queue]
    Queue --> Worker[Pipeline Worker]

    Worker --> IsAudio{Audio Input?}
    IsAudio -->|Yes| STT[STT Provider Port]
    IsAudio -->|No| LLM[LLM Provider Port]
    STT --> LLM
    LLM --> TTS[TTS Provider Port]
    TTS --> Output[Frontend Adapter Port]

    Output --> Raw[Raw API]
    Output --> VTS[VTube Studio]
    Output --> Web3D[Web3D]
~~~

## Internal Modules

~~~text
apps/backend/src/
├─ main.rs
├─ app.rs
├─ state.rs
├─ shutdown.rs
├─ routes/
│  ├─ mod.rs
│  ├─ health.rs
│  ├─ config.rs
│  ├─ secrets.rs
│  ├─ test.rs
│  └─ ws.rs
├─ middleware/
│  ├─ mod.rs
│  └─ admin_auth.rs
└─ static_files.rs
~~~

## Shared Crates

~~~text
crates/
├─ core/
├─ protocol/
├─ config/
├─ providers/
└─ runtime/        # optional, added when queue/worker logic grows
~~~

## Crate Responsibilities

### `crates/core`

Contains backend domain logic, ports, and core services.

Should contain:

- domain types
- provider-independent service logic
- ports / traits
- pipeline orchestration interface

Example folders:

~~~text
crates/core/src/
├─ domain/
├─ ports/
└─ service/
~~~

Example ports:

- `LlmProvider`
- `TtsProvider`
- `SttProvider`
- `LiveInputProvider`
- `FrontendAdapter`
- `ConfigStore`
- `SecretStore`

Rule:

> `crates/core` must not depend on Axum, provider SDKs, database drivers, or frontend-specific implementation.

### `crates/protocol`

Contains external API and WebSocket contract types.

Should contain:

- REST request/response DTOs
- WebSocket message DTOs
- shared error response shape
- protocol version constants

Example folders:

~~~text
crates/protocol/src/
├─ api/
└─ ws/
~~~

Rule:

> All public API and WebSocket JSON structures should come from `crates/protocol`.

### `crates/config`

Contains config loading, runtime config, validation, and secret status logic.

Should contain:

- default config
- env config loader
- runtime config model
- config validation
- secret status mapping
- config store adapters

Config priority:

~~~mermaid
flowchart TD
    Runtime[Runtime Config] --> Effective[Effective Config]
    Env[Environment Config] --> Effective
    Default[Default Config] --> Effective
    Runtime -. highest priority .-> Effective
    Default -. lowest priority .-> Effective
~~~

Rule:

> Raw secrets must never be returned from any API response.

### `crates/providers`

Contains external provider adapters.

Should contain:

- Gemini adapter
- OpenRouter adapter
- Ollama adapter
- Qwen TTS adapter
- Piper TTS adapter
- Whisper STT adapter
- YouTube live input adapter
- TikTok live input adapter
- Raw frontend adapter
- VTube Studio frontend adapter
- Web3D frontend adapter

Example folders:

~~~text
crates/providers/src/
├─ llm/
├─ tts/
├─ stt/
├─ live/
└─ frontend/
~~~

Rule:

> Provider adapters may depend on external SDKs, but the core must only depend on provider ports.

### `crates/runtime` Optional

Contains async runtime orchestration if the pipeline grows large.

Should contain:

- queue
- worker
- supervisor
- client registry
- metrics state
- shutdown helpers

This crate can be added later.

## App State

`AppState` should contain:

- `ConfigManager`
- `SecretManager`
- `ProviderRegistry`
- `PipelineService`
- `ClientRegistry`
- `MetricsState`

Example:

~~~rust
pub struct AppState {
    pub config: Arc<ConfigManager>,
    pub secrets: Arc<SecretManager>,
    pub providers: Arc<ProviderRegistry>,
    pub pipeline: Arc<PipelineService>,
    pub clients: Arc<ClientRegistry>,
    pub metrics: Arc<MetricsState>,
}
~~~

## Provider Registry

The provider registry is responsible for selecting the active provider implementation based on runtime config.

~~~mermaid
flowchart TD
    Config[Runtime Config] --> Registry[Provider Registry]
    Registry --> ActiveLLM[Active LLM Provider]
    Registry --> ActiveTTS[Active TTS Provider]
    Registry --> ActiveSTT[Active STT Provider]

    ActiveLLM --> Gemini[Gemini Adapter]
    ActiveLLM --> OpenRouter[OpenRouter Adapter]
    ActiveLLM --> Ollama[Ollama Adapter]

    ActiveTTS --> Qwen[Qwen Adapter]
    ActiveTTS --> Piper[Piper Adapter]
~~~

The route layer should never instantiate providers directly.

## Pipeline Service

The pipeline service is responsible for processing one input into one output flow.

~~~mermaid
flowchart TD
    Input[InputPayload] --> Normalize[Normalize Input]
    Normalize --> CheckAudio{Is Audio?}
    CheckAudio -->|Yes| Transcribe[Transcribe via STT Port]
    CheckAudio -->|No| Prompt[Prepare LLM Request]
    Transcribe --> Prompt
    Prompt --> LLM[Call Active LLM Provider]
    LLM --> TTS[Synthesize via Active TTS Provider]
    TTS --> Adapter[Format via Frontend Adapter]
    Adapter --> Send[Send Response to Client]
~~~

The pipeline should depend on ports, not concrete provider implementations.

## Admin API Design

Admin API exists to support the simple admin frontend.

~~~mermaid
sequenceDiagram
    participant AdminWeb
    participant Route
    participant Service
    participant ConfigManager
    participant ProviderRegistry

    AdminWeb->>Route: PATCH /api/config
    Route->>Service: Update config command
    Service->>ConfigManager: Validate and save
    ConfigManager->>ProviderRegistry: Refresh selected providers
    ProviderRegistry-->>Service: Active provider updated
    Service-->>Route: Result
    Route-->>AdminWeb: API response
~~~

Admin API should not:

- expose raw API keys,
- directly call provider SDKs from route handlers,
- bypass config validation,
- bypass admin authentication.

## Secret Handling

Secret handling rules:

1. Raw secrets must never be returned from API responses.
2. Secret status may return only `configured: true/false`.
3. Frontend must not store secrets in localStorage.
4. Secret update endpoint may accept raw secret values.
5. Secret values should be stored through `SecretManager` only.
6. Logs must not include raw secret values.

## WebSocket Design

The unified WebSocket endpoint is:

~~~text
/ws/unified?frontend=raw
/ws/unified?frontend=vts
/ws/unified?frontend=web3d
~~~

~~~mermaid
sequenceDiagram
    participant Client
    participant WSRoute
    participant Protocol
    participant Pipeline
    participant Adapter

    Client->>WSRoute: Connect with frontend query
    WSRoute->>Protocol: Decode client message
    WSRoute->>Pipeline: Submit input
    Pipeline->>Adapter: Format output for frontend
    Adapter-->>WSRoute: Protocol response frames
    WSRoute-->>Client: JSON and/or binary frames
~~~

Rules:

1. WebSocket message types must come from `crates/protocol`.
2. Protocol messages must include a version field.
3. Binary audio frames must follow the protocol spec.
4. Frontend-specific formatting should happen through frontend adapters.
5. Route handler should only handle connection setup and message forwarding.

## Route Handler Rules

1. HTTP handlers should be thin.
2. WebSocket handlers should be thin.
3. Provider logic should not live inside route handlers.
4. Config updates should validate before applying.
5. Secrets should never be returned in raw form.
6. WebSocket protocol should use types from `crates/protocol`.
7. Route handlers should call services, not provider SDKs.
8. Errors should use the shared API error response shape.

## Initial Migration Rule

Do not refactor everything at once.

~~~mermaid
flowchart TD
    A[Move current backend into apps/backend] --> B[Add root Cargo workspace]
    B --> C[Add crates/core protocol config providers]
    C --> D[Keep existing backend behavior working]
    D --> E[Move protocol types into crates/protocol]
    E --> F[Move config logic into crates/config]
    F --> G[Define core ports in crates/core]
    G --> H[Move provider implementations into crates/providers]
    H --> I[Add crates/runtime only when needed]
~~~

## Acceptance Criteria

Backend design is considered implemented when:

- Backend runs from `apps/backend`.
- Root Cargo workspace works.
- Routes are thin and call services.
- Provider implementations are not inside route handlers.
- Public API DTOs live in `crates/protocol`.
- Core ports live in `crates/core`.
- Runtime config is validated before applying.
- Secret API never returns raw secret values.
- Unified WebSocket behavior is preserved.
- Admin frontend can call backend config and test APIs.
