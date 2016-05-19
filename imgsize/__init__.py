from __future__ import absolute_import

from .core import ImageSize
from .formats import jpg, gif, png, bmp

__all__ = (
    'get_size',
)

DEFAULT = ImageSize()
DEFAULT.register(jpg.JPGSize)
DEFAULT.register(gif.GIFSize)
DEFAULT.register(png.PNGSize)
DEFAULT.register(bmp.BMPSize)

get_size = DEFAULT.get_size
