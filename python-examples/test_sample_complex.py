from typing import List, Dict

import pytest


@pytest.fixture()
def sample_list() -> List[int]:
    return [1, 2, 3]


@pytest.fixture()
def sample_nested_dict() -> List[List[Dict[int, str]]]:
    return [[{1: '1'}, {2: '2'}], [{1: '1'}, {1: '1'}]]


@pytest.fixture()
def sample_nested_list() -> List[List[int]]:
    return [[1, 1], [2, 1]]


def test_hello_3(sample_list: List[int]):
    pass


def test_hello_5(sample_nested_dict: Dict):
    pass


def test_hello_6(sample_nested_list: List[List]):
    pass
