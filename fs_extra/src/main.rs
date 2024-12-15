use fs_extra::dir::{move_dir, CopyOptions};
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let input = Path::new(r#"C:\Users\boni\.cache\selenium\chrome\win64\132.0.6834.46"#);
    let target = r#"C:\Users\boni\.cache\selenium\target"#;
    let mut options = CopyOptions::new();
    options.content_only = true;
    move_dir(input, target, &options)?;

    Ok(())
}
