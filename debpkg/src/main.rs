use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = r#"C:\Users\boni\Downloads\microsoft-edge-stable_116.0.1938.76-1_amd64.deb"#;
    let target = r#"C:\Users\boni\Downloads\extract"#;
    let file = std::fs::File::open(source).unwrap();
    let mut pkg = debpkg::DebPkg::parse(file).unwrap();
    let control_tar = pkg.control().unwrap();
    let control = debpkg::Control::extract(control_tar).unwrap();
    println!("Package Name: {}", control.name());
    println!("Package Version: {}", control.version());
    let arch = control.get("Architecture").unwrap();
    println!("Package Architecture: {}", arch);

    let mut data = pkg.data().unwrap();
    data.unpack(target).unwrap();

    Ok(())
}
