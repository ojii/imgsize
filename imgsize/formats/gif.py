from __future__ import absolute_import

import struct

from . import SignatureFormat


class GIFSize(SignatureFormat):
    signature = struct.pack('4B', 0x47, 0x49, 0x46, 0x38)

    @classmethod
    def get_size(cls, fobj):
        signature = fobj.read(6)
        if signature not in (b'GIF87a', b'GIF89a'):
            raise ValueError("Invalid GIF signature %r" % signature)
        return struct.unpack('<HH', fobj.read(4))
