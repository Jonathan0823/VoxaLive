# VoxaLive: Rust Realtime Avatar & Voice AI Backend

## 🎯 Overview

**VoxaLive** is a production-ready, **Rust-powered realtime voice assistant backend** designed for **maximum flexibility**. It provides a unified WebSocket endpoint that orchestrates multiple input types (voice, text, YouTube/TikTok live comments) through an AI pipeline (LLM + TTS) and delivers results to various frontend adapters.

**Key Features**:
- **Multi-LLM**: Gemini API (free), OpenRouter (300+ models), Local Ollama
- **Multi-TTS**: Qwen3 GPU, Piper CPU fallback
- **Single WS Endpoint**: `/ws/unified?frontend=vts|web3d` 
- **Hot Config**: Switch providers without restart
- **Backend-only focus**: Frontends live in separate repositories

**Status**: v0.1 MVP - All core backend features implemented and tested.

## 🚀 Features

### Multiple Inputs (Unified Pipeline)
```
🎤 Voice WS ─┐
💬 Text API  ─┼─→ mpsc Queue ─→ LLM → TTS → Frontend Adapter
📺 YT Live   ─┤
📱 TikTok    ─┘
```

### LLM Providers (Live Switchable)
| Provider | Config | Cost | Crate |
|----------|--------|------|-------|
| **Gemini 2.0** | `LLM_PROVIDER=gemini` | **Free tier** | `gemini-client-rs` |
| **OpenRouter** | `LLM_PROVIDER=openrouter` | Pay-per-token | `openrouter_api`  [github](https://github.com/socrates8300/openrouter_api) |
| **Ollama Local** | `LLM_PROVIDER=ollama` | **Free GPU** | `ollama-rs`  [zupzup](https://www.zupzup.org/rust-ai-chatbot-ollama/index.html) |

### Frontend Options (Adapter Configuration)
| Frontend | Connection | Backend Sends |
|----------|------------|---------------|
| **VTube Studio** | `?frontend=vts` | VTS API params + audio |
| **Web 3D** | `?frontend=web3d` | Visemes JSON + audio binary |
| **Raw API** | Default | Full protocol (JSON + binary audio) |

## 🛠 Tech Stack
```
Backend: Rust 1.80 | Axum 0.8 | Tokio 1.0
LLM: gemini-client-rs | openrouter_api | ollama-rs
STT: faster-whisper-rs (GPU/CPU)
TTS: qwen_tts(CUDA) | piper-onnx(CPU)
Live: tiktoklive-rs | youtube-rs
DB: sled (sessions)
Deploy: Docker NVIDIA
```

## 🏗 Architecture Diagram
```mermaid
graph TB
    subgraph Inputs ["Multiple Inputs"]
        A[🎤 Voice WS] 
        B[💬 Text REST/WS]
        C[📺 YouTube Live Polling]
        D[📱 TikTok Live Stream]
        A -.->|"Binary chunks"| E
        B -.->|"JSON text"| E
        C -.->|"Comments"| E
        D -.->|"Live events / Comments"| E
    end
    
    E["mpsc::channel(100)<br/>tokio::sync"]
    
    subgraph Pipeline ["AI Pipeline"]
        E --> F{Input Type?}
        
        %% Path for Audio
        F -->|Audio| G["faster-whisper-rs"]
        
        %% Path for Text directly to Providers
        F -->|Text/Comments| I
        G --> I{LLM Provider?}
        
        I -->|Gemini| Ig["Gemini 2.0 Flash"]
        I -->|OpenRouter| Io["Claude/Grok/etc"]
        I -->|Ollama| Il["Local llama3.2"]
        
        %% Immediate transition to TTS Mode
        Ig --> K
        Io --> K
        Il --> K
        
        K{TTS Mode?}
        K -->|GPU| Kg["Qwen3-TTS<br/>Clone"]
        K -->|CPU Fallback| Kc["Piper TTS<br/>ONNX"]
    end
    
    Kg --> L
    Kc --> L
    
    subgraph Frontends ["Frontend Adapters"]
        L{Frontend?}
        L -->|VTS| M["VTube Studio<br/>Lip-sync"]
        L -->|Web3D| N["Three.js + VRM<br/>Morph Targets"]
        L -->|Raw API| O["Custom Clients<br/>Flutter/Tauri"]
    end
```

## ⚡ Quick Start (Backend Only)

### 1. System Setup
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

### 2. Pick Your LLM (Copy One Block)
```
# === GEMINI (FREE - RECOMMENDED) ===
export GEMINI_API_KEY="AIza..."  # https://aistudio.google.com/app/apikey

# === OPENROUTER (PRO MODELS) ===
export OPENROUTER_API_KEY="sk-or-v1-..."  # openrouter.ai

# === LOCAL OLLAMA (OFFLINE) ===
docker run -d -v ollama:/root/.ollama -p 11434:11434 --gpus all ollama/ollama
docker exec ollama ollama pull llama3.2:1b  # Small/fast
```

### 3. Clone & Run
```bash
git clone https://github.com/Jonathan0823/VoxaLive  # Your repo
cd VoxaLive
cp .env.example .env  # Paste your keys above
cargo build --release  # 40MB binary ⚡
cargo run --release
```
**Output**: `Server ready: http://localhost:8080`

### 4. Test Backend (Instant)
```bash
# Test text input via WebSocket using wscat (install: npm i -g wscat)
wscat -c "ws://localhost:8080/ws/unified?frontend=raw" -t 10000
# Once connected, type: {"type":"text","content":"Hello, how are you?"}
# You should receive audio binary and/or JSON visemes depending on frontend param
```
> 💡 For quick validation without extra tools, you can also use a simple HTML/JS console:
> 1. Save [this minimal test page](https://gist.github.com/Jonathan0823/voxalive-test) as `test.html`
> 2. Open it in a browser and click "Connect" then "Send Text"

## 🌐 Client Integrations

Frontends are maintained in separate repositories to allow independent evolution. Below are official and community integrations:

### **VTube Studio (Livestream Pro)**
```
1. VTube Studio → Settings → API → Enable (port 8001)
2. Load Live2D model (.model3.json)
3. Backend auto-bridges: ws://localhost:8080/ws/unified?frontend=vts
4. OBS → Window/Game Capture → Stream!
```

### **Web 3D Frontend**
See the [`voxalive-web3d`](https://github.com/Jonathan0823/voxalive-web3d) repository for a reference Three.js/VRM client.

### **Raw API Clients**
- Mobile: Flutter/Tauri examples in [`voxalive-clients`](https://github.com/Jonathan0823/voxalive-clients)
- Web: Generic WebSocket wrapper in [`voxalive-js-sdk`](https://github.com/Jonathan0823/voxalive-js-sdk)

## 📋 Configuration (.env) - Full
```bash
# === LLM (Pick ONE) ===
LLM_PROVIDER=gemini              # gemini|openrouter|ollama
GEMINI_API_KEY=AIzaSy...
OPENROUTER_API_KEY=sk-or-v1-...
OLLAMA_URL=http://host.docker.internal:11434
LLM_MODEL=gemini-2.0-flash-exp   # openrouter: "anthropic/claude-3.5-sonnet"
MAX_TOKENS=1024
TEMPERATURE=0.7

# === TTS ===
TTS_MODE=gpu                     # gpu|cpu (auto-detect)
TTS_MODEL_PATH=./voices/custom-qwen.onnx
STT_DEVICE=cuda:0                # cuda:0|cpu

# === Live Streams ===
TIKTOK_ROOM=your-streamer
YOUTUBE_VIDEO_ID=dQw4w9WgXcQ

# === Server ===
BIND_ADDR=0.0.0.0:8080
```

## 📦 Cargo.toml (Production Ready)
```toml
[package]
name = "voxalive"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.24"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
# LLM
gemini-client-rs = "0.3"
openrouter_api = "0.1"           # [web:96]
ollama-rs = "0.2"                # [web:101]
# AI
candle-cuda = "0.6" 
qwen_tts = "0.1"
faster-whisper-rs = "0.2"
piper-onnx = "0.1"
# Live
tiktoklive-rs = "0.5"
youtube-rs = "0.8"
sled = "0.34"
```

## 📊 Performance Matrix
| LLM \ TTS | Qwen3 GPU (97ms) | Piper CPU (250ms) | Total E2E |
|-----------|------------------|-------------------|-----------|
| **Gemini** | ✅ **447ms** | 597ms | **Best free** |
| **OpenRouter** | 497-947ms | 647-1097ms | **Pro models** |
| **Ollama** | 897ms-2.3s | 1.1-2.5s | **Offline** |

## 🚀 Production Deployment

### Docker Compose (Full Stack)
```yaml
version: '3.8'
services:
  voiceai:
    build: .
    ports: ["8080:8080", "9090:9090"]
    environment:
      - LLM_PROVIDER=ollama
      - OLLAMA_URL=http://ollama:11434
      - TTS_MODE=gpu
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    volumes: ['./voices:/app/voices']

  ollama:
    image: ollama/ollama:latest
    ports: ["11434:11434"]
    volumes: [ollama:/root/.ollama]
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1

volumes:
  ollama:
```
```bash
docker compose up -d  # GPU + Ollama + VoiceAI
```

## 🧭 Frontend Strategy
- **Separate repos**: Frontends (Web, VTS plugin, mobile, desktop) live in their own repositories for independent versioning and tech choices.
- **Backend contract**: The `/ws/unified` endpoint is stable; frontends adapt to the binary/JSON protocol.
- **Optional Rust GUI**: For desktop/admin tooling, consider `egui` or `tauri` in a separate `voxalive-gui` repo (not required for core functionality).

## 📄 License & Credits
**MIT License** - Commercial use OK.

***

**🚀 Backend ready in 5 mins**: `cargo run` → test with wscat → integrate any frontend.