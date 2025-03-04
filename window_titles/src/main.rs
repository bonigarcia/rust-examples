use std::error::Error;
use std::fs;


fn force_error() -> anyhow::Result<()> {
    let source = r#"C:\Users\boni\Downloads\chrome-win"#;
    let target = r#"C:\Users\boni\Downloads\target"#;

    // When target already exists, there will be the following error:
    // Error: Os { code: 5, kind: PermissionDenied, message: "Access is denied." }
    fs::rename(source, target)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // When RUST_BACKTRACE=1, we will get the backtrace when running this program
    force_error()?;

    Ok(())
}