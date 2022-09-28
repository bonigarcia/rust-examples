use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;
use std::process::Command;

use directories::BaseDirs;
use regex::Regex;
use tempfile::{Builder, TempDir};
use zip::ZipArchive;

const CHROME: &str = "chrome";
const CHROMEDRIVER: &str = "chromedriver";
const CHROMEDRIVER_URL: &str = "https://chromedriver.storage.googleapis.com/";
const CACHE_FOLDER: &str = ".cache/selenium";

pub trait BrowserManager {
    fn get_browser_name(&self) -> &str;

    fn get_driver_name(&self) -> &str;

    fn get_browser_version(&self, os: &str) -> String;

    fn get_driver_url(&self, driver_version: &String, os: &str, arch: &str) -> String;

    fn get_driver_version(&self, browser_version: &String) -> Result<String, Box<dyn Error>>;

    fn download_driver(&self, driver_version: &String, os: &str, arch: &str) -> Result<(), Box<dyn Error>>;

    fn get_m1_prefix(&self, arch: &str) -> &str;
}

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

    fn get_browser_version(&self, os: &str) -> String {
        let command = match os {
            "windows" => "cmd",
            _ => "sh"
        };
        let args = match os {
            "windows" => ["/C", r#"wmic datafile where name='%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#],
            "macos" => ["-c", r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --version"#],
            _ => ["-c", "google-chrome --version"],
        };
        log::debug!("Running shell command: {:?}", args);

        let output = Command::new(command)
            .args(args)
            .output()
            .expect("command failed to start");
        log::debug!("{:?}", output);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let re = Regex::new(r"[^\d^.]").unwrap();

        let browser_version = re.replace_all(&*stdout, "").to_string();
        log::debug!("Your browser version is {}", browser_version);

        let browser_version_vec: Vec<&str> = browser_version.split(".").collect();
        browser_version_vec.get(0).unwrap().to_string()
    }

    fn get_driver_url(&self, driver_version: &String, os: &str, arch: &str) -> String {
        let m1 = Self::get_m1_prefix(&self, &arch);
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

    fn download_driver(&self, driver_version: &String, os: &str, arch: &str) -> Result<(), Box<dyn Error>> {
        let url = Self::get_driver_url(&self, &driver_version, os, arch);
        let (_tmp_dir, target_path) = download_file(url)?;

        let m1 = Self::get_m1_prefix(&self, &arch);
        let arch_folder = match os {
            "windows" => String::from("win32"),
            "macos" => format!("mac64{}", m1),
            _ => String::from("linux64")
        };

        let cache_folder = String::from(CACHE_FOLDER).replace("/", &*String::from(MAIN_SEPARATOR));
        let base_dirs = BaseDirs::new().unwrap();
        let cache = Path::new(base_dirs.home_dir())
            .join(cache_folder)
            .join(self.driver_name)
            .join(arch_folder)
            .join(driver_version);
        unzip(target_path, cache);

        Ok(())
    }

    fn get_m1_prefix(&self, arch: &str) -> &str {
        match arch {
            "aarch64" => "_m1",
            _ => "",
        }
    }
}

#[tokio::main]
async fn download_file(url: String) -> Result<(TempDir, String), Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix("selenium-manager").tempdir()?;
    log::debug!("Downloading {} to temporal folder {:?}", url, tmp_dir.path());

    let response = reqwest::get(url).await?;
    let target_path;
    let mut tmp_file = {
        let target_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        log::trace!("File to be downloaded: {}", target_name);
        let target_name = tmp_dir.path().join(target_name);
        target_path = String::from(target_name.to_str().unwrap());

        log::trace!("Temporal folder for driver package: {}", target_path);
        File::create(target_name)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut tmp_file)?;

    Ok((tmp_dir, target_path))
}

fn unzip(zip_file: String, target: PathBuf) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => target.join(path.to_owned()),
            None => continue,
        };
        if (file.name()).ends_with('/') {
            fs::create_dir_all(&out_path).unwrap();
        } else {
            log::debug!("File extracted to {} ({} bytes)", out_path.display(), file.size());
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&out_path).unwrap();

            log::info!("{}", out_path.display());
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}