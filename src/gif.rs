use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::utils::cursor_parser;
use crate::Size;

const MIME_TYPE: &str = "image/gif";
const ANIMATION_EXTENSION: [u8; 11] = [
    0x4e, 0x45, 0x54, 0x53, 0x43, 0x41, 0x50, 0x45, 0x32, 0x2e, 0x30,
];

pub fn get_size(data: &[u8]) -> Option<Size> {
    cursor_parser(data, |mut cursor| {
        cursor.seek(SeekFrom::Start(6))?;
        let width = cursor.read_u16::<LittleEndian>()?;
        let height = cursor.read_u16::<LittleEndian>()?;
        let gtc_flag = cursor.read_u8()?;
        let gce_offset = 0xd + if gtc_flag & (1 << 7) != 0 {
            // Ref : https://www.w3.org/Graphics/GIF/spec-gif89a.txt
            // 3 x 2^(Size of Global Color Table+1)
            (1 << ((gtc_flag & 0x07) + 1)) * 3
        } else {
            0
        };
        cursor.seek(SeekFrom::Start(gce_offset))?;
        let animated = if cursor.read_u8()? == 0x21 {
            cursor.seek(SeekFrom::Start(gce_offset + 3))?;
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
    })
}
