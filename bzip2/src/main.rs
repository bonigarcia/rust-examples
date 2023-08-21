use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // This file is downloaded from: https://ftp.mozilla.org/pub/firefox/releases/89.0.2/linux-x86_64/en-US/firefox-89.0.2.tar.bz2
    let input = File::open("/home/boni/Downloads/firefox-89.0.2.tar.bz2")?; // This fails, as follows:
    // Error: Custom { kind: Other, error: BlockError { reason: "huffman bitstream truncated" } }

    // let input = File::open("/home/boni/Downloads/firefox-90.0.2.tar.bz2")?; // This works
    // This file is downloaded from: https://ftp.mozilla.org/pub/firefox/releases/90.0.2/linux-x86_64/en-US/firefox-90.0.2.tar.bz2

    let mut output = File::create("/home/boni/Downloads/output")?;
    let mut decoder = bzip2_rs::DecoderReader::new(input);
    std::io::copy(&mut decoder, &mut output)?;

    Ok(())
}
