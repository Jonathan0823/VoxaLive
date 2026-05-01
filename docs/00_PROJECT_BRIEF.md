# 00 Project Brief

## Product Name

VoxaLive

## One Sentence Description

VoxaLive is a realtime voice/avatar AI runtime that accepts voice, text, and live comments, processes them through STT, LLM, and TTS providers, then sends responses to frontend adapters.

## Current State

The project currently focuses on backend functionality. The new direction is to migrate it into a monorepo with a simple admin web UI.

## New Direction

The repository should become a monorepo:

~~~text
VoxaLive Monorepo
- Rust backend runtime
- Simple admin web UI
- Shared Rust crates
- Documentation-first AI agent workflow
~~~

## Why Monorepo

The monorepo is needed because the project should include a small admin interface for:

- selecting LLM provider,
- selecting model,
- configuring TTS/STT,
- configuring live inputs,
- testing providers,
- checking backend status.

The admin web UI is not a full avatar client.

## Product Context

VoxaLive supports multiple input sources and output adapters.

~~~mermaid
flowchart LR
    Voice[Voice Input] --> Backend[VoxaLive Backend]
    Text[Text Input] --> Backend
    Youtube[YouTube Live Comments] --> Backend
    TikTok[TikTok Live Comments] --> Backend

    Backend --> Raw[Raw API Client]
    Backend --> VTS[VTube Studio Adapter]
    Backend --> Web3D[Web3D Adapter]
~~~

## Target Users

- Developer running a local realtime AI avatar backend.
- Streamer using VTube Studio or Web3D client.
- Admin/operator configuring providers and runtime settings.
- Future developer adding new LLM/TTS/STT/live adapters.

## MVP Goal

Build a monorepo with:

- Rust backend in `apps/backend`.
- Simple admin web UI in `apps/admin-web`.
- Shared Rust crates in `crates/`.
- Stable API and WebSocket contracts.
- Secure config and secret handling.
- Documentation suitable for AI agent handoff.

## MVP Non-Goals

The first MVP does not include:

- full Web3D avatar renderer,
- mobile app,
- desktop app,
- multi-tenant cloud dashboard,
- payment or billing,
- advanced user account system,
- plugin marketplace.

## Core Design Decision

The backend uses Hexagonal Architecture / Ports and Adapters.

This is because VoxaLive depends on many replaceable external components:

- Gemini,
- OpenRouter,
- Ollama,
- Qwen,
- Piper,
- faster-whisper,
- YouTube,
- TikTok,
- VTube Studio,
- Web3D clients.

The core backend must stay independent from these implementations.
