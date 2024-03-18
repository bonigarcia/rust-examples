use anyhow::Error;
use std::fs;
use std::io::{Cursor, Read};
use std::path::Path;
use xz2::bufread::XzDecoder;

fn main() -> Result<(), Error> {
    // let source = r#"C:\Users\boni\Downloads\microsoft-edge-stable-122.0.2365.80-1.x86_64.rpm"#;
    // let target = r#"C:\Users\boni\Downloads\extract"#;
    let source = "/home/boni/Downloads/microsoft-edge-stable-122.0.2365.92-1.x86_64.rpm";
    let target = "/home/boni/Downloads/extract";

    uncompress_xz(source, target)?;

    Ok(())
}

pub fn uncompress_xz(source: &str, target: &str) -> Result<(), Error> {
    let pkg = rpm::Package::open(source)?;

    let name = pkg.metadata.get_name()?;
    let version = pkg.metadata.get_version()?;
    let release = pkg.metadata.get_release()?;
    let arch = pkg.metadata.get_arch()?;
    println!("----> {}-{}-{}.{}", name, version, release, arch);

    let cursor = Cursor::new(pkg.content);
    let target_path = Path::new(target);
    let mut xz_decoder = XzDecoder::new(cursor);
    let mut buffer: Vec<u8> = Vec::new();
    xz_decoder.read_to_end(&mut buffer)?;

    for entry in cpio_reader::iter_files(&buffer) {
        let name = entry.name();
        let file = entry.file();

        let target_path_buf = target_path.join(name);

        if file.len() != 0 {
            let target_path = target_path_buf.as_path();
            fs::create_dir_all(target_path.parent().unwrap())?;
            fs::write(&target_path_buf, file)?;

            // Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let mode = entry.mode();
                fs::set_permissions(&target_path, fs::Permissions::from_mode(mode.bits()))?;
            }
        }
    }

    Ok(())
}
