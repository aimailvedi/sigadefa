import pytest
from sigadefa.frequency_model import FrequencyModel


def test_map_sequence_to_values():
    model = FrequencyModel()
    dna = "ATCG"
    values = model.map_sequence_to_values(dna)
    assert values == [6, 5, 4, 3]


def test_values_to_intervals():
    model = FrequencyModel()
    values = [6, 3, 3]
    intervals = model.values_to_intervals(values)
    assert intervals == [1.0, 0.5, 0.5]


def test_intervals_to_frequencies():
    model = FrequencyModel(base_frequency=440.0)
    intervals = [1.0, 0.5]
    freqs = model.intervals_to_frequencies(intervals)
    assert freqs == [440.0, 220.0]


def test_full_pipeline():
    model = FrequencyModel(base_frequency=440.0)
    dna = "ATCG"
    freqs = model.sequence_to_frequencies(dna)
    assert freqs == [
        440.0,
        440.0 * (5 / 6),
        440.0 * (4 / 6),
        440.0 * (3 / 6),
    ]
