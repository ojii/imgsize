from __future__ import absolute_import

import struct

import zlib

from . import signature, Struct
from ..core import UnknownSize


MAX_CHUNK_LENGTH = 2 ** 31 - 1
VERIFY_CONSTANT = 2 ** 32 - 1
CHUNK_IHDR = b'IHDR'  # Image HeaDeR
SIGNATURE = struct.pack('8B', 0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a)

ChunkLength = Struct('!I')
BytesChunkType = Struct('!4s')
Size = Struct('!II')


@signature('PNG', SIGNATURE)
def get_size(fobj):
    def sread(num):
        """
        reads num bytes and checks that the returned length matches
        """
        data = fobj.read(num)
        if len(data) != num:
            raise ValueError(
                'Expected to read {num}, got {got} instead'.format(
                    num=num,
                    got=len(data)
                )
            )
        return data

    width, height = None, None

    while width is None or height is None:
        chunk_length = ChunkLength.unpack_single_from(fobj)
        if chunk_length > MAX_CHUNK_LENGTH:
            raise ValueError("Chunk too long")
        bytes_chunk_type = BytesChunkType.unpack_single_from(fobj)
        chunk_type = bytes_chunk_type
        data = sread(chunk_length)
        checksum = sread(4)
        verify = zlib.crc32(data, zlib.crc32(bytes_chunk_type))
        verify &= VERIFY_CONSTANT
        verify = struct.pack('!I', verify)
        if checksum != verify:
            raise ValueError("Checksum error")
        if chunk_type == CHUNK_IHDR:
            if chunk_length != 13:
                raise ValueError("Invalid IHDR length")
            return Size.unpack(data[:Size.size])
    raise UnknownSize()
