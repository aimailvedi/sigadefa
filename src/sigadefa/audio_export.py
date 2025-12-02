import numpy as np
import wave

from sigadefa.frequency_model import FrequencyModel


RHYTHM_MAP = {
    "A": 0.40,
    "T": 0.30,
    "C": 0.25,
    "G": 0.20,
}


def normalize_audio(audio: np.ndarray) -> np.ndarray:
    max_val = np.max(np.abs(audio))
    if max_val == 0:
        return audio
    return audio / max_val


def frequencies_to_wav(
    dna_sequence: str,
    frequencies,
    filename="sigadefa_first_sound.wav",
    sample_rate=44100,
    amplitude=0.8,
    pause=0.05
):
    audio = []

    for nucleotide, freq in zip(dna_sequence, frequencies):
        duration = RHYTHM_MAP.get(nucleotide, 0.3)

        t = np.linspace(0, duration, int(sample_rate * duration), endpoint=False)
        wave_data = amplitude * np.sin(2 * np.pi * freq * t)

        pause_data = np.zeros(int(sample_rate * pause))

        audio.extend(wave_data)
        audio.extend(pause_data)

    audio = np.array(audio)
    audio = normalize_audio(audio)
    audio = np.int16(audio * 32767)

    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(audio.tobytes())


if __name__ == "__main__":
    model = FrequencyModel(base_frequency=440.0)
    dna = "ATCGATCG"

    frequencies = model.sequence_to_frequencies(dna)
    print("Frequencies:", frequencies)

    frequencies_to_wav(
        dna_sequence=dna,
        frequencies=frequencies,
        pause=0.05
    )

    print("Файл sigadefa_first_sound.wav создан с ритмом.")
