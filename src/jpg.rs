use std::io::{Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::utils::cursor_parser;
use crate::{Animation, Size};

const MIME_TYPE: &str = "image/jpeg";
const START_OF_FRAMES: [u8; 13] = [
    0xc0, 0xc1, 0xc2, 0xc3, 0xc5, 0xc6, 0xc7, 0xc9, 0xca, 0xcb, 0xcd, 0xce, 0xcf,
];

pub fn get_size(data: &[u8]) -> Option<Size> {
    cursor_parser(data, |mut cursor| {
        cursor.seek(SeekFrom::Start(3))?;
        loop {
            let marker = cursor.read_u8()?;
            if START_OF_FRAMES.contains(&marker) {
                cursor.seek(SeekFrom::Current(3))?;
                let height = cursor.read_u16::<BigEndian>()?;
                let width = cursor.read_u16::<BigEndian>()?;
                return Ok(Some(Size::new(
                    width as u64,
                    height as u64,
                    MIME_TYPE.to_string(),
                    Animation::No,
                )));
            } else {
                let length = cursor.read_u16::<BigEndian>()?;
                cursor.seek(SeekFrom::Current(length as i64 - 2))?;
                if cursor.read_u8()? != 0xff {
                    return Ok(None);
                }
            }
        }
    })
}
