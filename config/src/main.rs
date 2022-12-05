const BROWSER_NAME: &str = "chrome";
const DRIVER_NAME: &str = "chromedriver";

pub trait SeleniumManager {
    fn get_config(&self) -> &ManagerConfig;

    fn set_config(&mut self, config: ManagerConfig);

    fn get_browser_version(&self) -> &str {
        self.get_config().browser_version.as_str()
    }

    fn set_browser_version(&mut self, browser_version: String) {
        let mut config = ManagerConfig::clone(self.get_config());
        config.browser_version = browser_version;
        self.set_config(config);
    }

    fn get_driver_version(&self) -> &str {
        self.get_config().driver_version.as_str()
    }

    fn set_driver_version(&mut self, driver_version: String) {
        let mut config = ManagerConfig::clone(self.get_config());
        config.driver_version = driver_version;
        self.set_config(config);
    }
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
}

impl ManagerConfig {
    pub fn default() -> ManagerConfig {
        ManagerConfig {
            browser_version: "b1".to_string(),
            driver_version: "d1".to_string(),
        }
    }

    pub fn clone(config: &ManagerConfig) -> ManagerConfig {
        ManagerConfig {
            browser_version: config.browser_version.as_str().to_string(),
            driver_version: config.driver_version.as_str().to_string(),
        }
    }
}

fn main() {
    let mut manager: Box<dyn SeleniumManager> = ChromeManager::new();

    let browser_version = manager.get_browser_version();
    let driver_version = manager.get_driver_version();
    println!("The default browser version is {browser_version}");
    println!("The default driver version is {driver_version}");

    manager.set_browser_version("b2".to_string());
    manager.set_driver_version("d2".to_string());
    let new_browser_version = manager.get_config().browser_version.as_str();
    let new_driver_version = manager.get_config().driver_version.as_str();
    println!("The new browser version is {new_browser_version}");
    println!("The new browser driver is {new_driver_version}");
}
