use apple_flat_package::reader::PkgReader;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // File obtained from
    // https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/a989b093-c982-4d31-95d1-2c439f49b7e7/MicrosoftEdge-116.0.1938.76.pkg
    let source = r#"C:\Users\boni\Downloads\MicrosoftEdge-116.0.1938.76.pkg"#;
    // let target = r#"C:\Users\boni\Downloads\extract"#;
    let mut reader = PkgReader::new(File::open(source)?)?;
    let packages = reader.component_packages()?;
    println!("Number of components: {}", packages.len());

    Ok(())
}
