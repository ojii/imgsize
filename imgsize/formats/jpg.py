from __future__ import absolute_import

import struct

from . import signature
from ..core import UnknownSize


SOI = 0xd8
SOF0 = 0xc0
SOF1 = 0xc1


class InvalidMarker(ValueError):
    pass


@signature('JPG', struct.pack('3B', 0xff, 0xd8, 0xff))
def get_size(fobj):
    fobj.seek(0)

    def readbyte():
        return ord(fobj.read(1))

    def readword():
        return (readbyte() << 8) + readbyte()

    def readmarker():
        if readbyte() != 255:
            raise InvalidMarker()
        return readbyte()

    def skipmarker():
        length = readword()
        fobj.read(length - 2)

    width, height = None, None

    while width is None or height is None:
        try:
            marker = readmarker()
        except InvalidMarker:
            raise UnknownSize()
        if marker == SOI:  # Start Of Image
            pass
        elif marker in (SOF0, SOF1):
            readword()  # length
            readbyte()  # precision
            height = readword()
            width = readword()
            return width, height
        else:
            skipmarker()
    raise UnknownSize()
