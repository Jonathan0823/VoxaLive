import base64
import io
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from app.config import load_config

config = load_config()

router = APIRouter()


class SynthesizeRequest(BaseModel):
    text: str
    voice_id: str = "default"
    format: str = "wav"


class SynthesizeResponse(BaseModel):
    audio_data: str
    sample_rate: int = 22050
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

        # TODO: Replace with actual TTS inference
        # from qwen_tts import QwenTTS
        # tts = QwenTTS(model=config.tts_model, voice=request.voice_id)
        # audio = tts.synthesize(request.text)
        # buffer = io.BytesIO()
        # wavfile.write(buffer, 22050, audio)
        # audio_bytes = buffer.getvalue()

        import numpy as np
        from scipy.io import wavfile

        sample_rate = 22050
        duration = 1.0
        t = np.linspace(0, duration, int(sample_rate * duration))
        audio = np.sin(2 * np.pi * 440.0 * t).astype(np.float32)

        buffer = io.BytesIO()
        wavfile.write(buffer, sample_rate, (audio * 32767).astype(np.int16))
        audio_bytes = buffer.getvalue()

        audio_data = base64.b64encode(audio_bytes).decode("utf-8")

        return SynthesizeResponse(
            audio_data=audio_data,
            sample_rate=sample_rate,
            channels=1,
            format="wav",
        )

    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Synthesis failed: {e}")
