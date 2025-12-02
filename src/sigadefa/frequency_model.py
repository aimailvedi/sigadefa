class FrequencyModel:
    \"\"\"
    FrequencyModel — базовая модель перевода ДНК-последовательностей
    в числовые значения, интервалы и частоты.

    A, T, C, G → значения → относительные интервалы → частоты (Гц)
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

    def values_to_intervals(self, values: list) -> list:
        \"\"\"
        Преобразует список значений в относительные интервалы
        по отношению к первому элементу.
        \"\"\"
        if not values:
            return []

        base = values[0]
        intervals = []

        for value in values:
            interval = value / base
            intervals.append(interval)

        return intervals

    def intervals_to_frequencies(self, intervals: list) -> list:
        \"\"\"
        Преобразует интервалы в частоты на основе base_frequency.
        \"\"\"
        frequencies = []

        for interval in intervals:
            freq = self.base_frequency * interval
            frequencies.append(freq)

        return frequencies

    def sequence_to_frequencies(self, sequence: str) -> list:
        \"\"\"
        Полный конвейер:
        DNA → значения → интервалы → частоты
        \"\"\"
        values = self.map_sequence_to_values(sequence)
        intervals = self.values_to_intervals(values)
        frequencies = self.intervals_to_frequencies(intervals)
        return frequencies


# === БАЗОВЫЙ ТЕСТ ===

if __name__ == \"__main__\":
    model = FrequencyModel(base_frequency=440.0)
    dna = \"ATCG\"

    values = model.map_sequence_to_values(dna)
    intervals = model.values_to_intervals(values)
    freqs = model.intervals_to_frequencies(intervals)

    print(\"DNA:\", dna)
    print(\"Values:\", values)
    print(\"Intervals:\", intervals)
    print(\"Frequencies:\", freqs)
