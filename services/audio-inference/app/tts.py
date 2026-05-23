import base64
import io
import wave
from pathlib import Path

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from app.config import load_config

config = load_config()

router = APIRouter()

_piper_voice = None


def _get_piper_voice():
    global _piper_voice
    if _piper_voice is not None:
        return _piper_voice

    model_path = config.tts_model_path
    if not Path(model_path).exists():
        raise RuntimeError(
            f"Piper model not found at '{model_path}'. "
            "Set TTS_MODEL_PATH or place a .onnx file in ./voices/"
        )

    from piper import PiperVoice

    use_cuda = config.tts_device == "cuda"
    _piper_voice = PiperVoice.load(model_path, use_cuda=use_cuda)
    print(f"Piper voice loaded from '{model_path}' "
          f"(device={config.tts_device}, sr={_piper_voice.config.sample_rate})")
    return _piper_voice


class SynthesizeRequest(BaseModel):
    text: str
    voice_id: str = "default"
    format: str = "wav"


class SynthesizeResponse(BaseModel):
    audio_data: str
    sample_rate: int
    channels: int = 1
    format: str = "wav"


@router.post("/synthesize", response_model=SynthesizeResponse)
async def synthesize(request: SynthesizeRequest):
    try:
        if not request.text or not request.text.strip():
            raise HTTPException(status_code=400, detail="Text cannot be empty")

        print(f"TTS: {len(request.text)} chars, voice={request.voice_id}, "
              f"format={request.format}, model={config.tts_model}, "
              f"device={config.tts_device}")

        voice = _get_piper_voice()
        sample_rate = voice.config.sample_rate

        buffer = io.BytesIO()
        with wave.open(buffer, "wb") as wf:
            voice.synthesize_wav(request.text, wf)

        audio_bytes = buffer.getvalue()
        audio_data = base64.b64encode(audio_bytes).decode("utf-8")

        print(f"TTS: synthesized {len(audio_bytes)} bytes "
              f"({len(request.text)} chars)")

        return SynthesizeResponse(
            audio_data=audio_data,
            sample_rate=sample_rate,
            channels=1,
            format="wav",
        )

    except HTTPException:
        raise
    except RuntimeError as e:
        raise HTTPException(status_code=500, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Synthesis failed: {e}")
