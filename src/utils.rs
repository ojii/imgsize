use std::io;
use std::io::Cursor;

use crate::Size;

pub fn cursor_parser<F: Fn(io::Cursor<&[u8]>) -> io::Result<Option<Size>>>(
    data: &[u8],
    parser: F,
) -> Option<Size> {
    parser(Cursor::new(data)).ok().flatten()
}
