# src/sigadefa/frequency_model.py
# Ядро преобразования генетического кода в частотные паттерны (Мантру).

class FrequencyModel:
    """
    Класс для преобразования последовательностей ДНК в числовые частотные массивы.
    """

    # Определяем пифагорейские соотношения (пока в виде простых чисел)
    # A=Ля (6), T=Соль (5), C=Фа (4), G=Ми (3) - пример:
    NUCLEOTIDE_MAP = {
        'A': 6,
        'T': 5,
        'C': 4,
        'G': 3,
    }

    def __init__(self, base_frequency=440.0):
        """Инициализация с базовой частотой (например, Ля 440 Гц)."""
        self.base_frequency = base_frequency

    def map_sequence_to_values(self, sequence: str) -> list:
        """Преобразует последовательность ДНК в список числовых значений согласно NUCLEOTIDE_MAP."""
        sequence = sequence.upper().replace(' ', '')
        return [self.NUCLEOTIDE_MAP.get(n, 0) for n in sequence]

    def values_to_frequencies(self, values: list) -> list:
        """Преобразует числовые значения в частоты (пока заглушка)."""
        # Здесь будет сложная логика, использующая self.base_frequency и интервалы.
        return [v * self.base_frequency / 100 for v in values]


# Пример использования (для тестирования):
if __name__ == '__main__':
    model = FrequencyModel()
    dna_sample = "AGTCAG"
    
    values = model.map_sequence_to_values(dna_sample)
    print(f"Values: {values}")
    
    frequencies = model.values_to_frequencies(values)
    print(f"Frequencies: {frequencies}")
