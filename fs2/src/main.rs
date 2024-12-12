use std::fs::{File, metadata};
use std::io::{self, Write};
use std::path::Path;
use fs2::FileExt;

fn main() -> io::Result<()> {
    println!("1");
    let path = Path::new("example.lock");
    println!("2");
    let file = File::create(&path)?;
    println!("3");

    // Try to lock the file
    file.lock_exclusive()?;
    println!("File locked.");

    // Get the metadata of the file
    let metadata = metadata(&path)?;

    // Get the file size
    let file_size = metadata.len();

    println!("File size: {} bytes", file_size);

    if file_size == 0 {
        // Simulate some work with the locked file
        std::thread::sleep(std::time::Duration::from_secs(20));

        // Write to the file (optional)
        writeln!(&file, "This is a test.")?;
    }

    // Unlock the file
    file.unlock()?;
    println!("File unlocked.");

    Ok(())
}
