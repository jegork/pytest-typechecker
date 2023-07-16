import pytest


@pytest.fixture()
def sample_string() -> str:
    return 'hello'


@pytest.fixture()
def sample_string_2(sample_string) -> str:
    return sample_string + ' world'


def test_hello(sample_string_2):
    assert sample_string_2 == 'hello world'
