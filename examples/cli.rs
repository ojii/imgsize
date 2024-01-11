extern crate imgsize;
use std::{env, io};
use std::fs::File;
use std::io::Read;

pub fn main() -> io::Result<()> {
    let mut buffer = [0u8;1024];
    for path in env::args().skip(1) {
        let mut file = File::open(&path)?;
        file.read(&mut buffer)?;
        match imgsize::get_size(&buffer) {
            Some(size) => println!("{}: {}x{} ({})", path, size.width, size.height, size.mime_type),
            None => println!("{}: unsupported format", path)
        }
    }
    Ok(())
}