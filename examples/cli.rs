extern crate imgsize;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Arguments {
    #[arg(short, long, default_value_t = 1024)]
    buf_size: usize,
    paths: Vec<PathBuf>,
}

pub fn main() -> io::Result<()> {
    let arguments = Arguments::parse();
    let mut buffer = vec![0u8; arguments.buf_size];
    for path in arguments.paths.iter() {
        let name = path.to_str().unwrap();
        let mut file = File::open(&path)?;
        let read = file.read(&mut buffer)?;
        match imgsize::get_size(&buffer[..read]) {
            Some(size) => println!(
                "{}: {}x{}, {}, animated={}",
                name, size.width, size.height, size.mime_type, size.is_animated
            ),
            None => println!("{}: unsupported format", name),
        }
        buffer.fill(0);
    }
    Ok(())
}
