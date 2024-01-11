use std::io::{Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::Size;
use crate::utils::format_parser;

const MIME_TYPE: &'static str = "image/bmp";
const SIGNATURE: &'static [u8] = &[0x42, 0x4D];

pub fn get_size(data: &[u8]) -> Option<Size>{
    format_parser(data, SIGNATURE, |mut cursor| {
        cursor.seek(SeekFrom::Start(14))?;
        let header_size = cursor.read_u32::<LittleEndian>()?;
        match header_size {
            12 => {
                let width = cursor.read_u16::<LittleEndian>()?;
                let height = cursor.read_u16::<LittleEndian>()?;
                Ok(Some(Size::new(width as u64, height as u64, MIME_TYPE.to_string())))
            },
            40 | 64 |108 |124 => {
                let width = cursor.read_u32::<LittleEndian>()? as u64;
                let mut height = cursor.read_u32::<LittleEndian>()? as u64;
                cursor.seek(SeekFrom::Current(-1))?;
                if cursor.read_u8()? == 0xff {
                    height = 4294967296 - height;
                }
                Ok(Some(Size::new(width, height, MIME_TYPE.to_string())))
            },
            _ => Ok(None)
        }
    })
}