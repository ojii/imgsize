use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::Size;
use crate::utils::format_parser;

const MIME_TYPE: &'static str = "image/png";
const SIGNATURE:  &'static [u8] =&[ 0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

pub fn get_size(data: &[u8]) -> Option<Size>{
    format_parser(data, SIGNATURE, |mut cursor| {
        cursor.seek(SeekFrom::Start(8))?;
        let mut chunk_type_buf = [0u8;4];
        loop {
            let chunk_length = cursor.read_u32::<BigEndian>()?;
            cursor.read_exact(&mut chunk_type_buf)?;
            match chunk_type_buf {
                [0x49, 0x48, 0x44, 0x52] => { // IHDR
                    if chunk_length!= 13 {
                        return Ok(None)
                    }
                    let width = cursor.read_u32::<BigEndian>()?;
                    let height = cursor.read_u32::<BigEndian>()?;
                    return Ok(Some(Size::new(width as u64, height as u64, MIME_TYPE.to_string())))
                }
                _ => {
                    cursor.seek(SeekFrom::Current(chunk_length as i64 + 4))?;
                }
            }
        }
    })
}
