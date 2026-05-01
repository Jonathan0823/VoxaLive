# 07 API Contract

## Purpose

This document defines REST API contracts between the admin frontend and backend.

All public API DTOs should live in `crates/protocol`.

## Base URL

Development:

~~~text
http://localhost:8080
~~~

Production:

~~~text
same origin as backend
~~~

## Auth

Admin endpoints require:

~~~http
Authorization: Bearer <ADMIN_TOKEN>
~~~

Public endpoint:

- `GET /api/health`

Protected endpoints:

- `GET /api/config`
- `PATCH /api/config`
- `GET /api/secrets/status`
- `PUT /api/secrets`
- `POST /api/test/llm`
- `POST /api/test/tts`
- `POST /api/test/vts`

## API Overview

~~~mermaid
flowchart TD
    AdminWeb[Admin Web UI] --> Health[GET /api/health]
    AdminWeb --> ConfigGet[GET /api/config]
    AdminWeb --> ConfigPatch[PATCH /api/config]
    AdminWeb --> SecretStatus[GET /api/secrets/status]
    AdminWeb --> SecretPut[PUT /api/secrets]
    AdminWeb --> TestLLM[POST /api/test/llm]
    AdminWeb --> TestTTS[POST /api/test/tts]
    AdminWeb --> TestVTS[POST /api/test/vts]

    ConfigGet --> ConfigManager[ConfigManager]
    ConfigPatch --> ConfigManager
    SecretStatus --> SecretManager[SecretManager]
    SecretPut --> SecretManager
    TestLLM --> ProviderRegistry[ProviderRegistry]
    TestTTS --> ProviderRegistry
    TestVTS --> ProviderRegistry
~~~

## Standard Success Response

~~~json
{
  "ok": true,
  "data": {}
}
~~~

## Standard Error Response

~~~json
{
  "ok": false,
  "error": {
    "code": "CONFIG_VALIDATION_FAILED",
    "message": "Invalid LLM provider"
  }
}
~~~

## Error Codes

Initial error codes:

- `UNAUTHORIZED`
- `FORBIDDEN`
- `CONFIG_VALIDATION_FAILED`
- `SECRET_UPDATE_FAILED`
- `PROVIDER_NOT_CONFIGURED`
- `PROVIDER_TEST_FAILED`
- `WS_PROTOCOL_ERROR`
- `INTERNAL_ERROR`

## GET /api/health

Returns backend health.

### Response

~~~json
{
  "ok": true,
  "data": {
    "status": "healthy",
    "version": "0.1.0",
    "uptime_sec": 123,
    "active_llm_provider": "gemini",
    "active_tts_provider": "piper"
  }
}
~~~

## GET /api/config

Returns current runtime config.

Must not include raw secrets.

### Response

~~~json
{
  "ok": true,
  "data": {
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
      "youtube_video_id": "",
      "tiktok_room": "",
      "enabled": false
    },
    "server": {
      "admin_ui_enabled": true
    }
  }
}
~~~

## PATCH /api/config

Updates runtime config.

### Request

~~~json
{
  "llm": {
    "provider": "openrouter",
    "model": "anthropic/claude-3.5-sonnet",
    "temperature": 0.6,
    "max_tokens": 1024
  }
}
~~~

### Response

~~~json
{
  "ok": true,
  "data": {
    "updated": true
  }
}
~~~

## GET /api/secrets/status

Returns whether secrets are configured.

Must not return raw secret values.

### Response

~~~json
{
  "ok": true,
  "data": {
    "secrets": {
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
  }
}
~~~

## PUT /api/secrets

Updates secrets.

### Request

~~~json
{
  "gemini_api_key": "AIza...",
  "openrouter_api_key": "sk-or-v1-..."
}
~~~

### Response

~~~json
{
  "ok": true,
  "data": {
    "updated": [
      "gemini_api_key",
      "openrouter_api_key"
    ]
  }
}
~~~

## POST /api/test/llm

Tests the active or selected LLM provider.

### Request

~~~json
{
  "message": "Hello, test response only.",
  "provider": "gemini"
}
~~~

### Response

~~~json
{
  "ok": true,
  "data": {
    "provider": "gemini",
    "model": "gemini-2.0-flash-exp",
    "latency_ms": 421,
    "text": "Hello! The provider is working."
  }
}
~~~

## POST /api/test/tts

Tests the active or selected TTS provider.

### Request

~~~json
{
  "text": "Hello, this is a voice test.",
  "provider": "piper"
}
~~~

### Response

~~~json
{
  "ok": true,
  "data": {
    "provider": "piper",
    "latency_ms": 250,
    "audio_format": "wav"
  }
}
~~~

## POST /api/test/vts

Tests the VTube Studio connection.

### Request

~~~json
{}
~~~

### Response

~~~json
{
  "ok": true,
  "data": {
    "connected": true,
    "latency_ms": 120
  }
}
~~~

## Config Update Flow

~~~mermaid
sequenceDiagram
    participant AdminWeb
    participant API
    participant Auth
    participant ConfigService
    participant ConfigStore
    participant ProviderRegistry

    AdminWeb->>API: PATCH /api/config
    API->>Auth: Validate bearer token
    Auth-->>API: Authorized
    API->>ConfigService: Validate request
    ConfigService->>ConfigStore: Save runtime config
    ConfigService->>ProviderRegistry: Refresh provider selection
    ProviderRegistry-->>ConfigService: OK
    ConfigService-->>API: Updated config result
    API-->>AdminWeb: Success response
~~~

## API Rules

1. API responses must use the standard response shape.
2. API errors must use the standard error shape.
3. Raw secrets must never be returned.
4. Protected endpoints require admin token.
5. Route handlers should call services.
6. Route handlers must not directly call provider SDKs.
7. DTOs should live in `crates/protocol`.
