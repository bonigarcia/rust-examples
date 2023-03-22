use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::string::ToString;
use toml::Table;

pub const CONFIG_FILENAME: &str = "config.toml";
pub const ENV_PREFIX: &str = "SE_";

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

fn get_env_name(key: &str) -> String {
    let mut env_name: String = ENV_PREFIX.to_owned();
    let key_uppercase: String = key.replace('-', "_").to_uppercase();
    env_name.push_str(&key_uppercase);
    env_name
}

struct StringKey(String);

impl StringKey {
    fn get_value(&self) -> String {
        let config = get_config().unwrap();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_str().unwrap().to_string()
        } else {
            env::var(get_env_name(key)).unwrap_or_default()
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
            env::var(get_env_name(key))
                .unwrap_or_default()
                .parse::<i64>()
                .unwrap_or_default()
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
            env::var(get_env_name(key))
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or_default()
        }
    }
}

fn get_config() -> Result<Table, Box<dyn Error>> {
    let config_path = Path::new(CONFIG_FILENAME);
    Ok(read_to_string(config_path)?.parse()?)
}
