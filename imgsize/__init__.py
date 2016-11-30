from __future__ import absolute_import

from .core import ImageSize, UnknownSize
from .formats import jpg, gif, png, bmp

__all__ = (
    'get_size',
    'UnknownSize',
)

DEFAULT = ImageSize()
DEFAULT.register(jpg.get_size)
DEFAULT.register(gif.get_size)
DEFAULT.register(png.get_size)
DEFAULT.register(bmp.get_size)

get_size = DEFAULT.get_size
