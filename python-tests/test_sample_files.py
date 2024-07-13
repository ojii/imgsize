import json

import pytest
from conftest import ROOT

BYTES_TO_READ = 1024


def find_examples():
    test_data_dir = ROOT / 'src' / 'test-data'
    for input_path in test_data_dir.glob('*.input'):
        output_path = input_path.with_suffix('.output')
        if not output_path.exists(): continue
        with input_path.open('rb') as fobj:
            data = fobj.read(BYTES_TO_READ)
        with output_path.open('r') as fobj:
            output = json.load(fobj)
        yield pytest.param(data, output, id=input_path.stem)


@pytest.mark.parametrize('input,output', find_examples())
def test_sample_files(input, output, imgsize):
    assert imgsize.get_size(input).as_dict() == output
