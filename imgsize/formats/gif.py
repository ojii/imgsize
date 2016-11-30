from __future__ import absolute_import

from . import signature, Struct


Header = Struct('<HH')


@signature('GIF', b'GIF87a', b'GIF89a')
def get_size(fobj):
    return Header.unpack_from(fobj)
