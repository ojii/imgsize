from __future__ import absolute_import

import struct

import zlib

from . import SignatureFormat
from ..core import UnknownSize


class PNGSize(SignatureFormat):
    signature = struct.pack(
        '8B', 0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a
    )

    max_chunk_length = 2 ** 31 - 1
    verify_constant = 2 ** 32 - 1
    chunk_ihdr = b'IHDR'

    @classmethod
    def get_size(cls, fobj):
        def sread(num):
            """
            reads num bytes and checks that the returned length matches
            """
            data = fobj.read(num)
            if len(data) != num:
                raise ValueError(
                    "Expected to read %s, got %s instead" % (num, len(data))
                )
            return data

        signature = fobj.read(8)
        if signature != cls.signature:
            raise ValueError("Invalid signature %r" % signature)

        width, height = None, None

        while width is None or height is None:
            raw_chunk_length = sread(4)
            chunk_length = struct.unpack('!I', raw_chunk_length)[0]
            if chunk_length > cls.max_chunk_length:
                raise ValueError("Chunk too long")
            raw_chunk_type = sread(4)
            bytes_chunk_type = struct.unpack('!4s', raw_chunk_type)[0]
            chunk_type = bytes_chunk_type
            data = sread(chunk_length)
            checksum = sread(4)
            verify = zlib.crc32(data, zlib.crc32(bytes_chunk_type))
            verify &= cls.verify_constant
            verify = struct.pack('!I', verify)
            if checksum != verify:
                raise ValueError("Checksum error")
            if chunk_type == cls.chunk_ihdr:  # IHDR = Image HeaDeR
                if chunk_length != 13:
                    raise ValueError("Invalid IHDR length")
                (width, height, bit_depth, color_type, compression_method,
                 filter_method, interlace_method) = struct.unpack('!2I5B', data)
                return width, height
        raise UnknownSize()
