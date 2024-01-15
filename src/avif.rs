use crate::Size;

const MIME_TYPE: &str = "image/avif";
pub fn get_size(data: &[u8]) -> Option<Size> {
    let mut itr = BMFFBoxIter::new(data);
    let ftyp = itr.next()?;
    if ftyp.box_type != b"ftyp" {
        return None;
    }
    if &ftyp.data.get(..4)? != b"avif" {
        return None;
    }
    let meta = itr.next()?;
    if meta.box_type != b"meta" {
        return None;
    }
    let iprp = meta.find_inner_box_after(b"iprp", 4)?;
    let ipco = iprp.find_inner_box(b"ipco")?;
    let ispe = ipco.find_inner_box(b"ispe")?;
    let width = u32::from_be_bytes(ispe.data.get(4..8)?.try_into().ok()?);
    let height = u32::from_be_bytes(ispe.data.get(8..12)?.try_into().ok()?);
    Some(Size::new(
        width as u64,
        height as u64,
        MIME_TYPE.to_string(),
        false,
    ))
}

struct BMFFBoxIter<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BMFFBoxIter<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
}

impl<'a> Iterator for BMFFBoxIter<'a> {
    type Item = BMFFBox<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = &self.data.get(self.pos..)?;
        let bmffbox = BMFFBox::from_data(data)?;
        self.pos += bmffbox.size;
        Some(bmffbox)
    }
}

struct BMFFBox<'a> {
    data: &'a [u8],
    size: usize,
    box_type: &'a [u8; 4],
}

impl<'a> BMFFBox<'a> {
    fn from_data(data: &'a [u8]) -> Option<Self> {
        let size_hint = u32::from_be_bytes(data.get(..4)?.try_into().ok()?);
        let box_type = data.get(4..8)?.try_into().ok()?;
        let (size, big) = match size_hint {
            0 => None,
            1 => {
                let size = u64::from_be_bytes(data.get(8..16)?.try_into().ok()?);
                if size < 16 {
                    None
                } else {
                    Some((size as usize, true))
                }
            }
            _ => {
                if size_hint < 8 {
                    None
                } else {
                    Some((size_hint as usize, false))
                }
            }
        }?;
        let offset = if big { 16 } else { 8 };
        Some(Self {
            box_type,
            size,
            data: data.get(offset..size)?,
        })
    }

    fn find_inner_box_after(&self, box_type: &[u8; 4], after: usize) -> Option<BMFFBox> {
        BMFFBoxIter::new(self.data.get(after..)?).find(|bfmmbox| bfmmbox.box_type == box_type)
    }

    fn find_inner_box(&self, box_type: &[u8; 4]) -> Option<BMFFBox> {
        self.find_inner_box_after(box_type, 0)
    }
}
