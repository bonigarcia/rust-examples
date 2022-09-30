use std::error::Error;
use std::path::PathBuf;

use selenium_manager::{BrowserManager, create_driver_path, detect_browser_major_version, get_browser_ttl, get_driver_ttl, get_metadata, is_ttl_valid, write_metadata};

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

    fn get_browser_version(&self, os: &str) -> Result<String, String> {
        let mut metadata = get_metadata();

        let browser_version;
        if is_ttl_valid(metadata.chrome.browser_version_ttl) {
            log::debug!("Browser TTL is valid. Getting {} version from metadata", self.browser_name);
            Ok(metadata.chrome.browser_version)
        } else {
            log::debug!("Browser TTL is stale. Running command to find out {} version", self.browser_name);
            let (shell, flag, args) = match os {
                "windows" => ("cmd", "/C", vec!(r#"wmic datafile where name='%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                                                r#"wmic datafile where name='%PROGRAMFILES(X86):\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                                                r#"wmic datafile where name='%LOCALAPPDATA:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                                                r#"REG QUERY HKCU\Software\Google\Chrome\BLBeacon /v version"#)),
                "macos" => ("sh", "-c", vec!(r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --version"#)),
                _ => ("sh", "-c", vec!("google-chrome --version")),
            };
            browser_version = detect_browser_major_version(self.browser_name, shell, flag, args);

            if !browser_version.is_empty() {
                metadata.chrome.browser_version  = browser_version.to_string();
                metadata.chrome.browser_version_ttl = get_browser_ttl();
                write_metadata(&metadata);
            }

            Ok(browser_version)
        }
    }

    fn get_driver_name(&self) -> &str {
        self.driver_name
    }

    #[tokio::main]
    async fn get_driver_version(&self, browser_version: &str) -> Result<String, Box<dyn Error>> {
        let mut metadata = get_metadata();

        let driver_version;
        if is_ttl_valid(metadata.chrome.driver_version_ttl) {
            log::debug!("Driver TTL is valid. Getting {} version from metadata", &self.driver_name);
            driver_version = metadata.chrome.driver_version;
        } else {
            let driver_url = format!("{}LATEST_RELEASE_{}", CHROMEDRIVER_URL, browser_version);
            log::debug!("Driver TTL is stale. Reading {} version from {}", &self.driver_name, driver_url);
            driver_version = reqwest::get(driver_url).await?.text().await?;

            metadata.chrome.driver_version = driver_version.to_string();
            metadata.chrome.driver_version_ttl = get_driver_ttl();
            write_metadata(&metadata);
        }

        Ok(driver_version)
    }

    fn get_driver_url(&self, driver_version: &str, os: &str, arch: &str) -> String {
        let m1 = match arch {
            "aarch64" => "_m1",
            _ => "",
        };
        match os {
            "windows" => format!("{}{}/{}_win32.zip", CHROMEDRIVER_URL, driver_version, self.driver_name),
            "macos" => format!("{}{}/{}_mac64{}.zip", CHROMEDRIVER_URL, driver_version, self.driver_name, m1),
            _ => format!("{}{}/{}_linux64.zip", CHROMEDRIVER_URL, driver_version, self.driver_name),
        }
    }

    fn get_driver_path_in_cache(&self, driver_version: &str, os: &str, arch: &str) -> PathBuf {
        let mut arch_folder = match os {
            "windows" => "win32",
            "macos" => "mac64",
            _ => "linux64",
        };
        if os.eq("macos") && arch.eq("aarch64") {
            arch_folder = "mac-arm64";
        }
        create_driver_path(self.driver_name, os, arch_folder, driver_version)
    }
}