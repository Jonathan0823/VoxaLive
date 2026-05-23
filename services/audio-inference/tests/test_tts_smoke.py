import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from tests.support.client import tts_synthesize, health


def test_health():
    result = health()
    assert result["status"] == "healthy"
    assert "config" in result
    assert result["config"]["tts_device"] is not None


def test_tts_synthesize_returns_audio():
    result = tts_synthesize(text="Hello world")
    assert "audio_data" in result
    assert isinstance(result["audio_data"], str)
    assert len(result["audio_data"]) > 0
    assert result["format"] == "wav"
    assert result["sample_rate"] > 0
    print(f"TTS audio: {len(result['audio_data'])} base64 chars")


def test_tts_synthesize_empty_text_fails():
    import requests
    import os
    from urllib.parse import urljoin

    SERVICE_URL = os.getenv("AUDIO_SERVICE_URL", "http://127.0.0.1:8002")
    payload = {"text": "", "voice_id": "default", "format": "wav"}
    url = urljoin(SERVICE_URL, "/tts/synthesize")
    resp = requests.post(url, json=payload, timeout=10)
    assert resp.status_code == 400
    data = resp.json()
    assert "detail" in data
    print(f"Empty text correctly rejected: {data['detail']}")


if __name__ == "__main__":
    test_health()
    test_tts_synthesize_returns_audio()
    test_tts_synthesize_empty_text_fails()
    print("All TTS smoke tests passed.")
