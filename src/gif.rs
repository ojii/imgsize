/*
from __future__ import absolute_import

from . import signature, Struct


Header = Struct('<HH')


@signature('GIF', b'GIF87a', b'GIF89a')
def get_size(fobj):
    return Header.unpack_from(fobj)
 */

use crate::utils::format_parser;
use crate::Size;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom};

const GIF87_SIG: &[u8] = &[0x47, 0x49, 0x46, 0x38, 0x37, 0x61];
const GIF89_SIG: &[u8] = &[0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
const MIME_TYPE: &str = "image/gif";
const ANIMATION_EXTENSION: [u8; 11] = [
    0x4e, 0x45, 0x54, 0x53, 0x43, 0x41, 0x50, 0x45, 0x32, 0x2e, 0x30,
];

pub fn get_size(data: &[u8]) -> Option<Size> {
    format_parser(data, GIF87_SIG, parser).or_else(|| format_parser(data, GIF89_SIG, parser))
}

fn parser(mut cursor: Cursor<&[u8]>) -> io::Result<Option<Size>> {
    cursor.seek(SeekFrom::Start(6))?;
    let width = cursor.read_u16::<LittleEndian>()?;
    let height = cursor.read_u16::<LittleEndian>()?;
    cursor.seek(SeekFrom::Start(0x30d))?;
    let animated = if cursor.read_u8()? == 0x21 {
        cursor.seek(SeekFrom::Start(0x310))?;
        let mut buf = [0u8; 11];
        cursor.read_exact(&mut buf)?;
        buf == ANIMATION_EXTENSION
    } else {
        false
    };
    Ok(Some(Size::new(
        width as u64,
        height as u64,
        MIME_TYPE.to_string(),
        animated,
    )))
}
