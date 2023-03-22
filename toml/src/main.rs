use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::string::ToString;
use toml::Table;

pub const DEFAULT_STRING: &str = "";
pub const DEFAULT_INTEGER: i64 = 0;
pub const DEFAULT_BOOLEAN: bool = false;

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

    let no_string = StringKey("no-string".to_string());
    println!("no-string: {}", no_string.get_value());

    let no_integer = IntegerKey("no-integer".to_string());
    println!("no-integer: {}", no_integer.get_value());

    let no_boolean = BooleanKey("no-boolean".to_string());
    println!("no-boolean: {}", no_boolean.get_value());

    Ok(())
}

struct StringKey(String);

impl StringKey {
    fn get_value(&self) -> String {
        let config = get_config().unwrap();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_str().unwrap().to_string()
        } else {
            DEFAULT_STRING.to_string()
        }
    }
}

struct IntegerKey(String);

impl IntegerKey {
    fn get_value(&self) -> i64 {
        let config = get_config().unwrap();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_integer().unwrap()
        } else {
            DEFAULT_INTEGER
        }
    }
}

struct BooleanKey(String);

impl BooleanKey {
    fn get_value(&self) -> bool {
        let config = get_config().unwrap();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_bool().unwrap()
        } else {
            DEFAULT_BOOLEAN
        }
    }
}

fn get_config() -> Result<Table, Box<dyn Error>> {
    let config_path = Path::new("config.toml");
    Ok(read_to_string(config_path)?.parse()?)
}
