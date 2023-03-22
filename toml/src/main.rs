use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use toml::Table;

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("config.toml");
    let config: Table = read_to_string(config_path)?.parse()?;

    println!("{:?}", config);
    let browser = config["browser"].as_str().unwrap();
    println!("browser: {}", browser);

    Ok(())
}
