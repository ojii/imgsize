use std::env::args;
use std::fs::File;
use std::io::Read;

use imgsize::get_size;

fn main() {
    let mut buf = [0u8; 1000];
    for path in args().skip(1) {
        match File::open(&path) {
            Err(e) => eprintln!("{path}: {e}"),
            Ok(mut file) => match file.read(&mut buf) {
                Ok(_) => match get_size(&buf) {
                    None => println!("{path}: could not detect size"),
                    Some(size) => println!(
                        "{path}: {}x{}, {}, animated={}",
                        size.width, size.height, size.mime_type, size.is_animated
                    ),
                },
                Err(e) => eprintln!("{path}: {e}"),
            },
        };
        buf.fill(0);
    }
}
