import os
from enum import Enum
from pydantic import BaseModel, field_validator


class InferenceMode(str, Enum):
    cpu = "cpu"
    gpu = "gpu"
    auto = "auto"


class AudioServiceConfig(BaseModel):
    service_name: str = "audio-inference"
    service_version: str = "0.1.0"
    port: int = int(os.getenv("PORT", "8002"))

    inference_mode: InferenceMode = InferenceMode(
        os.getenv("INFERENCE_MODE", "auto")
    )

    stt_model: str = os.getenv("STT_MODEL", "base")
    stt_device_override: str | None = os.getenv("STT_DEVICE_OVERRIDE")

    tts_model: str = os.getenv("TTS_MODEL", "qwen3-tts")
    tts_voice: str = os.getenv("TTS_VOICE", "default")

    @field_validator("inference_mode", mode="before")
    @classmethod
    def validate_mode(cls, v: str) -> InferenceMode:
        try:
            return InferenceMode(v.lower())
        except ValueError:
            valid = ", ".join(m.value for m in InferenceMode)
            raise ValueError(
                f"Invalid inference_mode '{v}'. Must be one of: {valid}"
            )

    def resolve_device(self) -> str:
        if self.inference_mode == InferenceMode.cpu:
            return "cpu"
        if self.inference_mode == InferenceMode.gpu:
            return "cuda"
        return "auto"

    @property
    def stt_device(self) -> str:
        if self.stt_device_override:
            return self.stt_device_override
        return self.resolve_device()

    @property
    def tts_device(self) -> str:
        return self.resolve_device()


_config: AudioServiceConfig | None = None


def load_config() -> AudioServiceConfig:
    global _config
    if _config is None:
        _config = AudioServiceConfig()
    return _config
