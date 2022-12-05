use std::env::consts::{ARCH, OS};

const BROWSER_NAME: &str = "chrome";
const DRIVER_NAME: &str = "chromedriver";

pub trait SeleniumManager {
    fn get_config(&self) -> &ManagerConfig;

    fn set_config(&mut self, config: ManagerConfig);
}

pub struct ChromeManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
    pub config: ManagerConfig,
}

impl ChromeManager {
    pub fn new() -> Box<Self> {
        Box::new(ChromeManager {
            browser_name: BROWSER_NAME,
            driver_name: DRIVER_NAME,
            config: ManagerConfig::default(),
        })
    }
}

impl SeleniumManager for ChromeManager {
    fn get_config(&self) -> &ManagerConfig {
        &self.config
    }

    fn set_config(&mut self, config: ManagerConfig) {
        self.config = config;
    }
}

pub struct ManagerConfig {
    pub browser_version: String,
    pub driver_version: String,
    pub os: String,
    pub arch: String,
}

impl ManagerConfig {
    pub fn default() -> ManagerConfig {
        ManagerConfig {
            browser_version: "AAA".to_string(),
            driver_version: "DDD".to_string(),
            os: OS.to_string(),
            arch: ARCH.to_string(),
        }
    }
}

fn main() {
    let mut manager: Box<dyn SeleniumManager> = ChromeManager::new();

    let browser_version = manager.get_config().browser_version.as_str();
    let driver_version = manager.get_config().driver_version.as_str();
    println!("The default browser version is {browser_version}");
    println!("The default driver version is {driver_version}");

    let mut new_config = ManagerConfig::default();
    new_config.browser_version = "111".to_string();
    new_config.driver_version = "222".to_string();

    manager.set_config(new_config);
    let new_browser_version = manager.get_config().browser_version.as_str();
    let new_driver_version = manager.get_config().driver_version.as_str();
    println!("The default browser version is {new_browser_version}");
    println!("The default browser driver is {new_driver_version}");
}
