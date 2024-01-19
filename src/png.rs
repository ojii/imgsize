use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::utils::format_parser;
use crate::Size;

const MIME_TYPE: &str = "image/png";
const SIGNATURE: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

pub fn get_size(data: &[u8]) -> Option<Size> {
    format_parser(data, SIGNATURE, |mut cursor| {
        cursor.seek(SeekFrom::Start(8))?;
        let mut chunk_type_buf = [0u8; 4];
        let mut size = None;
        let mut animated = false;
        loop {
            let chunk_length = cursor.read_u32::<BigEndian>()?;
            cursor.read_exact(&mut chunk_type_buf)?;
            match chunk_type_buf {
                // IHDR
                [0x49, 0x48, 0x44, 0x52] => {
                    if chunk_length != 13 {
                        return Ok(None);
                    }
                    let width = cursor.read_u32::<BigEndian>()?;
                    let height = cursor.read_u32::<BigEndian>()?;
                    size = Some((width, height));
                    if animated {
                        break;
                    }
                    cursor.seek(SeekFrom::Current(chunk_length as i64 - 4))?;
                }
                // acTL
                [0x61, 0x63, 0x54, 0x4c] => {
                    animated = true;
                    if size.is_some() {
                        break;
                    }
                    cursor.seek(SeekFrom::Current(chunk_length as i64 + 4))?;
                }
                //IDAT
                [0x49, 0x44, 0x41, 0x54] => {
                    break;
                }
                _ => {
                    cursor.seek(SeekFrom::Current(chunk_length as i64 + 4))?;
                }
            }
        }
        Ok(size.map(|(width, height)| {
            Size::new(width as u64, height as u64, MIME_TYPE.to_string(), animated)
        }))
    })
}
