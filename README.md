# imgsize

Pure Python library to get the size of image files. Supports JPG, PNG, GIF and
BMP, though some files in those formats may not be supported.

## Why

For fun. Also, because if all you need is the size of an image, pulling in
Pillow is overkill. While Pillow is a fantastic library, it relies on quite a
few C libraries, making it a somewhat "heavy" dependency. If you need to work
with images, it's great, but if all you need is the size, maybe something a
bit more lightweight is more suited.

It is also quite a bit faster than Pillow (for getting the size), as it does
no actual decoding of image data and stops doing anything as soon as the
size information is found.

### Benchmarks

Note that benchmarks are a lie. But for what it's worth, here's the time it
took on my machine to read the image size of four different formats 10'000
times:

| format   |   imgsize |   pillow | speedup |
| -------- | --------- | -------- | --------- |
| jpg      |    0.3220 |   1.1276 | 3.50x |
| png      |    0.2856 |   0.8805 | 3.08x | 
| gif      |    0.2052 |   0.9481 | 4.62x |
| bmp      |    0.2435 |   0.6003 | 2.46x |


## Usage

```python
import io

from imgsize import get_size

with io.open('/path/to/your/image', 'rb') as fobj:
    width, height = get_size(fobj)
```

You can also use it from the command line using `python -m imgsize <path>`.

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
