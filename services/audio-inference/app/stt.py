import base64
import os
import tempfile
from pathlib import Path

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from app.config import load_config, InferenceMode

router = APIRouter()

_whisper_model = None


def _get_whisper_model():
    global _whisper_model
    if _whisper_model is not None:
        return _whisper_model

    config = load_config()
    model_name = config.stt_model
    device = config.stt_device
    compute_type = "float16" if device == "cuda" else "int8"

    if not _is_valid_model_name(model_name) and not Path(model_name).exists():
        raise RuntimeError(
            f"STT model '{model_name}' not found. "
            "Specify a valid model name ('tiny', 'base', 'small', 'medium', 'large-v3') "
            "or a path to a converted model."
        )

    from faster_whisper import WhisperModel

    _whisper_model = WhisperModel(
        model_name,
        device=device,
        compute_type=compute_type,
        download_root=os.getenv("WHISPER_DOWNLOAD_ROOT", None),
    )
    print(f"Whisper model '{model_name}' loaded on {device} (compute={compute_type})")
    return _whisper_model


def _is_valid_model_name(name: str) -> bool:
    valid = {"tiny", "base", "small", "medium", "large", "large-v2", "large-v3", "distil-small.en", "distil-medium.en", "distil-large-v2", "distil-large-v3"}
    return name.lower() in valid


class TranscribeRequest(BaseModel):
    audio_format: str = "pcm16"
    sample_rate: int = 16000
    channels: int = 1
    language: str | None = "auto"
    audio_data: str


class TranscribeResponse(BaseModel):
    transcript: str


@router.post("/transcribe", response_model=TranscribeResponse)
async def transcribe(request: TranscribeRequest):
    try:
        audio_bytes = base64.b64decode(request.audio_data)

        model = _get_whisper_model()

        lang = request.language if request.language and request.language != "auto" else None

        if request.audio_format == "wav":
            import wave
            import io
            with wave.open(io.BytesIO(audio_bytes), "rb") as wf:
                sr = wf.getframerate()
                raw = wf.readframes(wf.getnframes())
        elif request.audio_format == "pcm16":
            sr = request.sample_rate
            raw = audio_bytes
        else:
            sr = request.sample_rate
            raw = audio_bytes

        import numpy as np
        audio_array = np.frombuffer(raw, dtype=np.int16).astype(np.float32) / 32768.0

        segments, info = model.transcribe(audio_array, language=lang)
        transcript = " ".join(segment.text for segment in segments)

        detected_lang = info.language if lang is None else lang
        print(f"STT: transcribed {len(audio_bytes)} bytes -> {len(transcript)} chars "
              f"(lang={detected_lang}, prob={info.language_probability:.2f})")

        return TranscribeResponse(transcript=transcript)

    except base64.binascii.Error as e:
        raise HTTPException(status_code=400, detail=f"Invalid base64 audio data: {e}")
    except RuntimeError as e:
        raise HTTPException(status_code=500, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Transcription failed: {e}")
