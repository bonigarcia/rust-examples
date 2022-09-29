use std::error::Error;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;

use directories::BaseDirs;

use selenium_manager::{BrowserManager, CACHE_FOLDER, detect_browser_version};

const CHROME: &str = "chrome";
const CHROMEDRIVER: &str = "chromedriver";
const CHROMEDRIVER_URL: &str = "https://chromedriver.storage.googleapis.com/";

pub struct ChromeManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
}

impl ChromeManager {
    pub fn new() -> Box<Self> {
        Box::new(ChromeManager {
            browser_name: CHROME,
            driver_name: CHROMEDRIVER,
        })
    }
}

impl BrowserManager for ChromeManager {
    fn get_browser_name(&self) -> &str {
        self.browser_name
    }

    fn get_driver_name(&self) -> &str {
        self.driver_name
    }

    fn get_browser_version(&self, os: &str) -> Result<String, String> {
        let (shell, flag, args) = match os {
            "windows" => ("cmd", "/C", vec!(r#"wmic datafile where name='%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                                            r#"wmic datafile where name='%PROGRAMFILES(X86):\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                                            r#"wmic datafile where name='%LOCALAPPDATA:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                                            r#"REG QUERY HKCU\Software\Google\Chrome\BLBeacon /v version"#)),
            "macos" => ("sh", "-c", vec!(r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --version"#)),
            _ => ("sh", "-c", vec!("google-chrome --version")),
        };
        detect_browser_version(self.browser_name, shell, flag, args)
    }

    fn get_driver_url(&self, driver_version: &String, os: &str, arch: &str) -> String {
        let m1 = get_m1_prefix(&arch);
        match os {
            "windows" => format!("{}{}/{}_win32.zip", CHROMEDRIVER_URL, driver_version, self.driver_name),
            "macos" => format!("{}{}/{}_mac64{}.zip", CHROMEDRIVER_URL, driver_version, self.driver_name, m1),
            _ => format!("{}{}/{}_linux64.zip", CHROMEDRIVER_URL, driver_version, self.driver_name),
        }
    }

    #[tokio::main]
    async fn get_driver_version(&self, browser_version: &String) -> Result<String, Box<dyn Error>> {
        let driver_url = format!("{}LATEST_RELEASE_{}", CHROMEDRIVER_URL, browser_version);
        let driver_version = reqwest::get(driver_url).await?.text().await?;

        Ok(driver_version)
    }

    fn get_cache_path(&self, driver_version: &String, os: &str, arch: &str) -> PathBuf {
        let m1 = get_m1_prefix(&arch);
        let arch_folder = match os {
            "windows" => String::from("win32"),
            "macos" => format!("mac64{}", m1),
            _ => String::from("linux64")
        };
        let cache_folder = String::from(CACHE_FOLDER).replace("/", &*String::from(MAIN_SEPARATOR));
        let base_dirs = BaseDirs::new().unwrap();
        Path::new(base_dirs.home_dir())
            .join(cache_folder)
            .join(self.driver_name)
            .join(arch_folder)
            .join(driver_version)
    }
}

fn get_m1_prefix(arch: &str) -> &str {
    match arch {
        "aarch64" => "_m1",
        _ => "",
    }
}