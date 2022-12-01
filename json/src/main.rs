use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::time::Duration;
use std::fs::write;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
struct Browser {
    browser_name: String,
    browser_version: String,
    #[serde(with = "time::serde::timestamp")]
    browser_version_ttl: OffsetDateTime,
    driver_name: String,
    driver_version: String,
    #[serde(with = "time::serde::timestamp")]
    driver_version_ttl: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    browsers: Vec<Browser>,
}

const TTL_BROWSERS: u64 = 3600;
const TTL_DRIVERS: u64 = 86400;

fn main() {
    let now = OffsetDateTime::now_utc();

    println!("Now is {}, which is {}", now, now.unix_timestamp());

    let data = r#"
{
  "browsers": [
    {
      "browser_name": "chrome",
      "browser_version": "105",
      "browser_version_ttl": 1664459382,
      "driver_name": "chromedriver",
      "driver_version": "105.0.5195.52",
      "driver_version_ttl": 1664459382
    },
    {
      "browser_name": "firefox",
      "browser_version": "104",
      "browser_version_ttl": 1664459382,
      "driver_name": "geckodriver",
      "driver_version": "0.31.0",
      "driver_version_ttl": 1664459382
    }
  ]
}
"#;

    let mut metadata: Metadata = serde_json::from_str(data).unwrap();

    println!("{} {} {}", metadata.browsers[0].browser_name, metadata.browsers[0].browser_version, metadata.browsers[0].browser_version_ttl);
    println!("{} {} {}", metadata.browsers[0].driver_name, metadata.browsers[0].driver_version, metadata.browsers[0].driver_version_ttl);

    metadata.browsers[0].browser_version_ttl = metadata.browsers[0].browser_version_ttl.add(Duration::from_secs(TTL_BROWSERS));
    metadata.browsers[0].driver_version_ttl = metadata.browsers[0].driver_version_ttl.add(Duration::from_secs(TTL_DRIVERS));

    // Write to file
    let filename = "out.json";
    write(&filename, serde_json::to_string_pretty(&metadata).unwrap()).unwrap();

    // Read file
    let mut file = File::open(&filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let metadata2: Metadata = serde_json::from_str(&data).unwrap();

    println!("{} {} {}", metadata2.browsers[0].browser_name, metadata2.browsers[0].browser_version, metadata2.browsers[0].browser_version_ttl);
    println!("{} {} {}", metadata2.browsers[0].driver_name, metadata2.browsers[0].driver_version, metadata2.browsers[0].driver_version_ttl);

    // Metadata #3
    let browser_time = OffsetDateTime::from_unix_timestamp(now.unix_timestamp() + (TTL_BROWSERS as i64)).unwrap();
    let driver_time = OffsetDateTime::from_unix_timestamp(now.unix_timestamp() + (TTL_DRIVERS as i64)).unwrap();

    let mut metadata3 = Metadata {
        browsers: Vec::new()
    };
    let chrome: Browser = Browser {
        browser_name: "chrome".to_string(),
        browser_version: "107".to_string(),
        browser_version_ttl: browser_time,
        driver_name: "chromedriver".to_string(),
        driver_version: "107.xxxx".to_string(),
        driver_version_ttl: driver_time
    };
    metadata3.browsers.push(chrome);

    let filename2 = "out2.json";
    write(&filename2, serde_json::to_string_pretty(&metadata3).unwrap()).unwrap();

    println!("{} {} {}", metadata3.browsers[0].browser_name, metadata3.browsers[0].browser_version, metadata3.browsers[0].browser_version_ttl);
    println!("{} {} {}", metadata3.browsers[0].driver_name, metadata3.browsers[0].driver_version, metadata3.browsers[0].driver_version_ttl);

}