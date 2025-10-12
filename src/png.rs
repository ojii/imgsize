use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::utils::cursor_parser;
use crate::{Animation, Size};

const MIME_TYPE: &str = "image/png";

pub fn get_size(data: &[u8]) -> Option<Size> {
    cursor_parser(data, |mut cursor| {
        cursor.seek(SeekFrom::Start(8))?;
        let mut chunk_type_buf = [0u8; 4];
        let mut size = None;
        let mut animated = Animation::No;
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
                    if animated == Animation::Yes {
                        break;
                    }
                    cursor.seek(SeekFrom::Current(chunk_length as i64 - 4))?;
                }
                // acTL
                [0x61, 0x63, 0x54, 0x4c] => {
                    animated = Animation::Yes;
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
