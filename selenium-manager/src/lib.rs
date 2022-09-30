use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use directories::BaseDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tempfile::{Builder, TempDir};
use zip::ZipArchive;

const CACHE_FOLDER: &str = ".cache/selenium";
const METADATA_FILE: &str = "selenium-manager.json";
const TTL_BROWSERS_SEC: u64 = 3600;
const TTL_DRIVERS_SEC: u64 = 86400;

pub trait BrowserManager {
    fn get_browser_name(&self) -> &str;

    fn get_browser_version(&self, os: &str) -> Result<String, String>;

    fn get_driver_name(&self) -> &str;

    fn get_driver_version(&self, browser_version: &str) -> Result<String, Box<dyn Error>>;

    fn get_driver_url(&self, driver_version: &str, os: &str, arch: &str) -> String;

    fn get_driver_path_in_cache(&self, driver_version: &str, os: &str, arch: &str) -> PathBuf;

    fn download_driver(&self, driver_version: &str, os: &str, arch: &str) -> Result<(), Box<dyn Error>> {
        let driver_url = Self::get_driver_url(self, driver_version, os, arch);
        let (_tmp_folder, driver_zip_file) = download_to_tmp_folder(driver_url)?;
        let driver_path_in_cache = Self::get_driver_path_in_cache(self, driver_version, os, arch);
        unzip(driver_zip_file, driver_path_in_cache);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Browser {
    pub browser_version: String,
    pub browser_version_ttl: u64,
    pub driver_version: String,
    pub driver_version_ttl: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub chrome: Browser,
}


#[tokio::main]
pub async fn download_to_tmp_folder(url: String) -> Result<(TempDir, String), Box<dyn Error>> {
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

pub fn unzip(zip_file: String, target: PathBuf) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if (file.name()).ends_with('/') {
            fs::create_dir_all(&target).unwrap();
        } else {
            log::debug!("File extracted to {} ({} bytes)", target.display(), file.size());
            if let Some(p) = target.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&target).unwrap();

            // Set permissions in Unix-like systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&target, fs::Permissions::from_mode(mode)).unwrap();
                }
            }

            io::copy(&mut file, &mut outfile).unwrap();

            break;
        }
    }
}

pub fn run_shell_command(command: &str, flag: &str, args: &str) -> Result<String, Box<dyn Error>> {
    log::debug!("Running {} command: {:?}",command, args);
    let output = Command::new(command)
        .args([flag, args])
        .output()?;
    log::debug!("{:?}", output);

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn parse_version(version_text: String) -> String {
    let re = Regex::new(r"[^\d^.]").unwrap();
    re.replace_all(&*version_text, "").to_string()
}

pub fn detect_browser_major_version(browser_name: &str, shell: &str, flag: &str, args: Vec<&str>) -> String {
    for arg in args.iter() {
        let output = match run_shell_command(shell, flag, *arg) {
            Ok(out) => out,
            Err(_e) => continue,
        };
        let browser_version = parse_version(output);
        if browser_version.is_empty() {
            continue;
        }
        log::debug!("Your {} version is {}", browser_name, browser_version);
        let browser_version_vec: Vec<&str> = browser_version.split('.').collect();
        return browser_version_vec.first().unwrap().to_string();
    }
    log::warn!("The version of {} cannot be detected. Trying with latest driver version", browser_name);
    "".to_string()
}

pub fn get_cache_folder() -> PathBuf {
    Path::new(BaseDirs::new().unwrap().home_dir())
        .join(String::from(CACHE_FOLDER).replace('/', &MAIN_SEPARATOR.to_string()))
}

pub fn create_driver_path(driver_name: &str, os: &str, arch_folder: &str, driver_version: &str) -> PathBuf {
    get_cache_folder()
        .join(driver_name)
        .join(arch_folder)
        .join(driver_version)
        .join(get_driver_filename(driver_name, os))
}

pub fn get_driver_filename(driver_name: &str, os: &str) -> String {
    format!("{}{}", driver_name, get_binary_extension(&os))
}

pub fn get_binary_extension(os: &str) -> &str {
    match os {
        "windows" => ".exe",
        _ => "",
    }
}

pub fn get_metadata_path() -> PathBuf {
    get_cache_folder().join(&METADATA_FILE.to_string())
}

pub fn new_metadata_browser() -> Browser {
    Browser {
        browser_version: "".to_string(),
        browser_version_ttl: 0,
        driver_version: "".to_string(),
        driver_version_ttl: 0,
    }
}

pub fn get_metadata() -> Metadata {
    let metadata_path = get_cache_folder().join(&METADATA_FILE.to_string());
    log::trace!("Reading metadata from {}", metadata_path.display());

    if metadata_path.exists() {
        let metadata_file = File::open(metadata_path).unwrap();
        serde_json::from_reader(metadata_file).unwrap()
    } else {
        log::debug!("Metadata does not exist. Creating a new metadata file");

        Metadata {
            chrome: new_metadata_browser()
        }
    }
}

pub fn now_unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn get_driver_ttl() -> u64 {
    now_unix_timestamp() + TTL_DRIVERS_SEC
}

pub fn get_browser_ttl() -> u64 {
    now_unix_timestamp() + TTL_BROWSERS_SEC
}

pub fn write_metadata(metadata: &Metadata) {
    let metadata_path = get_metadata_path();
    log::debug!("Writing metadata to {}", metadata_path.display());
    fs::write(metadata_path, serde_json::to_string_pretty(metadata).unwrap()).unwrap();
}

pub fn is_ttl_valid(ttl: u64) -> bool {
    now_unix_timestamp() < ttl
}