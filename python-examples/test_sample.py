import pytest
from typing import List, Dict


@pytest.fixture()
def sample_string() -> str:
    return 'hello'


@pytest.fixture()
def sample_string_2(sample_string) -> str:
    return sample_string + ' world'


@pytest.fixture()
def sample_list() -> List[int]:
    return [1, 2, 3]


@pytest.fixture()
def sample_nested_dict() -> List[List[Dict[int, str]]]:
    return [[{1: '1'}, {2: '2'}], [{1: '1'}, {1: '1'}]]


@pytest.fixture()
def sample_nested_list() -> List[List[int]]:
    return [[1, 1], [2, 1]]


@pytest.fixture()
def sample_missing_return_type():
    return 1


def test_hello(sample_string_2, sample_string: int):
    assert sample_string_2 == 'hello world'


def test_hello_2(sample_string: str):
    pass


def test_hello_3(sample_list: List[str]):
    pass


def test_hello_4(sample_missing_return_type: str):
    pass


def test_hello_5(sample_nested_dict: Dict):
    pass


def test_hello_6(sample_nested_list: List[List]):
    pass
