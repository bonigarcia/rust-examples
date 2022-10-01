use std::error::Error;
use std::path::PathBuf;

use crate::browser::{BrowserManager, detect_browser_version};
use crate::files::compose_driver_path_in_cache;
use crate::metadata::{create_driver_metadata, get_driver_version_from_metadata, get_metadata, write_metadata};

const BROWSER_NAME: &str = "firefox";
const DRIVER_NAME: &str = "geckodriver";
const DRIVER_URL: &str = "https://github.com/mozilla/geckodriver/releases/download/";
const LATEST_RELEASE: &str = "0.31.0";

pub struct FirefoxManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
}

impl FirefoxManager {
    pub fn new() -> Box<Self> {
        Box::new(FirefoxManager {
            browser_name: BROWSER_NAME,
            driver_name: DRIVER_NAME,
        })
    }
}

impl BrowserManager for FirefoxManager {
    fn get_browser_name(&self) -> &str {
        self.browser_name
    }

    fn get_browser_version(&self, os: &str) -> Option<String> {
        let (shell, flag, args) = match os {
            "windows" => ("cmd", "/C", vec!(r#"cmd.exe /C wmic datafile where name='%PROGRAMFILES:\=\\%\\Mozilla Firefox\\firefox.exe' get Version /value"#,
                                            r#"cmd.exe /C wmic datafile where name='%PROGRAMFILES(X86):\=\\%\\Mozilla Firefox\\firefox.exe' get Version /value' get Version /value"#,
                                            r#"REG QUERY "HKCU\Software\Mozilla\Mozilla Firefox" /v CurrentVersion"#)),
            "macos" => ("sh", "-c", vec!(r#"/Applications/Firefox.app/Contents/MacOS/firefox -v"#)),
            _ => ("sh", "-c", vec!("firefox -v")),
        };
        detect_browser_version(self.browser_name, shell, flag, args)
    }

    fn get_driver_name(&self) -> &str {
        self.driver_name
    }

    fn get_driver_version(&self, browser_version: &str, _os: &str) -> Result<String, Box<dyn Error>> {
        let mut metadata = get_metadata();

        match get_driver_version_from_metadata(&metadata.drivers, self.driver_name, browser_version) {
            Some(v) => {
                log::trace!("Driver TTL is valid. Getting {} version from metadata", &self.driver_name);
                Ok(v)
            }
            _ => {
                let driver_version = LATEST_RELEASE.to_string(); // TODO use online info

                if !browser_version.is_empty() {
                    metadata.drivers.push(create_driver_metadata(browser_version, self.driver_name, &driver_version));
                    write_metadata(&metadata);
                }

                Ok(driver_version)
            }
        }
    }

    fn get_driver_url(&self, driver_version: &str, os: &str, arch: &str) -> String {
        let driver_label = if os.eq("windows") {
            if arch.eq("x86") {
                "win32.zip"
            }
            else {
                "win64.zip"
            }
        } else if os.eq("macos") {
            if arch.eq("aarch64") {
                "macos-aarch64.tar.gz"
            }
            else {
                "macos.tar.gz"
            }
        } else if arch.eq("x86") {
            "linux32.tar.gz"
        }
        else {
            "linux64.tar.gz"
        };
        format!("{}v{}/{}-v{}-{}", DRIVER_URL, driver_version, self.driver_name, driver_version, driver_label)
    }

    fn get_driver_path_in_cache(&self, driver_version: &str, os: &str, arch: &str) -> PathBuf {
        let arch_folder = if os.eq("windows") {
            if arch.eq("x86") {
                "win32"
            }
            else {
                "win64"
            }
        } else if os.eq("macos") {
            if arch.eq("aarch64") {
                "mac-arm64"
            }
            else {
                "mac64"
            }
        } else if arch.eq("x86") {
            "linux32"
        }
        else {
            "linux64"
        };
        compose_driver_path_in_cache(self.driver_name, os, arch_folder, driver_version)
    }
}