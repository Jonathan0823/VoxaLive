import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from tests.support.client import stt_transcribe, health
from tests.support.fixtures import SAMPLE_AUDIO_WAV


def test_health():
    result = health()
    assert result["status"] == "healthy"
    assert "config" in result
    assert result["config"]["stt_device"] is not None
    print(f"Service healthy: mode={result['config']['inference_mode']}")


def test_stt_transcribe_returns_transcript():
    result = stt_transcribe(
        audio_bytes=SAMPLE_AUDIO_WAV,
        audio_format="wav",
        sample_rate=16000,
        channels=1,
        language="auto",
    )
    assert "transcript" in result
    assert isinstance(result["transcript"], str)
    print(f"STT transcript ({len(result['transcript'])} chars): '{result['transcript']}'")


def test_stt_transcribe_with_language():
    result = stt_transcribe(
        audio_bytes=SAMPLE_AUDIO_WAV,
        language="en",
    )
    assert "transcript" in result


if __name__ == "__main__":
    test_health()
    test_stt_transcribe_returns_transcript()
    test_stt_transcribe_with_language()
    print("All STT smoke tests passed.")
