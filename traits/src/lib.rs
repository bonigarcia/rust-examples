pub trait BrowserManager {
    fn get_browser_version(&self) -> String;
}

pub struct ChromeManager {
    pub browser_name : &'static str,
    pub driver_name : &'static str,
}

impl BrowserManager for ChromeManager {
    fn get_browser_version(&self) -> String {
        format!("{} 106", self.browser_name)
    }
}

pub struct FirefoxManager {
    pub browser_name : &'static str,
    pub driver_name : &'static str,
}

impl BrowserManager for FirefoxManager {
    fn get_browser_version(&self) -> String {
        format!("{} 105", self.browser_name)
    }
}