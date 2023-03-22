use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use toml::{Table, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    println!("{:?}", config);
    let browser = config["browser"].as_str().unwrap();
    println!("browser: {}", browser);

    // -----

    let value = config["browser"].to_owned();
    let content = StringKey(value);
    println!("browser: {}", content.get_value());

    Ok(())
}

struct StringKey(Value);

impl StringKey {
    fn get_value(&self) -> &str {
        self.0.as_str().unwrap()
    }
}

fn get_config() -> Result<Table, Box<dyn Error>> {
    let config_path = Path::new("config.toml");
    Ok(read_to_string(config_path)?.parse()?)
}
