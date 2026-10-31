#!/usr/bin/env python3
"""Generate assets/sounds/tick.wav, the embedded running-timer loop.

The file is a 2 second mono loop with a tick at 0.0 s and a lower tock at
1.0 s. It is fully synthesized, so it has no third-party license.
"""

import math
import random
import struct
import wave
from pathlib import Path

RATE = 22050
SECONDS = 2.0
OUT = Path(__file__).resolve().parent.parent / "assets" / "sounds" / "tick.wav"


def click(t, freq, decay, noise_gain):
    """Return one sample of a damped sine plus noise burst at time t."""
    if t < 0:
        return 0.0
    env = math.exp(-t * decay)
    tone = math.sin(2 * math.pi * freq * t)
    noise = random.uniform(-1.0, 1.0) * math.exp(-t * decay * 3)
    return env * (0.7 * tone + noise_gain * noise)


def main():
    random.seed(7)
    frames = int(RATE * SECONDS)
    samples = []
    for i in range(frames):
        t = i / RATE
        s = click(t, 2200.0, 60.0, 0.5) + click(t - 1.0, 1500.0, 70.0, 0.4)
        samples.append(max(-1.0, min(1.0, s * 0.8)))

    OUT.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(OUT), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(RATE)
        w.writeframes(b"".join(struct.pack("<h", int(s * 32767)) for s in samples))
    print(f"wrote {OUT} ({OUT.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
