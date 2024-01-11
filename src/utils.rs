use std::io;
use std::io::Cursor;

use crate::Size;

pub fn format_parser<'a, F:Fn(io::Cursor<&'a [u8]>) -> io::Result<Option<Size>>>(data: &'a [u8], signature: &'a [u8], parser: F) -> Option<Size> {
    if !data.starts_with(signature) {
        return None
    }
    parser(Cursor::new(data)).ok().flatten()
}