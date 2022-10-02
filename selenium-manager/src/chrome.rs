use std::error::Error;
use std::path::PathBuf;

use crate::downloads::read_content_from_link;
use crate::files::compose_driver_path_in_cache;
use crate::manager::{BrowserManager, detect_browser_version, get_major_version};
use crate::manager::ARCH::ARM64;
use crate::manager::OS::{MACOS, WINDOWS};
use crate::metadata::{create_driver_metadata, get_driver_version_from_metadata, get_metadata, write_metadata};

const BROWSER_NAME: &str = "chrome";
const DRIVER_NAME: &str = "chromedriver";
const DRIVER_URL: &str = "https://chromedriver.storage.googleapis.com/";
const LATEST_RELEASE: &str = "LATEST_RELEASE";

pub struct ChromeManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
}

impl ChromeManager {
    pub fn new() -> Box<Self> {
        Box::new(ChromeManager {
            browser_name: BROWSER_NAME,
            driver_name: DRIVER_NAME,
        })
    }
}

impl BrowserManager for ChromeManager {
    fn get_browser_name(&self) -> &str {
        self.browser_name
    }

    fn get_browser_version(&self, os: &str) -> Option<String> {
        let (shell, flag, args) = if WINDOWS.is(os) {
            ("cmd", "/C", vec!(r#"wmic datafile where name='%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                               r#"wmic datafile where name='%PROGRAMFILES(X86):\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                               r#"wmic datafile where name='%LOCALAPPDATA:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#,
                               r#"REG QUERY HKCU\Software\Google\Chrome\BLBeacon /v version"#))
        } else if MACOS.is(os) {
            ("sh", "-c", vec!(r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --version"#))
        } else {
            ("sh", "-c", vec!("google-chrome --version"))
        };
        detect_browser_version(self.browser_name, shell, flag, args)
    }

    fn get_driver_name(&self) -> &str {
        self.driver_name
    }

    fn get_driver_version(&self, browser_version: &str, _os: &str) -> Result<String, Box<dyn Error>> {
        let mut metadata = get_metadata();

        match get_driver_version_from_metadata(&metadata.drivers, self.driver_name, browser_version) {
            Some(driver_version) => {
                log::trace!("Driver TTL is valid. Getting {} version from metadata", &self.driver_name);
                Ok(driver_version)
            }
            _ => {
                let driver_url = if browser_version.is_empty() {
                    format!("{}{}", DRIVER_URL, LATEST_RELEASE)
                } else {
                    format!("{}{}_{}", DRIVER_URL, LATEST_RELEASE, browser_version)
                };
                log::debug!("Reading {} version from {}", &self.driver_name, driver_url);
                let driver_version = read_content_from_link(driver_url)?;

                if !browser_version.is_empty() {
                    metadata.drivers.push(create_driver_metadata(browser_version, self.driver_name, &driver_version));
                    write_metadata(&metadata);
                }
                Ok(driver_version)
            }
        }
    }

    fn get_driver_url(&self, driver_version: &str, os: &str, arch: &str) -> String {
        let driver_label = if WINDOWS.is(os) {
            "win32"
        } else if MACOS.is(os) {
            if ARM64.is(arch) {
                // As of chromedriver 106, the naming convention for macOS ARM64 releases changed. See:
                // https://groups.google.com/g/chromedriver-users/c/JRuQzH3qr2c
                let major_driver_version = get_major_version(driver_version).parse::<i32>().unwrap();
                if major_driver_version < 106 {
                    "mac64_m1"
                } else {
                    "mac_arm64"
                }
            } else {
                "mac64"
            }
        } else {
            "linux64"
        };
        format!("{}{}/{}_{}.zip", DRIVER_URL, driver_version, self.driver_name, driver_label)
    }

    fn get_driver_path_in_cache(&self, driver_version: &str, os: &str, arch: &str) -> PathBuf {
        let arch_folder = if WINDOWS.is(os) {
            "win32"
        } else if MACOS.is(os) {
            if ARM64.is(arch) {
                "mac-arm64"
            } else {
                "mac64"
            }
        } else {
            "linux64"
        };
        compose_driver_path_in_cache(self.driver_name, os, arch_folder, driver_version)
    }
}