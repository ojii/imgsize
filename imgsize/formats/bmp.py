from __future__ import absolute_import

import struct

from imgsize.core import WrongFormat
from . import signature, Struct

HeaderSize = Struct('<I')
HeaderData5 = Struct('<II')
HeaderDataCore = Struct('<HH')
YFlip = Struct('<B')
MODIFIER = 2**32


@signature('BMP', struct.pack('<2B', 0x42, 0x4D))
def get_size(fobj):
    fobj.seek(14)  # skip 14 bytes header
    header_size = HeaderSize.unpack_single_from(fobj)
    if header_size == 12:
        return HeaderDataCore.unpack_from(fobj)
    elif header_size in (40, 64, 108, 124):
        data = HeaderData5.safe_read(fobj)
        width, height = HeaderData5.unpack(data)
        if YFlip.unpack_single(data[7:8]) == 0xff:
            height = MODIFIER - height
        return width, height
    else:
        raise WrongFormat(
            'Unknown or unsupported BMP header size {size}'.format(
                size=header_size
            )
        )
