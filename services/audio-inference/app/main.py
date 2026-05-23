from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.config import AudioServiceConfig, load_config
from app.stt import router as stt_router
from app.tts import router as tts_router

config: AudioServiceConfig = load_config()


@asynccontextmanager
async def lifespan(app: FastAPI):
    print(f"Starting {config.service_name} v{config.service_version}")
    print(f"  Inference mode:  {config.inference_mode.value}")
    print(f"  STT model:       {config.stt_model}")
    print(f"  STT device:      {config.stt_device}")
    print(f"  TTS model:       {config.tts_model}")
    print(f"  TTS device:      {config.tts_device}")
    yield
    print(f"Shutting down {config.service_name}")


app = FastAPI(
    title=config.service_name,
    version=config.service_version,
    description="Unified STT + TTS audio inference service for VoxaLive",
    lifespan=lifespan,
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(stt_router, prefix="/stt", tags=["STT"])
app.include_router(tts_router, prefix="/tts", tags=["TTS"])


@app.get("/")
async def root():
    return {
        "service": config.service_name,
        "version": config.service_version,
        "status": "running",
    }


@app.get("/health")
async def health():
    return {
        "service": config.service_name,
        "version": config.service_version,
        "status": "healthy",
        "config": {
            "inference_mode": config.inference_mode.value,
            "stt_device": config.stt_device,
            "tts_device": config.tts_device,
            "stt_model": config.stt_model,
            "tts_model": config.tts_model,
        },
        "endpoints": {
            "stt": "/stt/transcribe",
            "tts": "/tts/synthesize",
        },
    }


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "app.main:app",
        host="127.0.0.1",
        port=config.port,
        reload=True,
    )
