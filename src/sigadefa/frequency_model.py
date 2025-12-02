class FrequencyModel:
    \"\"\"
    FrequencyModel — базовая модель перевода ДНК-последовательностей
    в числовые значения и частоты.

    A, T, C, G → значения → частоты (Гц)
    \"\"\"

    NUCLEOTIDE_MAP = {
        'A': 6,
        'T': 5,
        'C': 4,
        'G': 3,
    }

    def __init__(self, base_frequency: float = 440.0):
        self.base_frequency = base_frequency

    def map_sequence_to_values(self, sequence: str) -> list:
        \"\"\"
        Преобразует строку ДНК (ATCG...) в список числовых значений.
        \"\"\"
        values = []

        for nucleotide in sequence.upper():
            if nucleotide not in self.NUCLEOTIDE_MAP:
                raise ValueError(f\"Недопустимый нуклеотид: {nucleotide}\")
            values.append(self.NUCLEOTIDE_MAP[nucleotide])

        return values

    def values_to_frequencies(self, values: list) -> list:
        \"\"\"
        Преобразует список числовых значений в частоты на основе base_frequency.
        \"\"\"
        frequencies = []

        for value in values:
            freq = self.base_frequency * value
            frequencies.append(freq)

        return frequencies


# === БАЗОВЫЙ ТЕСТ (можно запускать напрямую) ===

if __name__ == \"__main__\":
    model = FrequencyModel()
    dna = \"ATCG\"
    values = model.map_sequence_to_values(dna)
    freqs = model.values_to_frequencies(values)

    print(\"DNA:\", dna)
    print(\"Values:\", values)
    print(\"Frequencies:\", freqs)
