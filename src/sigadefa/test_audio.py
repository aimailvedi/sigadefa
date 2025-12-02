from audio_export import frequencies_to_wav
from frequency_model import FrequencyModel

model = FrequencyModel(base_frequency=440.0)
dna = "ATCGATCG"

frequencies = model.sequence_to_frequencies(dna)
frequencies_to_wav(frequencies, filename="sigadefa_test_sound.wav")

print("Тестовый WAV-файл sigadefa_test_sound.wav создан.")
