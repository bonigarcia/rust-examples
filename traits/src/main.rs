use traits::{BrowserManager, ChromeManager, FirefoxManager};

fn main() {
    let is_chrome = true;
    let browser_driver : Box<dyn BrowserManager>;

    if is_chrome {
        browser_driver = Box::new(ChromeManager{browser_name: "chrome", driver_name: "chromedriver" } );
    }
    else {
        browser_driver = Box::new(FirefoxManager{browser_name: "firefox", driver_name: "geckodriver" } );
    }

    println!("Browser version: {}", browser_driver.get_browser_version());
}