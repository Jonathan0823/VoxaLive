# 01 Product Specification

## Product Goal

VoxaLive provides a realtime AI pipeline for voice, text, and live comments.

The backend accepts input from multiple sources, processes it through AI providers, and returns responses to selected frontend adapters.

## MVP Product Scope

The MVP includes:

- unified WebSocket endpoint,
- text input processing,
- voice input processing interface,
- live comment processing interface,
- LLM provider selection,
- TTS provider selection,
- runtime config API,
- secret status/update API,
- simple admin web UI,
- provider test API,
- static admin UI serving in production.

## Core User Flows

### Flow 1: Admin Selects LLM Provider

~~~mermaid
sequenceDiagram
    participant Admin
    participant AdminWeb
    participant Backend
    participant ConfigManager
    participant ProviderRegistry

    Admin->>AdminWeb: Open /admin
    AdminWeb->>Backend: GET /api/config
    Backend->>ConfigManager: Read current config
    ConfigManager-->>Backend: Runtime config
    Backend-->>AdminWeb: Config response
    Admin->>AdminWeb: Select provider/model
    AdminWeb->>Backend: PATCH /api/config
    Backend->>ConfigManager: Validate and apply config
    ConfigManager->>ProviderRegistry: Notify active provider change
    Backend-->>AdminWeb: Updated config
~~~

### Flow 2: Client Sends Text via WebSocket

~~~mermaid
sequenceDiagram
    participant Client
    participant WSRoute
    participant Pipeline
    participant LLM
    participant TTS
    participant Adapter

    Client->>WSRoute: Connect /ws/unified?frontend=raw
    Client->>WSRoute: input.text
    WSRoute->>Pipeline: Submit text input
    Pipeline->>LLM: Generate response
    LLM-->>Pipeline: Text response
    Pipeline->>TTS: Synthesize audio
    TTS-->>Pipeline: Audio + optional visemes
    Pipeline->>Adapter: Format for frontend
    Adapter-->>Client: response.text / binary audio / visemes
~~~

### Flow 3: Audio Input Processing

~~~mermaid
sequenceDiagram
    participant Client
    participant WSRoute
    participant Pipeline
    participant STT
    participant LLM
    participant TTS

    Client->>WSRoute: input.audio.start
    Client->>WSRoute: binary audio chunks
    WSRoute->>Pipeline: Submit audio input
    Pipeline->>STT: Transcribe audio
    STT-->>Pipeline: Text transcript
    Pipeline->>LLM: Generate response
    LLM-->>Pipeline: Text response
    Pipeline->>TTS: Synthesize response
    TTS-->>Pipeline: Audio response
    Pipeline-->>Client: Response frames
~~~

### Flow 4: Live Comment Processing

~~~mermaid
sequenceDiagram
    participant LiveProvider
    participant LiveInputAdapter
    participant PipelineQueue
    participant Worker
    participant LLM
    participant TTS
    participant FrontendAdapter

    LiveProvider->>LiveInputAdapter: New comment event
    LiveInputAdapter->>PipelineQueue: Enqueue live comment
    PipelineQueue->>Worker: Dequeue job
    Worker->>LLM: Generate response
    LLM-->>Worker: Text response
    Worker->>TTS: Synthesize audio
    TTS-->>Worker: Audio response
    Worker->>FrontendAdapter: Format output
~~~

## MVP Features

### Backend

- `GET /api/health`
- `GET /api/config`
- `PATCH /api/config`
- `GET /api/secrets/status`
- `PUT /api/secrets`
- `POST /api/test/llm`
- `POST /api/test/tts`
- `GET /ws/unified`

### Admin Web

- Dashboard page
- Provider settings page
- Voice/TTS settings page
- Live input settings page
- Test console page

### Protocol

- WebSocket protocol version field
- JSON messages for text and metadata
- binary frames for audio
- shared API error shape

## Non-Goals

The MVP does not include:

- full avatar rendering,
- advanced OBS/VTube Studio UI,
- cloud multi-user management,
- billing,
- analytics dashboard,
- plugin marketplace,
- advanced permissions.

## Success Criteria

The MVP is successful when:

- backend runs from monorepo,
- admin UI can configure runtime settings,
- secrets are not leaked,
- unified WebSocket still works,
- provider implementations are behind ports,
- docs are sufficient for AI agent continuation.
