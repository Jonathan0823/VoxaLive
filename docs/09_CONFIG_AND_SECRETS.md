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

## Runtime Config Persistence

Runtime config is persisted to a local JSON file so settings survive backend restarts.

### Storage

- File: `data/config.runtime.json` (configurable via `CONFIG_FILE` env var)
- Format: pretty-printed JSON
- Contents: all runtime settings (`llm`, `tts`, `stt`, `live`, `server`)

### Behavior

On startup, the backend:
1. Starts from default runtime config
2. Loads persisted runtime config from `data/config.runtime.json` if present
3. Applies admin UI `PATCH /api/config` updates in-memory and persists them immediately

This means provider/model changes made in the admin UI are retained after refresh/restart.

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

## Secret Persistence

Secrets can be managed via the admin UI at `/secrets` and are persisted to an encrypted file.

### Storage

- File: `data/secrets.enc` (configurable via `SECRETS_FILE` env var)
- Encryption: AES-256-GCM
- Key derivation: from `SECRETS_MASTER_KEY` env var
- Fallback key (dev only): `voxalive-dev-key-change-in-prod` — **never use in production**

### Bootstrap vs Runtime

On startup, the backend:
1. Loads persisted secrets from `data/secrets.enc` (if exists)
2. Overlays any env var values found in `.env` (for bootstrap rotation)
3. Persists the merged state back to disk

This means:
- **Env vars** are for initial bootstrap / first-run defaults
- **UI at `/secrets`** is for day-to-day management and key rotation
- Env values override persisted values on every restart (rotation use case)

### Safety Rules

1. Raw secrets must never be returned from API responses.
2. Secret status may return only configured/not configured.
3. Frontend must not store secrets in localStorage.
4. Secret update endpoint accepts raw secret values, backend stores encrypted.
5. Logs must not include raw secret values.
6. Secret values are accessed through `SecretManager`.
7. Provider adapters receive secrets through controlled backend services.
8. VTube Studio auth tokens are treated as secrets.
9. `SECRETS_MASTER_KEY` must be set in production.

### Secret Keys

| Key | Description | Source |
|-----|-------------|--------|
| `gemini_api_key` | Gemini API key | env or `/secrets` |
| `openrouter_api_key` | OpenRouter API key | env or `/secrets` |
| `admin_token` | Admin UI auth token | env or `/secrets` |
| `vts_auth_token` | VTube Studio auth token | env or `/secrets` |

## Secret Status Shape

~~~json
{
  "secrets": {
    "gemini_api_key": {
      "configured": true
    },
    "openrouter_api_key": {
      "configured": false
    },
    "admin_token": {
      "configured": true
    },
    "vts_auth_token": {
      "configured": false
    }
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
    participant EncryptedFile[Encrypted Secrets File]
    participant ProviderRegistry

    AdminWeb->>API: PUT /api/secrets
    API->>Auth: Validate admin token
    Auth-->>API: Authorized
    API->>SecretManager: Update secrets
    SecretManager->>EncryptedFile: Persist (AES-256-GCM)
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
# Admin auth
ADMIN_TOKEN=change-me

# LLM provider keys (optional -- prefer UI at /secrets)
GEMINI_API_KEY=
OPENROUTER_API_KEY=

# VTube Studio auth
VTS_AUTH_TOKEN=

# Static file serving
STATIC_DIR=apps/admin-web/dist

# Runtime config persistence
CONFIG_FILE=data/config.runtime.json

# Secret persistence encryption key (required in production)
SECRETS_MASTER_KEY=your-production-key-here
SECRETS_FILE=data/secrets.enc
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
