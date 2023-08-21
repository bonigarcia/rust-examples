use bzip2::read::BzDecoder;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // This file is downloaded from: https://ftp.mozilla.org/pub/firefox/releases/89.0.2/linux-x86_64/en-US/firefox-89.0.2.tar.bz2
    let input = File::open(r#"C:\Users\boni\Downloads\firefox-89.0.2.tar.bz2"#)?;

    let mut decompressor = BzDecoder::new(input);
    let mut buffer: Vec<u8> = Vec::new();
    decompressor.read_to_end(&mut buffer)?;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(r#"C:\Users\boni\Downloads\output.tar"#)?;
    file.write_all(&buffer)?;
    Ok(())
}
