# 09 Config and Secrets

## Purpose

This document defines how runtime config and secrets should work.

The admin frontend can update configuration, but secrets must be handled safely.

## Config Categories

### Default Config

Hardcoded safe defaults.

Examples:

- default bind address,
- default LLM provider,
- default TTS mode,
- default token limits.

### Environment Config

Loaded from environment variables or `.env`.

Examples:

- `ADMIN_TOKEN`
- `GEMINI_API_KEY`
- `OPENROUTER_API_KEY`
- `OLLAMA_URL`

### Runtime Config

Updated through admin API and stored by config store.

Examples:

- selected LLM provider,
- selected model,
- temperature,
- max tokens,
- TTS mode,
- STT device,
- live input settings.

## Config Precedence

~~~mermaid
flowchart TD
    Default[Default Config] --> Merge[Config Merge]
    Env[Environment Config] --> Merge
    Runtime[Runtime Config] --> Merge
    Merge --> Effective[Effective Config]

    Runtime -. highest priority .-> Effective
    Env -. middle priority .-> Effective
    Default -. lowest priority .-> Effective
~~~

Priority:

~~~text
Runtime config > Environment config > Default config
~~~

## Secret Rules

1. Raw secrets must never be returned from API responses.
2. Secret status may return only configured/not configured.
3. Frontend must not store secrets in localStorage.
4. Secret update endpoint may accept raw secret values.
5. Logs must not include raw secret values.
6. Secret values should be accessed through `SecretManager`.
7. Provider adapters should receive secrets through controlled backend services.
8. VTube Studio auth tokens must be treated as secrets.

## Secret Status Shape

~~~json
{
  "gemini_api_key": {
    "configured": true
  },
  "openrouter_api_key": {
    "configured": false
  },
  "admin_token": {
    "configured": true
  }
}
~~~

## Secret Update Flow

~~~mermaid
sequenceDiagram
    participant AdminWeb
    participant API
    participant Auth
    participant SecretManager
    participant SecretStore
    participant ProviderRegistry

    AdminWeb->>API: PUT /api/secrets
    API->>Auth: Validate admin token
    Auth-->>API: Authorized
    API->>SecretManager: Validate secret update
    SecretManager->>SecretStore: Store secret
    SecretManager->>ProviderRegistry: Notify secret update
    ProviderRegistry-->>SecretManager: Providers can refresh
    SecretManager-->>API: Updated keys only
    API-->>AdminWeb: Success without raw secrets
~~~

## Runtime Config Update Flow

~~~mermaid
sequenceDiagram
    participant AdminWeb
    participant API
    participant ConfigManager
    participant ConfigStore
    participant ProviderRegistry

    AdminWeb->>API: PATCH /api/config
    API->>ConfigManager: Validate update
    ConfigManager->>ConfigStore: Save runtime config
    ConfigManager->>ProviderRegistry: Refresh provider selection
    ProviderRegistry-->>ConfigManager: Active providers updated
    ConfigManager-->>API: Effective config
    API-->>AdminWeb: Config response
~~~

## Provider Switch Flow

~~~mermaid
flowchart TD
    Patch[PATCH /api/config] --> Validate[Validate Config]
    Validate --> Save[Save Runtime Config]
    Save --> Registry[Provider Registry]
    Registry --> ProviderKind{Provider Kind}
    ProviderKind --> Gemini[Gemini]
    ProviderKind --> OpenRouter[OpenRouter]
    ProviderKind --> Ollama[Ollama]
    ProviderKind --> Piper[Piper]
    ProviderKind --> Qwen[Qwen]
    Gemini --> Active[Set Active Provider]
    OpenRouter --> Active
    Ollama --> Active
    Piper --> Active
    Qwen --> Active
~~~

## Example .env

~~~env
BIND_ADDR=0.0.0.0:8080
ADMIN_UI_ENABLED=true
ADMIN_TOKEN=change-me

LLM_PROVIDER=gemini
LLM_MODEL=gemini-2.0-flash-exp
GEMINI_API_KEY=
OPENROUTER_API_KEY=
OLLAMA_URL=http://localhost:11434

TTS_MODE=cpu
TTS_PROVIDER=piper
TTS_MODEL_PATH=./voices/default.onnx

STT_DEVICE=cpu
CONFIG_STORE=file
CONFIG_FILE=./data/config.runtime.json
~~~

## Runtime Config Example

~~~json
{
  "llm": {
    "provider": "gemini",
    "model": "gemini-2.0-flash-exp",
    "temperature": 0.7,
    "max_tokens": 1024
  },
  "tts": {
    "mode": "cpu",
    "provider": "piper",
    "model_path": "./voices/default.onnx"
  },
  "stt": {
    "device": "cpu"
  },
  "live": {
    "enabled": false,
    "youtube_video_id": "",
    "tiktok_room": ""
  }
}
~~~

## Validation Rules

### LLM

- provider must be one of: `gemini`, `openrouter`, `ollama`
- temperature must be between `0.0` and `2.0`
- max tokens must be greater than `0`
- model name must not be empty

### TTS

- mode must be one of: `cpu`, `gpu`, `auto`
- provider must be valid
- model path should be validated if provider requires it

### STT

- device must be one of: `cpu`, `cuda:0`, `auto`

### Live Inputs

- disabled live inputs may have empty config
- enabled YouTube input requires video ID
- enabled TikTok input requires room/user identifier

## Logging Rules

Never log:

- API keys,
- admin token,
- full Authorization header,
- raw secret update body.

Safe to log:

- provider name,
- secret configured true/false,
- config validation error without raw secret,
- latency,
- request ID,
- protocol version.
