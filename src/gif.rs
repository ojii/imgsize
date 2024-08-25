use std::io;
use std::io::{Cursor, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::utils::cursor_parser;
use crate::{Animation, Size};

const MIME_TYPE: &str = "image/gif";

pub fn get_size(data: &[u8]) -> Option<Size> {
    cursor_parser(data, |mut cursor| {
        // skip header
        cursor.seek(SeekFrom::Start(6))?;
        // Logical Screen Descriptor
        let width = cursor.read_u16::<LittleEndian>()? as u64;
        let height = cursor.read_u16::<LittleEndian>()? as u64;
        let flags = cursor.read_u8()?;
        // skip Background Color Index and Pixel Aspect Ratio
        cursor.seek(SeekFrom::Current(2))?;
        if let Some(size) = color_table_size(flags) {
            // skip Global Color Table
            cursor.seek(SeekFrom::Current(size))?;
        }
        let animation = detect_animation(&mut cursor)?;
        Ok(animation.map(|animation| Size::new(width, height, MIME_TYPE.to_string(), animation)))
    })
}

fn detect_animation(cursor: &mut Cursor<&[u8]>) -> io::Result<Option<Animation>> {
    match detect_animation_inner(cursor) {
        Ok(animation) => Ok(animation.map(|a| a.into())),
        Err(_) => Ok(Some(Animation::Unknown)),
    }
}

fn detect_animation_inner(cursor: &mut Cursor<&[u8]>) -> io::Result<Option<bool>> {
    let mut found_image = false;
    let mut gce_found = false;
    loop {
        match cursor.read_u8()? {
            // Image Descriptor
            0x2c => {
                if found_image {
                    return Ok(Some(true));
                } else if !gce_found {
                    return Ok(Some(false));
                }
                found_image = true;
                cursor.seek(SeekFrom::Current(8))?;
                let flags = cursor.read_u8()?;
                if let Some(size) = color_table_size(flags) {
                    cursor.seek(SeekFrom::Current(size))?;
                }
                // skip LZW Minimum Code Size
                cursor.seek(SeekFrom::Current(1))?;
                skip_data_sub_blocks(cursor)?;
            }
            // Extension
            0x21 => match cursor.read_u8()? {
                // Graphic Control Extension
                0xf9 => {
                    gce_found = true;
                    // skip block size (always 4) and extension data
                    cursor.seek(SeekFrom::Current(5))?;
                    skip_data_sub_blocks(cursor)?;
                }
                // Comment Extension
                0xfe => {
                    skip_data_sub_blocks(cursor)?;
                }
                // Plain Text Extension
                0x01 => {
                    // skip block size (always 12) and extension data
                    cursor.seek(SeekFrom::Current(13))?;
                    skip_data_sub_blocks(cursor)?;
                }
                // Application Extension
                0xff => {
                    // skip block size (always 11) and extension data
                    cursor.seek(SeekFrom::Current(12))?;
                    skip_data_sub_blocks(cursor)?;
                }
                _ => {
                    return Ok(None);
                }
            },
            // Trailer
            0x3B => {
                if found_image {
                    return Ok(Some(false));
                }
                return Ok(None);
            }
            _ => return Ok(None),
        }
    }
}

fn color_table_size(flags: u8) -> Option<i64> {
    // Ref : https://www.w3.org/Graphics/GIF/spec-gif89a.txt
    // 3 x 2^(Size of Global Color Table+1)
    if flags & (1 << 7) != 0 {
        Some((1 << ((flags & 0x07) + 1)) * 3)
    } else {
        None
    }
}

fn skip_data_sub_blocks(cursor: &mut Cursor<&[u8]>) -> io::Result<()> {
    loop {
        match cursor.read_u8()? {
            0x00 => return Ok(()),
            size => {
                cursor.seek(SeekFrom::Current(size as i64))?;
            }
        }
    }
}
