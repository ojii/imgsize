from __future__ import absolute_import

from ..core import UnknownSize


class Format(object):
    @classmethod
    def match(cls, fobj):
        return False

    @classmethod
    def get_size(cls, fobj):
        raise UnknownSize()


class SignatureFormat(Format):
    signature = None
    @classmethod
    def match(cls, fobj):
        length = len(cls.signature)
        data = fobj.peek(length)[:length]
        return data == cls.signature
