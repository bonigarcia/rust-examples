use std::error::Error;
use std::io::Cursor;
use apple_xar::reader::XarReader;

fn main() -> Result<(), Box<dyn Error>> {
    let source = r#"C:\Users\boni\Downloads\MicrosoftEdge-116.0.1938.76.pkg"#;
    let target = r#"C:\Users\boni\Downloads\extract"#;

    let cursor = Cursor::new(source);
    let mut reader = XarReader::new(cursor)?;
    reader.unpack(target)?;

    Ok(())
}

