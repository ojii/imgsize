from __future__ import absolute_import

import struct
from functools import wraps

from ..core import WrongFormat


class Struct(object):
    def __init__(self, fmt, single=False):
        self.fmt = fmt
        self.size = struct.calcsize(fmt)
        if single:
            self.unpack_from = self.unpack_from_single
        else:
            self.unpack_from = self.unpack_from_many

    def unpack(self, data):
        return struct.unpack(self.fmt, data)

    def safe_read(self, fobj):
        data = fobj.read(self.size)
        if len(data) != self.size:
            raise ValueError(
                'Tried to read {expected} bytes, only got {got}'.format(
                    expected=self.size,
                    got=len(data)
                )
            )
        return data

    def unpack_from_many(self, fobj):
        data = self.safe_read(fobj)
        return self.unpack(data)

    def unpack_from_single(self, fobj):
        return self.unpack_from_many(fobj)[0]


def signature(name, *candidates):
    length = max(map(len, candidates))

    def decorator(func):
        @wraps(func)
        def wrapper(fobj):
            got = fobj.peek(length)[:length]
            if got not in candidates:
                raise WrongFormat("Invalid {name} signature {got!r}".format(
                    name=name,
                    got=got,
                ))
            fobj.seek(length)
            return func(fobj)
        return wrapper

    return decorator
