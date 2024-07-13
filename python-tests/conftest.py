from importlib import import_module
from pathlib import Path
from subprocess import check_call

import pytest
import sys

ROOT = Path(__file__).parent.parent


@pytest.fixture(scope='session')
def imgsize():
    assert sys.prefix != sys.base_prefix, "must be in virtualenv"
    check_call(['maturin', 'develop', '--manifest-path', ROOT / 'Cargo.toml'])
    return import_module('imgsize')
