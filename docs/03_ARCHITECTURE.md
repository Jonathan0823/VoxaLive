# 03 Architecture

## Architecture Overview

VoxaLive uses a monorepo structure and a backend architecture based on Hexagonal Architecture / Ports and Adapters.

The system is divided into:

- runtime data plane,
- admin control plane,
- external provider adapters,
- external frontend adapters.

## System Context

~~~mermaid
flowchart TB
    User[Admin / Operator] --> AdminWeb[Admin Web UI]
    AdminWeb --> Backend[VoxaLive Backend]

    VoiceClient[Voice Client] --> Backend
    TextClient[Text Client] --> Backend
    Youtube[YouTube Live] --> Backend
    TikTok[TikTok Live] --> Backend

    Backend --> Gemini[Gemini]
    Backend --> OpenRouter[OpenRouter]
    Backend --> Ollama[Ollama]
    Backend --> Whisper[faster-whisper]
    Backend --> Qwen[Qwen TTS]
    Backend --> Piper[Piper TTS]

    Backend --> Raw[Raw API Client]
    Backend --> VTS[VTube Studio]
    Backend --> Web3D[Web3D Client]
~~~

## Runtime Data Plane

The runtime data plane handles realtime input and output.

~~~mermaid
flowchart LR
    Inputs[Input Adapters] --> Queue[Pipeline Queue]
    Queue --> Worker[Pipeline Worker]
    Worker --> STT[STT Port]
    Worker --> LLM[LLM Port]
    Worker --> TTS[TTS Port]
    Worker --> Frontend[Frontend Adapter Port]

    STT --> STTAdapter[Whisper Adapter]
    LLM --> LLMAdapters[Gemini / OpenRouter / Ollama]
    TTS --> TTSAdapters[Qwen / Piper]
    Frontend --> OutputAdapters[Raw / VTS / Web3D]
~~~

## Admin Control Plane

The admin control plane manages runtime settings and provider tests.

~~~mermaid
flowchart TD
    Admin[Admin User] --> AdminWeb[Admin Web UI]
    AdminWeb --> HealthAPI[GET /api/health]
    AdminWeb --> ConfigAPI[GET/PATCH /api/config]
    AdminWeb --> SecretsAPI[GET/PUT /api/secrets]
    AdminWeb --> TestAPI[POST /api/test/llm or /tts]

    HealthAPI --> Metrics[MetricsState]
    ConfigAPI --> ConfigManager[ConfigManager]
    SecretsAPI --> SecretManager[SecretManager]
    TestAPI --> ProviderRegistry[ProviderRegistry]
    ConfigManager --> ProviderRegistry
~~~

## Hexagonal Boundary

~~~mermaid
flowchart TB
    subgraph Core["crates/core"]
        Domain[Domain Types]
        Ports[Ports / Traits]
        Services[Core Services]
        Domain --> Services
        Ports --> Services
    end

    subgraph Delivery["apps/backend"]
        HTTP[HTTP Routes]
        WS[WebSocket Routes]
        Static[Static Admin UI Serving]
    end

    subgraph Adapters["crates/providers + crates/config"]
        LLM[LLM Adapters]
        TTS[TTS Adapters]
        STT[STT Adapters]
        Live[Live Input Adapters]
        Frontend[Frontend Output Adapters]
        Config[Config Store Adapter]
        Secrets[Secret Store Adapter]
    end

    HTTP --> Services
    WS --> Services
    Services --> Ports
    Ports --> Adapters
~~~

## Control Plane vs Data Plane

~~~mermaid
flowchart LR
    subgraph ControlPlane["Control Plane"]
        AdminWeb[Admin Web]
        API[Admin API]
        Config[Config Manager]
        Secrets[Secret Manager]
        Registry[Provider Registry]
        AdminWeb --> API --> Config --> Registry
        API --> Secrets
    end

    subgraph DataPlane["Data Plane"]
        Inputs[Voice/Text/Live Inputs]
        Queue[Pipeline Queue]
        Worker[Pipeline Worker]
        Outputs[Frontend Outputs]
        Inputs --> Queue --> Worker --> Outputs
    end

    Registry -. selects active providers .-> Worker
    Config -. runtime settings .-> Worker
~~~

## Dependency Direction

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
~~~

## Architecture Rules

1. Core defines ports.
2. Providers implement ports.
3. Routes call services.
4. Services depend on ports.
5. Public DTOs live in `crates/protocol`.
6. Config and secrets live in `crates/config`.
7. Provider SDKs live in `crates/providers`.
8. Admin frontend talks only to backend APIs.
9. Secrets never flow back to frontend as raw values.
10. Diagrams in docs must use Mermaid JS.
