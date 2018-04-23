from __future__ import absolute_import

import io
import json
import os
import struct
import unittest

from imgsize.formats import jpg
from . import get_size, UnknownSize

TEST_DATA_DIR = os.path.join(
    os.path.abspath(os.path.dirname(__file__)),
    'testdata',
)


class UnitTests(unittest.TestCase):
    def test_jpg_get_size_invalid_marker(self):
        bio = io.BufferedReader(io.BytesIO(struct.pack('5B', 0xff, 0xd8, 0xff, 0xd8, 0xab)))
        with self.assertRaises(UnknownSize):
            jpg.get_size(bio)


class Base(unittest.TestCase):
    def assertImage(self, path):
        with open(path + '.json') as fobj:
            info = json.load(fobj)
        with io.open(path, 'rb') as fobj:
            width, height = get_size(fobj)
            got = {
                'width': width,
                'height': height
            }
            self.assertDictEqual(info, got)


def method_factory(path):
    def test_method(self):
        self.assertImage(path)
    test_method.__name__ = 'test_' + path
    test_method.__doc__ = path
    return test_method


def build():
    methods = {}
    for name in os.listdir(TEST_DATA_DIR):
        path = os.path.join(TEST_DATA_DIR, name)
        if os.path.exists(path + '.json'):
            methods['test_' + path] = method_factory(path)
    return type('Tests', (Base,), methods)


Tests = build()
