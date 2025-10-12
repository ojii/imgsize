import json

import pytest
from conftest import ROOT


def find_examples():
    test_data_dir = ROOT / "src" / "test-data"
    for input_path in test_data_dir.glob("*.input"):
        output_path = input_path.with_suffix(".output")
        if not output_path.exists():
            continue
        with input_path.open("rb") as fobj:
            data = fobj.read()
        with output_path.open("r") as fobj:
            output = json.load(fobj)
        yield pytest.param(
            data, fix_output(remove_comments(output)), id=input_path.stem
        )


def remove_comments(data):
    data.pop("comment", None)
    return data


def fix_output(data):
    return lambda imgsize: data | {"animation": animation(imgsize, data["animation"])}


def animation(imgsize, v):
    match v:
        case True | "yes":
            return imgsize.Animation.Yes
        case False | "no":
            return imgsize.Animation.No
        case "unknown":
            return imgsize.Animation.Unknown
        case _ as value:
            assert False, "unexpected value: {value!r}".format(value=value)


@pytest.mark.parametrize("input,output", find_examples())
def test_sample_files(input, output, imgsize):
    assert imgsize.get_size(input).as_dict() == output(imgsize)
