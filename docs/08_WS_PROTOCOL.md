# 08 WebSocket Protocol

## Purpose

This document defines the unified WebSocket protocol.

The unified endpoint is:

~~~text
/ws/unified
~~~

Supported frontend query values:

~~~text
/ws/unified?frontend=vts
~~~

## Protocol Rules

1. JSON messages must include `v`.
2. Initial protocol version is `1`.
3. JSON messages use `type`.
4. Binary frames are allowed for audio.
5. Public WebSocket DTOs must live in `crates/protocol`.
6. Frontend-specific output formatting must happen through frontend adapters.

## Connection Lifecycle

~~~mermaid
sequenceDiagram
    participant Client
    participant WSRoute
    participant Protocol
    participant Pipeline
    participant Adapter

    Client->>WSRoute: Connect /ws/unified?frontend=vts
    WSRoute->>Protocol: Validate query and protocol
    WSRoute-->>Client: connection.ready
    Client->>WSRoute: input message
    WSRoute->>Protocol: Decode message
    WSRoute->>Pipeline: Submit input
    Pipeline->>Adapter: Format output
    Adapter-->>WSRoute: Response frames
    WSRoute-->>Client: JSON / binary frames
    Client->>WSRoute: Close
    WSRoute->>Pipeline: Cleanup client state
~~~

## Client to Server: Text Input

~~~json
{
  "v": 1,
  "type": "input.text",
  "request_id": "uuid",
  "text": "Hello, how are you?"
}
~~~

## Client to Server: Audio Start

~~~json
{
  "v": 1,
  "type": "input.audio.start",
  "request_id": "uuid",
  "format": "pcm16",
  "sample_rate": 16000,
  "channels": 1,
  "language": "auto"
}
~~~

## Client to Server: Audio Chunk

Binary frame:

~~~text
PCM16
16000 Hz
mono
~~~

## Client to Server: Audio End

~~~json
{
  "v": 1,
  "type": "input.audio.end",
  "request_id": "uuid"
}
~~~

## Server to Client: Connection Ready

~~~json
{
  "v": 1,
  "type": "connection.ready",
  "frontend": "vts",
  "session_id": "uuid"
}
~~~

## Server to Client: Text Response

~~~json
{
  "v": 1,
  "type": "response.text",
  "request_id": "uuid",
  "text": "Hello, how can I help?"
}
~~~

## Server to Client: Transcript

~~~json
{
  "v": 1,
  "type": "response.transcript",
  "request_id": "uuid",
  "transcript": "Hello world",
  "language": "en"
}
~~~

## Server to Client: Audio Response Metadata

~~~json
{
  "v": 1,
  "type": "response.audio.start",
  "request_id": "uuid",
  "format": "wav",
  "sample_rate": 22050,
  "channels": 1
}
~~~

## Server to Client: Audio Binary

Binary frame:

~~~text
audio bytes according to response.audio.start metadata
~~~

## Server to Client: Audio End

~~~json
{
  "v": 1,
  "type": "response.audio.end",
  "request_id": "uuid"
}
~~~

## Server to Client: Visemes

~~~json
{
  "v": 1,
  "type": "response.visemes",
  "request_id": "uuid",
  "visemes": [
    {
      "time_ms": 0,
      "value": "A",
      "weight": 0.8
    }
  ]
}
~~~

## Server to Client: Error

~~~json
{
  "v": 1,
  "type": "error",
  "request_id": "uuid",
  "code": "LLM_PROVIDER_FAILED",
  "message": "LLM provider failed"
}
~~~

## Text Input Flow

~~~mermaid
sequenceDiagram
    participant Client
    participant WS
    participant Pipeline
    participant LLM
    participant TTS

    Client->>WS: input.text
    WS->>Pipeline: PipelineJob Text
    Pipeline->>LLM: Generate
    LLM-->>Pipeline: Text response
    Pipeline->>TTS: Synthesize
    TTS-->>Pipeline: Audio
    Pipeline-->>WS: Response frames
    WS-->>Client: response.text + audio frames
~~~

## Audio Input Flow

~~~mermaid
sequenceDiagram
    participant Client
    participant WS
    participant Pipeline
    participant STT
    participant LLM
    participant TTS

    Client->>WS: input.audio.start
    Client->>WS: binary audio frames
    Client->>WS: input.audio.end
    WS->>Pipeline: Audio job
    Pipeline->>STT: Transcribe
    STT-->>Pipeline: Transcript
    Pipeline->>LLM: Generate
    LLM-->>Pipeline: Text response
    Pipeline->>TTS: Synthesize
    TTS-->>Pipeline: Audio
    Pipeline-->>WS: Response frames
    WS-->>Client: text/audio/viseme frames
~~~

## Error Flow

~~~mermaid
sequenceDiagram
    participant Client
    participant WS
    participant Protocol
    participant Pipeline

    Client->>WS: Invalid message
    WS->>Protocol: Decode
    Protocol-->>WS: Protocol error
    WS-->>Client: error frame

    Client->>WS: Valid input
    WS->>Pipeline: Submit job
    Pipeline-->>WS: Provider error
    WS-->>Client: error frame
~~~

## Frontend Adapter Output Flow

~~~mermaid
flowchart TD
    PipelineOutput[Pipeline Output] --> VTSAdapter[VTube Studio Adapter]
    VTSAdapter --> VTSFrames[VTS Params + Audio]
~~~

## Protocol Versioning

The version field allows future changes.

Initial version:

~~~json
{
  "v": 1
}
~~~

Breaking protocol changes must increment the version and update this document.

## Voice Input User Notes

### Microphone Permission

When using voice input (push-to-talk), the browser will request microphone access. If permission is denied, the client will display an error message: "Microphone permission denied. Please allow microphone access in your browser settings."

To grant permission:
- Click the microphone icon or push-to-talk button
- When prompted, allow microphone access
- Refresh the page if permission was previously denied

### Push-to-Talk Flow

1. **Start Recording**: Press and hold the push-to-talk button. The button will turn red and pulse while recording.
2. **Speak**: Release the button when finished speaking.
3. **Processing**: Audio is converted to PCM16 format (16kHz, mono) and sent to the backend for transcription.
4. **Response**: The transcript is sent to the LLM, and the response is synthesized to audio and sent back with visemes for lip-sync.

The text input path remains available and independent of voice input.
