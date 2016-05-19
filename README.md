# imgsize

Pure Python library to get the size of image files. Supports JPG, PNG, GIF and
BMP, though some files in those formats may not be supported.

## Usage

```python
import io

from imgsize import get_size

with io.open('/path/to/your/image', 'rb') as fobj:
    width, height = get_size(fobj)
```

## Extend

You can extend imgsize with new formats. In this example, we assume an image
format with the magic number `0x64 0x78 0x61 0x6d 0x70 0x6c 0x65`, followed by
an unsigned int for the width, followed by an unsigned int for the height. All
values are little-endian.

For that format, we would write this class:

```python
import struct

from imgsize.formats import SignatureFormat


class ExampleFormat(SignatureFormat):
    signature = struct.pack('<BBBBBBB', 0x64, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65)
    
    @classmethod
    def get_size(cls, fobj):
        signature = fobj.read(7)
        if signature != cls.signature:
            raise ValueError("Invalid signature %r" % signature)
        width, height = struct.unpack('<II', fobj.read(2))
        return width, height
```

Now to use it (together with the built-in formats), use this code:

```python
from imgsize.core import ImageSize
from imgsize.formats import jpg, gif, png, bmp

imgsize = ImageSize()
imgsize.register(jpg.JPGSize)
imgsize.register(gif.GIFSize)
imgsize.register(png.PNGSize)
imgsize.register(bmp.BMPSize)
imgsize.register(ExampleFormat)

with io.open('/path/to/image', 'rb') as fobj:
    width, height = imgsize.get_size(fobj)
```
