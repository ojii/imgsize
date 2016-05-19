from __future__ import absolute_import

import struct

from . import SignatureFormat
from ..core import UnknownSize


class JPGSize(SignatureFormat):
    signature = struct.pack('3B', 0xff, 0xd8, 0xff)

    soi = 0xd8
    sof0 = 0xc0
    sof1 = 0xc1

    @classmethod
    def get_size(cls, fobj):
        def readbyte():
            return ord(fobj.read(1))

        def readword():
            return (readbyte() << 8) + readbyte()

        def readmarker():
            if readbyte() != 255:
                raise ValueError("Invalid marker")
            return readbyte()

        def skipmarker():
            length = readword()
            fobj.read(length - 2)

        width, height = None, None

        while width is None or height is None:
            marker = readmarker()
            if marker == cls.soi:  # Start Of Image
                pass
            elif marker in (cls.sof0, cls.sof1):
                readword()  # length
                readbyte()  # precision
                height = readword()
                width = readword()
                return width, height
            else:
                skipmarker()
        raise UnknownSize()
