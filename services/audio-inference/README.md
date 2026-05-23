# Audio Inference Service

Unified local audio service for VoxaLive that handles both STT (Whisper) and TTS (Qwen3-TTS).

## Purpose

This service keeps CUDA build complexity out of the Rust workspace by running audio inference in a separate Python process. The Rust backend orchestrates by calling this service via HTTP.

## Architecture

```
┌─────────────────┐     HTTP      ┌──────────────────────┐
│  Rust Backend   │ ─────────────>│  Audio Inference    │
│  (Orchestrator) │               │  Service (Python)   │
└─────────────────┘               └──────────────────────┘
                                              │
                    ┌──────────────────────────┼──────────────────────────┐
                    │                          │                          │
              ┌─────▼─────┐             ┌──────▼──────┐           ┌──────▼──────┐
              │  Whisper  │             │  Qwen TTS   │           │  Piper TTS  │
              │   (STT)   │             │   (TTS)     │           │   (TTS)     │
              └───────────┘             └─────────────┘           └─────────────┘
```

## API Endpoints

### STT: POST /stt/transcribe

Transcribe audio to text using Whisper.

**Request:**
```json
{
  "audio_format": "pcm16",
  "sample_rate": 16000,
  "channels": 1,
  "language": "auto",
  "audio_data": "<base64 encoded audio>"
}
```

**Response:**
```json
{
  "transcript": "Hello world"
}
```

### TTS: POST /tts/synthesize

Synthesize text to audio using Qwen3-TTS (or Piper as fallback).

**Request:**
```json
{
  "text": "Hello world",
  "voice_id": "default",
  "format": "wav"
}
```

**Response:**
```json
{
  "audio_data": "<base64 encoded audio>",
  "sample_rate": 22050,
  "channels": 1,
  "format": "wav"
}
```

## Running the Service

```bash
# Install dependencies
pip install -r requirements.txt

# Run the service
python -m app.main

# The service will start on http://127.0.0.1:8002
```

## Configuration

Environment variables:
- `STT_MODEL` - Whisper model to use (default: "base")
- `STT_DEVICE` - Device for inference (default: "cuda")
- `TTS_MODEL` - TTS model to use (default: "qwen3-tts")
- `PORT` - Service port (default: 8002)

## Development Status

- [x] STT endpoint structure (stub)
- [ ] STT endpoint implementation (Whisper)
- [ ] TTS endpoint structure (stub)
- [ ] TTS endpoint implementation (Qwen3-TTS)

## Integration with Backend

The Rust backend calls this service via HTTP. The provider factory creates a `WhisperAdapter` that:
1. Encodes audio as base64
2. POSTs to `/stt/transcribe`
3. Parses the JSON response for the transcript

The service URL is configured via `STT_SERVICE_URL` in the runtime config.