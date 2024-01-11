# imgsize

Library to quickly get the type & dimensions of JPEG, PNG, BMP and GIF files.

## Usage (Python)

```python
from imgsize import get_size

some_image_data: bytes = ...

size = get_size(some_image_data)
if size is None:
    print("Could not handle data")
else:
    size.width
    size.height
    size.mime_type
```

You don't need to pass the entire image data, the first kilobyte or so should suffice.

## Building

Use [maturin](https://www.maturin.rs/) to build: `maturin build`

To build & install into your local env: `maturin develop`

## Testing

`cargo test`

