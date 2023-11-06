use apple_xar::reader::XarReader;
use std::error::Error;
use std::fs::File;
use tar::Archive;
use flate2::read::GzDecoder;

fn main() -> Result<(), Box<dyn Error>> {
    let source = r#"C:\Users\boni\Downloads\MicrosoftEdge-119.0.2151.46.pkg"#;
    let target = r#"C:\Users\boni\Downloads\extract"#;

    let mut reader = XarReader::new(File::open(source)?)?;
    reader.unpack(target)?;

    // let payload = File::open(r#"C:\Users\boni\Downloads\extract\MicrosoftEdge-119.0.2151.46.pkg\Payload"#)?;
    // let tar = GzDecoder::new(&payload);
    //
    // let mut archive = Archive::new(tar);
    // archive.unpack(target)?;

    Ok(())
}
