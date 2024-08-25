# imgsize

Given some data, determines whether the data is likely an image and if so, what size and type it is and whether it is
animated or not.

## Formats

Supported formats:

* PNG/APNG
* JPEG
* GIF
* AVIF/AVIS
* BMP

## Usage

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
    size.is_animated
```

You should not pass the entire image data, the first kilobyte or so should suffice for most formats, other than GIF
where a larger amount of data may be required to determine whether the image is animated or not.

### API

#### `imgsize.get_size(data: bytes) -> imgsize.Size | None`

Given the data in the bytes provided, attempts to determine the image format, size and whether it
is an animated image or not, otherwise returns None.

#### `imgsize.Size`

A class with four properties: `width: int`, `height: int`, `mime_type: str`, `is_animated: bool`.

Instances of `imgsize.Size` are equatable, hashable and iterable (yielding `width` and `height`).

Instances of `imgsize.Size` have a `as_dict()` method which returns the properties as a dictionary.

## Notes

`imgsize` does not validate whether the data passed is a valid image or not. The intended use of
this library is to reject data early and quickly if it does not appear to be an image format you
intend to support. If you need to validate the entire image, the suggested workflow is to use this
library to reject data that is not images, is not a file format you support, has dimensions beyond
what you wish to support or is animated if you only want static images, then pass it to a library
that does actual image parsing to determine if the data is actually an image.

`imgsize` only supports a few formats, the supported formats is mostly based on what browsers support,
and does not necessarily support all features or variants of those formats, as a result, there might be
false positives and false negatives.

## Example CLI

You can use `cargo example --cli` for a simple command line tool to try out this library. See
`cargo example --cli -- --help` for details.

## Building

Use [maturin](https://www.maturin.rs/) to build: `maturin build`

To build & install into your local env: `maturin develop`

## Testing

### Rust

`cargo test`

### Python

The following must be run in a virtual env:

```
pip install '.[test]'
pytest python-tests
```

## Release

1. Change the version number in `Cargo.toml`
2. Push to the main branch on GitHub (preferably via Pull Request)
3. Create a Release (git tag) on GitHub
4. Release will automatically be pushed to PyPI