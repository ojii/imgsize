from __future__ import absolute_import

import io
import json
import os
import unittest

from . import get_size

TEST_DATA_DIR = os.path.join(
    os.path.abspath(os.path.dirname(__file__)),
    'testdata',
)


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
