import base64
import os
from urllib.parse import urljoin

import requests

SERVICE_URL = os.getenv("AUDIO_SERVICE_URL", "http://127.0.0.1:8002")


def stt_transcribe(
    audio_bytes: bytes,
    audio_format: str = "pcm16",
    sample_rate: int = 16000,
    channels: int = 1,
    language: str | None = "auto",
) -> dict:
    audio_data = base64.b64encode(audio_bytes).decode("utf-8")
    payload = {
        "audio_format": audio_format,
        "sample_rate": sample_rate,
        "channels": channels,
        "language": language,
        "audio_data": audio_data,
    }
    url = urljoin(SERVICE_URL, "/stt/transcribe")
    resp = requests.post(url, json=payload, timeout=30)
    resp.raise_for_status()
    return resp.json()


def tts_synthesize(
    text: str,
    voice_id: str = "default",
    format: str = "wav",
) -> dict:
    payload = {
        "text": text,
        "voice_id": voice_id,
        "format": format,
    }
    url = urljoin(SERVICE_URL, "/tts/synthesize")
    resp = requests.post(url, json=payload, timeout=30)
    resp.raise_for_status()
    return resp.json()


def health() -> dict:
    url = urljoin(SERVICE_URL, "/health")
    resp = requests.get(url, timeout=10)
    resp.raise_for_status()
    return resp.json()
