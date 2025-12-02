import numpy as np
import wave
from frequency_model import FrequencyModel


def frequencies_to_wav(frequencies, filename=\"sigadefa_first_sound.wav\", duration=0.5, sample_rate=44100):
    \"\"\"
    Создаёт WAV-файл из списка частот.
    Каждая частота звучит duration секунд.
    \"\"\"
    audio = []

    for freq in frequencies:
        t = np.linspace(0, duration, int(sample_rate * duration), endpoint=False)
        wave_data = 0.5 * np.sin(2 * np.pi * freq * t)
        audio.extend(wave_data)

    audio = np.array(audio)
    audio = np.int16(audio / np.max(np.abs(audio)) * 32767)

    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(audio.tobytes())


if __name__ == \"__main__\":
    model = FrequencyModel(base_frequency=440.0)
    dna = \"ATCGATCG\"

    frequencies = model.sequence_to_frequencies(dna)
    print(\"Frequencies:\", frequencies)

    frequencies_to_wav(frequencies)
    print(\"Файл sigadefa_first_sound.wav создан.\")
