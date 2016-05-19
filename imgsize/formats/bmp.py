from __future__ import absolute_import

import struct

from . import SignatureFormat


class BMPSize(SignatureFormat):
    signature = struct.pack('<2B', 0x42, 0x4D)

    @classmethod
    def get_size(cls, fobj):
        signature = fobj.read(2)
        if signature != cls.signature:
            raise ValueError("Invalid BMP signature %r" % signature)
        fobj.read(4)  # length
        fobj.read(4)  # reserved
        fobj.read(4)  # offset
        fobj.read(4)  # header size
        (width, height, nplanes, bits_per_pixel, compression_method, bmp_bytesz,
         hres, vres, ncolors, nimpcolors
         ) = struct.unpack_from('<IihhiiIIii', fobj.read(36))
        if nplanes != 1:
            raise ValueError("Unexpected nplanes %s" % nplanes)
        return width, height
