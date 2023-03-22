use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use toml::Table;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    println!("{:?}", config);
    let browser = config["browser"].as_str().unwrap();
    println!("browser: {}", browser);

    // -----

    let browser = StringKey("browser".to_string());
    println!("browser: {}", browser.get_value());

    let driver = StringKey("driver".to_string());
    println!("driver: {}", driver.get_value());

    let driver_ttl = IntegerKey("driver-ttl".to_string());
    println!("driver-ttl: {}", driver_ttl.get_value());

    let debug = BooleanKey("debug".to_string());
    println!("debug: {}", debug.get_value());

    Ok(())
}

struct StringKey(String);

impl StringKey {
    fn get_value(&self) -> String {
        let config = get_config().unwrap();
        config[self.0.as_str()].as_str().unwrap().to_string()
    }
}

struct IntegerKey(String);

impl IntegerKey {
    fn get_value(&self) -> i64 {
        let config = get_config().unwrap();
        config[self.0.as_str()].as_integer().unwrap()
    }
}

struct BooleanKey(String);

impl BooleanKey {
    fn get_value(&self) -> bool {
        let config = get_config().unwrap();
        config[self.0.as_str()].as_bool().unwrap()
    }
}

fn get_config() -> Result<Table, Box<dyn Error>> {
    let config_path = Path::new("config.toml");
    Ok(read_to_string(config_path)?.parse()?)
}
