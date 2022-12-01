use std::fs;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const METADATA_FILE: &str = "sm-metadata-v4.json";
//const TTL_BROWSERS_SEC: u64 = 3600;
//const TTL_DRIVERS_SEC: u64 = 86400;

#[derive(Serialize, Deserialize)]
pub struct Browser {
    pub name: String,
    pub version: String,
    pub ttl: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Driver {
    pub name: String,
    pub browser_version: String,
    pub driver_version: String,
    pub ttl: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub browsers: Vec<Browser>,
    pub drivers: Vec<Driver>,
}

fn main() {
    let now = now_unix_timestamp();
    println!("now {}", now);
    let metadata_file = File::open(METADATA_FILE).unwrap();
    let mut metadata: Metadata = serde_json::from_reader(metadata_file).unwrap();

    let chromedriver: Vec<Driver> = metadata
        .drivers
        .into_iter()
        .filter(|d| d.name.eq("chromedriver") && d.browser_version.eq("105") && d.ttl > now)
        .collect();

    if !chromedriver.is_empty() {
        println!(
            "--> chromedriver version {}",
            chromedriver.get(0).unwrap().driver_version
        );
    }

    let chrome: Vec<Browser> = metadata
        .browsers
        .into_iter()
        .filter(|b| b.name.eq("chrome") && b.ttl > now)
        .collect();

    if !chrome.is_empty() {
        println!("--> chrome version {}", chrome.get(0).unwrap().version);
    }

    metadata.drivers = chromedriver;
    metadata.browsers = chrome;

    fs::write(
        METADATA_FILE,
        serde_json::to_string_pretty(&metadata).unwrap(),
    )
    .unwrap();
}

fn now_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
