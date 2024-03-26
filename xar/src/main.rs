use apple_flat_package::reader::PkgReader;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Source file obtained from
    // https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/61b13da3-c921-482a-9166-743689310b71/MicrosoftEdge-122.0.2365.92.pkg
    // let source = r#"C:\Users\boni\Downloads\MicrosoftEdge-122.0.2365.92.pkg"#;
    // let target = r#"C:\Users\boni\Downloads\extract"#;
    let source = "/home/boni/Downloads/MicrosoftEdge-122.0.2365.92.pkg";
    let target = "/home/boni/Downloads/extract";

    let target_path = Path::new(target);

    let mut reader = PkgReader::new(File::open(source)?)?;
    // let mut xar_reader = reader.into_inner();
    // xar_reader.unpack(target_path)?;

    let packages = reader.component_packages()?;
    println!("Number of components: {}", packages.len());
    for package in packages.iter() {
        if let Some(mut cpio_reader) = package.payload_reader()? {
            while let Some(next) = cpio_reader.next() {
                let entry = next?;
                let name = entry.name();
                let mut file = Vec::new();
                cpio_reader.read_to_end(&mut file)?;
                println!("---> {}", name);

                let target_path_buf = target_path.join(name);
                if entry.file_size() != 0 {
                    let target_path = target_path_buf.as_path();
                    fs::create_dir_all(target_path.parent().unwrap())?;
                    fs::write(&target_path_buf, file)?;

                    // Set permissions
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;

                        let mode = entry.mode();
                        fs::set_permissions(target_path, fs::Permissions::from_mode(mode))?;
                    }
                }
            }
        }
    }

    Ok(())
}
