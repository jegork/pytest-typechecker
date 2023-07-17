import pytest


@pytest.fixture()
def sample_string() -> str:
    return 'hello'


@pytest.fixture()
def sample_string_2(sample_string) -> str:
    return sample_string + ' world'


@pytest.fixture()
def sample_missing_return_type():
    return 1


def test_hello(sample_string_2, sample_string: int):
    assert sample_string_2 == 'hello world'


def test_hello_2(sample_string: str):
    pass


def test_hello_4(sample_missing_return_type: str):
    pass
