use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};

const SEVEN_ZIP_HEADER: &[u8; 6] = b"7z\xBC\xAF\x27\x1C";

fn main() -> Result<(), Box<dyn Error>> {
    let sfx_file = r#"C:\Users\boni\Downloads\Firefox Setup 116.0.2.exe"#;
    let target = r#"C:\Users\boni\Downloads\tmp"#;
    // let source = r#"C:\Users\boni\Downloads\Firefox Setup 116.0.2 (mod).7z"#;
    // sevenz_rust::decompress_file(source, target).unwrap();

    let file_bytes = read_bytes_from_file(sfx_file)?;
    println!("{}", file_bytes.len());

    let header = find_bytes(&file_bytes, SEVEN_ZIP_HEADER);
    println!("----> {:?}", header);

    if header.is_some() {
        let index = header.unwrap();
        let cursor = Cursor::new(&file_bytes[index..]);
        sevenz_rust::decompress(cursor, target).unwrap();
    }

    Ok(())
}

fn read_bytes_from_file(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn find_bytes(buffer: &Vec<u8>, bytes: &[u8]) -> Option<usize> {
    buffer
        .windows(bytes.len())
        .position(|window| window == bytes)
}
