use directories::BaseDirs;
use std::error::Error;
use std::fs::read_to_string;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::string::ToString;
use std::{env, fs};
use toml::Table;

pub const CONFIG_FILE: &str = "selenium-manager-config.toml";
pub const ENV_PREFIX: &str = "SE_";
const CACHE_FOLDER: &str = ".cache/selenium";

fn main() -> Result<(), Box<dyn Error>> {
    let browser = StringKey("browser".to_string(), "".to_string());
    println!("browser: {}", browser.get_value());

    let driver = StringKey("driver".to_string(), "".to_string());
    println!("driver: {}", driver.get_value());

    let driver_ttl = IntegerKey("driver-ttl".to_string(), 0);
    println!("driver-ttl: {}", driver_ttl.get_value());

    let debug = BooleanKey("debug".to_string(), true);
    println!("debug: {}", debug.get_value());

    let no_string = StringKey("no-string".to_string(), "Default".to_string());
    println!("no-string: {}", no_string.get_value());

    let no_integer = IntegerKey("no-integer".to_string(), 10);
    println!("no-integer: {}", no_integer.get_value());

    let no_boolean = BooleanKey("no-boolean".to_string(), false);
    println!("no-boolean: {}", no_boolean.get_value());

    Ok(())
}

fn get_env_name(key: &str) -> String {
    let mut env_name: String = ENV_PREFIX.to_owned();
    let key_uppercase: String = key.replace('-', "_").to_uppercase();
    env_name.push_str(&key_uppercase);
    env_name
}

struct StringKey(String, String);

impl StringKey {
    fn get_value(&self) -> String {
        let config = get_config().unwrap_or_default();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_str().unwrap().to_string()
        } else {
            env::var(get_env_name(key)).unwrap_or(self.1.to_owned())
        }
    }
}

struct IntegerKey(String, i64);

impl IntegerKey {
    fn get_value(&self) -> i64 {
        let config = get_config().unwrap_or_default();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_integer().unwrap()
        } else {
            env::var(get_env_name(key))
                .unwrap_or_default()
                .parse::<i64>()
                .unwrap_or(self.1.to_owned())
        }
    }
}

struct BooleanKey(String, bool);

impl BooleanKey {
    fn get_value(&self) -> bool {
        let config = get_config().unwrap_or_default();
        let key = self.0.as_str();
        if config.contains_key(key) {
            config[key].as_bool().unwrap()
        } else {
            env::var(get_env_name(key))
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(self.1.to_owned())
        }
    }
}

fn get_config() -> Result<Table, Box<dyn Error>> {
    let config_path = get_config_path();
    Ok(read_to_string(config_path)?.parse()?)
}

fn get_config_path() -> PathBuf {
    get_cache_folder().join(CONFIG_FILE)
}

pub fn get_cache_folder() -> PathBuf {
    let cache_path = compose_cache_folder();
    create_path_if_not_exists(&cache_path);
    cache_path
}

pub fn create_path_if_not_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

pub fn compose_cache_folder() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        return Path::new(base_dirs.home_dir())
            .join(String::from(CACHE_FOLDER).replace('/', &MAIN_SEPARATOR.to_string()));
    }
    PathBuf::new()
}
