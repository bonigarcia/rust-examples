use apple_flat_package::reader::PkgReader;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Source file obtained from
    // https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/61b13da3-c921-482a-9166-743689310b71/MicrosoftEdge-122.0.2365.92.pkg
    let source = r#"C:\Users\boni\Downloads\MicrosoftEdge-122.0.2365.92.pkg"#;
    // let target = "/home/boni/Downloads/extract";
    // let target_path = Path::new(target);

    let mut reader = PkgReader::new(File::open(source)?)?;
    let packages = reader.component_packages()?;
    println!("Number of components: {}", packages.len());
    for package in packages.iter() {
        if let Some(cpio_reader) = package.payload_reader()? {
            for entry in cpio_reader {
                let e = entry?;
                let name = e.name();
                println!("---> {}", name);
            }
        }
    }

    Ok(())
}
