/*
from __future__ import absolute_import

from . import signature, Struct


Header = Struct('<HH')


@signature('GIF', b'GIF87a', b'GIF89a')
def get_size(fobj):
    return Header.unpack_from(fobj)
 */

use std::io;
use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::Size;
use crate::utils::format_parser;

const GIF87_SIG: &'static [u8] =&[0x47, 0x49, 0x46, 0x38, 0x37, 0x61];
const GIF89_SIG: &'static [u8] =&[0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
const MIME_TYPE: &'static str = "image/gif";

pub fn get_size(data: &[u8]) -> Option<Size> {
    format_parser(data, GIF87_SIG, parser).or_else(|| format_parser(data, GIF89_SIG, parser))
}

fn parser(mut cursor: Cursor<&[u8]>) -> io::Result<Option<Size>> {
    cursor.seek(SeekFrom::Start(6))?;
    let width = cursor.read_u16::<LittleEndian>()?;
    let height = cursor.read_u16::<LittleEndian>()?;
    Ok(Some(Size::new(width as u64, height as u64, MIME_TYPE.to_string())))
}