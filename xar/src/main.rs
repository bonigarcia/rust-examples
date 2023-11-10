use apple_flat_package::reader::PkgReader;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Source file obtained from
    // https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/a989b093-c982-4d31-95d1-2c439f49b7e7/MicrosoftEdge-116.0.1938.76.pkg
    let source = r#"C:\Users\boni\Downloads\MicrosoftEdge-116.0.1938.76.pkg"#;

    let mut reader = PkgReader::new(File::open(source)?)?;
    let packages = reader.component_packages()?;
    println!("Number of components: {}", packages.len());
    for package in packages.iter() {
        let package_info = package.package_info();
        println!("Package info: {:?}", package_info);
        let payload = &package_info.unwrap().payload;
        println!("Payload: {:?}", payload.clone().unwrap());
    }

    let resolve = reader.resolve_component("Payload")?;
    if resolve.is_some() {
        let component = resolve.unwrap();
        println!("Resolved payload: {:?}", component.package_info());
    }

    Ok(())
}
