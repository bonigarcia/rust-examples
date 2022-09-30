use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use selenium_manager::get_cache_folder;

const METADATA_FILE: &str = "selenium-manager.json";
const TTL_BROWSERS_SEC: u64 = 3600;
const TTL_DRIVERS_SEC: u64 = 86400;


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


fn get_metadata_path() -> PathBuf {
    get_cache_folder().join(&METADATA_FILE.to_string())
}

fn new_metadata_browser() -> Browser {
    Browser {
        browser_version: "".to_string(),
        browser_version_ttl: 0,
        driver_version: "".to_string(),
        driver_version_ttl: 0,
    }
}

fn now_unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
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