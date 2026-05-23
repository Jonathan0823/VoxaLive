import io
import math
import struct
import wave

SAMPLE_RATE = 16000
DURATION_SEC = 0.5


def generate_sine_wave_bytes(
    frequency: float = 440.0,
    sample_rate: int = SAMPLE_RATE,
    duration: float = DURATION_SEC,
) -> bytes:
    num_samples = int(sample_rate * duration)
    samples = []
    for i in range(num_samples):
        t = i / sample_rate
        val = int(math.sin(2 * math.pi * frequency * t) * 32767 * 0.5)
        samples.append(val)

    buf = io.BytesIO()
    with wave.open(buf, "wb") as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(sample_rate)
        wf.writeframes(struct.pack("<" + "h" * num_samples, *samples))
    return buf.getvalue()


SAMPLE_AUDIO_WAV = generate_sine_wave_bytes()
SAMPLE_AUDIO_PCM = SAMPLE_AUDIO_WAV[44:]
