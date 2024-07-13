use crate::Size;

const MIME_TYPE: &str = "image/avif";
pub fn get_size(data: &[u8]) -> Option<Size> {
    let mut itr = BMFFBoxIter::new(data);
    let animated = itr.next_typed(b"ftyp").and_then(|bmff| {
        let ftyp = Ftyp::maybe_from(bmff)?;
        match ftyp.major_brand {
            b"avif" => Some(ftyp.minor_brands.contains(&b"avis")),
            b"avis" => Some(true),
            _ => None,
        }
    })?;
    let meta = itr.next_typed(b"meta")?;
    let iprp = meta.find_inner_box_after(b"iprp", 4)?;
    let ipco = iprp.find_inner_box(b"ipco")?;
    let ispe = ipco.find_inner_box(b"ispe")?;
    let width = get_u32(ispe.data, 4)?;
    let height = get_u32(ispe.data, 8)?;
    Some(Size::new(
        width as u64,
        height as u64,
        MIME_TYPE.to_string(),
        animated,
    ))
}

#[derive(Debug)]
struct Ftyp<'a> {
    major_brand: &'a [u8; 4],
    minor_brands: Vec<&'a [u8; 4]>,
}

impl<'a> Ftyp<'a> {
    fn maybe_from(bmff: BMFFBox<'a>) -> Option<Self> {
        let num_minors = (bmff.data.len() - 8) / 4;
        let mut minors = Vec::with_capacity(num_minors);
        for index in 0..num_minors {
            let start = 8 + (index * 4);
            let end = start + 4;
            minors.push(bmff.data.get(start..end)?.try_into().ok()?);
        }
        Some(Self {
            major_brand: bmff.data.get(..4)?.try_into().ok()?,
            minor_brands: minors,
        })
    }
}

struct BMFFBoxIter<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BMFFBoxIter<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn next_typed(&mut self, tag: &[u8; 4]) -> Option<BMFFBox<'a>> {
        self.next().filter(|bmffbox| bmffbox.box_type == tag)
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
        let size_hint = get_u32(data, 0)?;
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

    fn find_inner_box_after(&self, box_type: &[u8; 4], skipping: usize) -> Option<BMFFBox> {
        BMFFBoxIter::new(self.data.get(skipping..)?).find(|bfmmbox| bfmmbox.box_type == box_type)
    }

    fn find_inner_box(&self, box_type: &[u8; 4]) -> Option<BMFFBox> {
        self.find_inner_box_after(box_type, 0)
    }
}

fn get_u32(data: &[u8], start: usize) -> Option<u32> {
    data.get(start..start + 4)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u32::from_be_bytes)
}
