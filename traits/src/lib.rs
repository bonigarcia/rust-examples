pub trait BrowserManager {
    fn get_browser_version(&self, version: &str) -> String;
}

pub struct ChromeManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
}

impl BrowserManager for ChromeManager {
    fn get_browser_version(&self, version: &str) -> String {
        format!("{} {}", self.browser_name, version)
    }
}

pub struct FirefoxManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
}

impl BrowserManager for FirefoxManager {
    fn get_browser_version(&self, version: &str) -> String {
        format!("{} {}", self.browser_name, version)
    }
}
