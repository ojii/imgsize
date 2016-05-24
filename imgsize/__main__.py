from __future__ import absolute_import

import argparse
import io

import sys

from . import get_size, UnknownSize


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('path')
    args = parser.parse_args()
    with io.open(args.path, 'rb') as fobj:
        try:
            width, height = get_size(fobj)
        except UnknownSize:
            print("Could not determine image dimensions")
            sys.exit(1)
        else:
            print("%sx%s" % (width, height))


if __name__ == '__main__':
    main()
