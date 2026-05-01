# 04 Backend Design

## Architecture Style

The backend uses **Hexagonal Architecture / Ports and Adapters**.

This means the backend is organized around a stable application core. The core defines the business/runtime rules and the ports it needs. External technologies such as Axum, WebSocket, Gemini, OpenRouter, Ollama, Piper, Qwen, Whisper, VTube Studio, config files, and secret stores are adapters.

The core must not depend directly on:

- HTTP framework details,
- WebSocket implementation details,
- LLM/TTS/STT SDKs,
- VTube Studio API implementation details,
- file/database storage implementation details,
- frontend framework details.

## Why Hexagonal Architecture

VoxaLive has many replaceable external components:

- LLM providers: Gemini, OpenRouter, Ollama
- TTS providers: Qwen, Piper
- STT provider: faster-whisper
- live input providers: YouTube, TikTok
- frontend output adapter: VTube Studio for MVP
- config store
- secret store

The backend must be able to change these implementations without rewriting the core pipeline.

Therefore:

- `crates/core` owns domain logic, ports, and use cases.
- `apps/backend` owns delivery mechanisms such as HTTP and WebSocket routes.
- `crates/providers` owns external provider adapters.
- `crates/config` owns config and secret adapters.
- `crates/protocol` owns public API and WebSocket DTOs.

## MVP Frontend Adapter Decision

For MVP, VoxaLive supports **VTube Studio only** as the frontend output adapter.

Supported in MVP:

- VTube Studio connection
- VTube Studio authentication
- VTube Studio parameter updates
- VTube Studio expression/hotkey control if needed
- TTS audio coordination with VTube Studio avatar movement

Not supported in MVP:

- Raw API frontend adapter
- Web3D frontend adapter
- mobile client adapter
- desktop client adapter
- custom client SDK

The architecture should still keep the `FrontendAdapter` port so Raw API, Web3D, or other adapters can be added later without changing the pipeline core.

## Hexagonal Boundary

~~~mermaid
flowchart TB
    subgraph Core["crates/core: Application Core"]
        Domain[Domain Models]
        Ports[Ports / Traits]
        UseCases[Use Cases / Services]

        Domain --> UseCases
        Ports --> UseCases
    end

    subgraph Delivery["Delivery Adapters"]
        HTTP[apps/backend HTTP Routes]
        WS[apps/backend WebSocket Routes]
        AdminStatic[Static Admin UI Serving]
    end

    subgraph ProviderAdapters["Provider Adapters"]
        Gemini[Gemini LLM Adapter]
        OpenRouter[OpenRouter LLM Adapter]
        Ollama[Ollama LLM Adapter]
        Piper[Piper TTS Adapter]
        Qwen[Qwen TTS Adapter]
        Whisper[Whisper STT Adapter]
        Youtube[YouTube Live Adapter]
        TikTok[TikTok Live Adapter]
        VTS[VTube Studio Adapter]
    end

    subgraph ConfigAdapters["Config / Secret Adapters"]
        EnvConfig[Environment Config]
        FileConfig[Runtime Config File]
        SecretStore[Secret Store]
    end

    HTTP --> UseCases
    WS --> UseCases
    AdminStatic --> HTTP

    UseCases --> Ports

    Ports -. implemented by .-> Gemini
    Ports -. implemented by .-> OpenRouter
    Ports -. implemented by .-> Ollama
    Ports -. implemented by .-> Piper
    Ports -. implemented by .-> Qwen
    Ports -. implemented by .-> Whisper
    Ports -. implemented by .-> Youtube
    Ports -. implemented by .-> TikTok
    Ports -. implemented by .-> VTS
    Ports -. implemented by .-> EnvConfig
    Ports -. implemented by .-> FileConfig
    Ports -. implemented by .-> SecretStore
~~~

## Dependency Direction

The dependency direction must point inward.

~~~mermaid
flowchart TD
    Backend[apps/backend] --> Core[crates/core]
    Backend --> Protocol[crates/protocol]
    Backend --> Config[crates/config]
    Backend --> Providers[crates/providers]

    Providers --> Core
    Config --> Core
    Protocol --> Core

    Core -. must not depend on .-> Backend
    Core -. must not depend on .-> Providers
    Core -. must not depend on .-> Config
    Core -. must not depend on .-> Axum[Axum]
    Core -. must not depend on .-> SDKs[Provider SDKs]
~~~

Rules:

1. `crates/core` defines ports.
2. `crates/providers` implements provider ports.
3. `crates/config` implements config and secret ports.
4. `apps/backend` wires everything together.
5. Route handlers call use cases/services.
6. Use cases depend on ports, not concrete adapters.

## Responsibilities

The backend is responsible for:

- HTTP routes
- WebSocket routes
- VTube Studio connection route/control flow
- pipeline orchestration
- provider selection
- runtime config
- secret handling
- provider health check
- admin API
- static admin UI serving
- graceful shutdown

The backend is not responsible for:

- full Web3D rendering
- mobile app UI
- VTube Studio UI
- multi-tenant user management
- payment or billing

## Main Routes

- `GET /api/health`
- `GET /api/config`
- `PATCH /api/config`
- `GET /api/secrets/status`
- `PUT /api/secrets`
- `POST /api/test/llm`
- `POST /api/test/tts`
- `POST /api/test/vts`
- `GET /ws/unified?frontend=vts`

Reserved for future:

- `GET /ws/unified?frontend=raw`
- `GET /ws/unified?frontend=web3d`

Unsupported frontend values should return:

~~~json
{
  "ok": false,
  "error": {
    "code": "FRONTEND_NOT_SUPPORTED",
    "message": "Only VTube Studio frontend is supported in the MVP."
  }
}
~~~

## Runtime Flow

~~~mermaid
flowchart TD
    Voice[Voice Input] --> InputAdapter[Input Adapter]
    Text[Text Input] --> InputAdapter
    Youtube[YouTube Live Comment] --> InputAdapter
    TikTok[TikTok Live Comment] --> InputAdapter

    InputAdapter --> PipelineUseCase[Pipeline Use Case]
    PipelineUseCase --> IsAudio{Audio Input?}

    IsAudio -->|Yes| STTPort[STT Port]
    IsAudio -->|No| LLMRequest[Prepare LLM Request]

    STTPort --> WhisperAdapter[Whisper Adapter]
    WhisperAdapter --> Transcript[Transcript]
    Transcript --> LLMRequest

    LLMRequest --> LLMPort[LLM Port]
    LLMPort --> ActiveLLM[Active LLM Adapter]
    ActiveLLM --> LLMResponse[LLM Text Response]

    LLMResponse --> TTSPort[TTS Port]
    TTSPort --> ActiveTTS[Active TTS Adapter]
    ActiveTTS --> TTSOutput[Audio + Optional Viseme Cues]

    TTSOutput --> FrontendPort[Frontend Adapter Port]
    FrontendPort --> VTSAdapter[VTube Studio Adapter]
    VTSAdapter --> VTubeStudio[VTube Studio]
    VTubeStudio --> Live2D[Live2D Model]
~~~

## VTube Studio Flow

~~~mermaid
sequenceDiagram
    participant Backend
    participant VTSAdapter
    participant VTS as VTube Studio API
    participant Model as Live2D Model

    Backend->>VTSAdapter: Initialize VTS adapter
    VTSAdapter->>VTS: Connect WebSocket
    VTS-->>VTSAdapter: Connected
    VTSAdapter->>VTS: Request authentication token if needed
    VTS-->>VTSAdapter: Authentication token / response
    VTSAdapter->>VTS: Authenticate plugin
    VTS-->>VTSAdapter: Authentication success
    Backend->>VTSAdapter: Send avatar output command
    VTSAdapter->>VTS: Update model parameters / trigger hotkey
    VTS->>Model: Apply parameter update
~~~

## Internal Modules

`apps/backend` is a delivery and composition layer, not the application core.

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

Responsibilities:

| Module | Responsibility |
|---|---|
| `main.rs` | Start process, load env, initialize tracing, build app |
| `app.rs` | Compose Axum router |
| `state.rs` | Hold shared app dependencies |
| `shutdown.rs` | Graceful shutdown |
| `routes/*` | HTTP/WebSocket delivery only |
| `middleware/admin_auth.rs` | Admin token auth |
| `static_files.rs` | Serve built admin frontend |

Rule:

> `apps/backend` may depend on Axum, but `crates/core` must not.

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

Application core.

Contains:

- domain models,
- ports/traits,
- use cases/services,
- provider-independent pipeline logic.

Example structure:

~~~text
crates/core/src/
├─ domain/
│  ├─ input.rs
│  ├─ output.rs
│  ├─ audio.rs
│  ├─ provider.rs
│  ├─ vts.rs
│  └─ error.rs
├─ ports/
│  ├─ llm.rs
│  ├─ tts.rs
│  ├─ stt.rs
│  ├─ live_input.rs
│  ├─ frontend.rs
│  ├─ config_store.rs
│  └─ secret_store.rs
└─ use_cases/
   ├─ process_input.rs
   ├─ update_config.rs
   ├─ test_provider.rs
   └─ connect_vts.rs
~~~

Core rules:

1. Must not depend on Axum.
2. Must not depend on provider SDKs.
3. Must not depend on VTube Studio WebSocket SDK implementation.
4. Must not depend on database/file storage implementation.
5. Must define traits for external behavior.

Example ports:

~~~rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, LlmError>;
}

#[async_trait]
pub trait TtsProvider: Send + Sync {
    async fn synthesize(&self, request: TtsRequest) -> Result<TtsResponse, TtsError>;
}

#[async_trait]
pub trait SttProvider: Send + Sync {
    async fn transcribe(&self, request: SttRequest) -> Result<SttResponse, SttError>;
}

#[async_trait]
pub trait FrontendAdapter: Send + Sync {
    async fn send_output(&self, output: FrontendOutput) -> Result<(), FrontendAdapterError>;
}

#[async_trait]
pub trait VTubeStudioClient: Send + Sync {
    async fn connect(&self) -> Result<(), VtsError>;
    async fn authenticate(&self) -> Result<(), VtsError>;
    async fn update_parameters(&self, params: Vec<VtsParameter>) -> Result<(), VtsError>;
}
~~~

### `crates/protocol`

External contract crate.

Contains:

- REST request/response DTOs,
- WebSocket message DTOs,
- shared error response shape,
- protocol version constants.

Example structure:

~~~text
crates/protocol/src/
├─ api/
│  ├─ health.rs
│  ├─ config.rs
│  ├─ secrets.rs
│  ├─ test.rs
│  └─ error.rs
└─ ws/
   ├─ client.rs
   ├─ server.rs
   └─ error.rs
~~~

Rule:

> All public API and WebSocket JSON structures should come from `crates/protocol`.

### `crates/config`

Config and secret adapter crate.

Contains:

- default config,
- env config loader,
- runtime config model,
- config validation,
- secret status mapping,
- config store adapters.

Config priority:

~~~mermaid
flowchart TD
    Runtime[Runtime Config] --> Effective[Effective Config]
    Env[Environment Config] --> Effective
    Default[Default Config] --> Effective

    Runtime -. highest priority .-> Effective
    Env -. middle priority .-> Effective
    Default -. lowest priority .-> Effective
~~~

Rule:

> Raw secrets must never be returned from any API response.

### `crates/providers`

External provider adapter crate.

Contains concrete implementations of ports defined by `crates/core`.

Example structure for MVP:

~~~text
crates/providers/src/
├─ llm/
│  ├─ mod.rs
│  ├─ gemini.rs
│  ├─ openrouter.rs
│  └─ ollama.rs
├─ tts/
│  ├─ mod.rs
│  ├─ piper.rs
│  └─ qwen.rs
├─ stt/
│  ├─ mod.rs
│  └─ whisper.rs
├─ live/
│  ├─ mod.rs
│  ├─ youtube.rs
│  └─ tiktok.rs
└─ frontend/
   ├─ mod.rs
   └─ vts.rs
~~~

Not implemented in MVP:

~~~text
crates/providers/src/frontend/raw.rs
crates/providers/src/frontend/web3d.rs
~~~

Rule:

> Provider adapters may depend on external SDKs, but the core must only depend on provider ports.

### `crates/runtime` Optional

Use only when async queue, workers, supervisor, client registry, and metrics become large.

Possible structure:

~~~text
crates/runtime/src/
├─ queue.rs
├─ worker.rs
├─ supervisor.rs
├─ client_registry.rs
├─ metrics.rs
└─ shutdown.rs
~~~

Until then, runtime logic may stay in `crates/core::use_cases` or `apps/backend` as long as dependency boundaries remain clear.

## App State

`AppState` is the composition container for the delivery layer.

It wires concrete adapters into core use cases.

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

Rule:

> `AppState` may hold concrete dependencies, but route handlers should still call services/use cases rather than directly calling adapters.

## Provider Registry

The provider registry selects active provider adapters based on runtime config.

~~~mermaid
flowchart TD
    Config[Effective Runtime Config] --> Registry[Provider Registry]

    Registry --> LLMPort[LLM Provider Port]
    Registry --> TTSPort[TTS Provider Port]
    Registry --> STTPort[STT Provider Port]
    Registry --> FrontendPort[Frontend Adapter Port]

    LLMPort --> Gemini[Gemini Adapter]
    LLMPort --> OpenRouter[OpenRouter Adapter]
    LLMPort --> Ollama[Ollama Adapter]

    TTSPort --> Piper[Piper Adapter]
    TTSPort --> Qwen[Qwen Adapter]

    STTPort --> Whisper[Whisper Adapter]

    FrontendPort --> VTS[VTube Studio Adapter]
~~~

The route layer must never instantiate providers directly.

## Pipeline Use Case

The pipeline use case processes one input into one frontend output.

~~~mermaid
flowchart TD
    Input[InputPayload] --> Normalize[Normalize Input]
    Normalize --> AudioCheck{Audio?}

    AudioCheck -->|Yes| STT[Use STT Port]
    AudioCheck -->|No| PreparePrompt[Prepare Prompt]

    STT --> PreparePrompt
    PreparePrompt --> LLM[Use LLM Port]
    LLM --> TTS[Use TTS Port]
    TTS --> MapVTS[Map audio/viseme/emotion to VTS output]
    MapVTS --> Frontend[Use FrontendAdapter Port]
    Frontend --> Done[Done]
~~~

The use case depends on:

- `LlmProvider`
- `TtsProvider`
- `SttProvider`
- `FrontendAdapter`
- `ConfigStore`
- `SecretStore`

The use case must not depend on:

- Axum,
- Gemini SDK,
- OpenRouter SDK,
- Ollama SDK,
- VTube Studio SDK,
- file storage implementation.

## Admin API Design

Admin API exists to support the simple React + Vite admin frontend.

Admin API should allow:

- reading current config,
- updating runtime config,
- checking secret status,
- updating secrets,
- testing LLM provider,
- testing TTS provider,
- testing VTube Studio connection.

~~~mermaid
sequenceDiagram
    participant AdminWeb
    participant Route
    participant UseCase
    participant Port
    participant Adapter

    AdminWeb->>Route: POST /api/test/vts
    Route->>UseCase: Test VTS connection
    UseCase->>Port: connect/authenticate/update test parameter
    Port->>Adapter: VTube Studio adapter implementation
    Adapter-->>Port: Result
    Port-->>UseCase: Result
    UseCase-->>Route: Result DTO
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
7. VTube Studio auth tokens should be treated as secrets.

## WebSocket Design

MVP supported endpoint:

~~~text
/ws/unified?frontend=vts
~~~

Future reserved endpoints:

~~~text
/ws/unified?frontend=raw
/ws/unified?frontend=web3d
~~~

~~~mermaid
sequenceDiagram
    participant Client
    participant WSRoute
    participant Protocol
    participant PipelineUseCase
    participant VTSAdapter

    Client->>WSRoute: Connect /ws/unified?frontend=vts
    WSRoute->>Protocol: Decode client message
    Protocol-->>WSRoute: Typed input
    WSRoute->>PipelineUseCase: Submit input
    PipelineUseCase->>VTSAdapter: Send VTS output
    VTSAdapter-->>PipelineUseCase: OK
    PipelineUseCase-->>WSRoute: Protocol response
    WSRoute-->>Client: JSON response
~~~

Rules:

1. WebSocket message types must come from `crates/protocol`.
2. Protocol messages must include a version field.
3. Binary audio frames must follow the protocol spec if voice input is enabled.
4. MVP output adapter is VTube Studio only.
5. Unsupported frontend query values return `FRONTEND_NOT_SUPPORTED`.
6. Route handler should only handle connection setup and message forwarding.

## Route Handler Rules

1. HTTP handlers should be thin.
2. WebSocket handlers should be thin.
3. Provider logic should not live inside route handlers.
4. VTube Studio API calls should not live inside route handlers.
5. Config updates should validate before applying.
6. Secrets should never be returned in raw form.
7. WebSocket protocol should use types from `crates/protocol`.
8. Route handlers should call use cases/services, not provider SDKs.
9. Errors should use the shared API error response shape.

## Initial Migration Rule

Do not refactor everything at once.

~~~mermaid
flowchart TD
    A[Move current backend into apps/backend] --> B[Add root Cargo workspace]
    B --> C[Add crates/core protocol config providers]
    C --> D[Keep existing backend behavior working]
    D --> E[Move protocol types into crates/protocol]
    E --> F[Move config logic into crates/config]
    F --> G[Define core domain and ports in crates/core]
    G --> H[Move provider implementations into crates/providers]
    H --> I[Implement VTube Studio adapter only]
    I --> J[Add crates/runtime only when needed]
~~~

## Acceptance Criteria

Backend design is considered implemented when:

- backend runs from `apps/backend`,
- root Cargo workspace works,
- routes are thin and call use cases/services,
- provider implementations are not inside route handlers,
- VTube Studio implementation is not inside route handlers,
- public API DTOs live in `crates/protocol`,
- WebSocket DTOs live in `crates/protocol`,
- core domain and ports live in `crates/core`,
- runtime config is validated before applying,
- secret API never returns raw secret values,
- VTube Studio token is handled as secret,
- `/ws/unified?frontend=vts` works,
- unsupported frontend values return `FRONTEND_NOT_SUPPORTED`,
- Raw API and Web3D adapters are not implemented in MVP,
- admin frontend can call backend config, provider test, and VTube Studio test APIs.
