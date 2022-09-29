use std::ops::Add;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
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
struct Browsers {
    browsers: Vec<Browser>,
}

const TTL_BROWSERS: u64 = 3600;
const TTL_DRIVERS: u64 = 86400;

fn main() -> Result<()> {
    let now = OffsetDateTime::now_utc();

    println!("Now is {}, which is {}", now.to_string(), now.unix_timestamp());

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

    let mut metadata: Browsers = serde_json::from_str(data)?;

    println!("{} {} {}", metadata.browsers[0].browser_name, metadata.browsers[0].browser_version, metadata.browsers[0].browser_version_ttl);
    println!("{} {} {}", metadata.browsers[0].driver_name, metadata.browsers[0].driver_version, metadata.browsers[0].driver_version_ttl);

    metadata.browsers[0].browser_version_ttl = metadata.browsers[0].browser_version_ttl.add(Duration::from_secs(TTL_BROWSERS));
    metadata.browsers[0].driver_version_ttl = metadata.browsers[0].driver_version_ttl.add(Duration::from_secs(TTL_DRIVERS));

    println!("{} {} {}", metadata.browsers[0].browser_name, metadata.browsers[0].browser_version, metadata.browsers[0].browser_version_ttl);
    println!("{} {} {}", metadata.browsers[0].driver_name, metadata.browsers[0].driver_version, metadata.browsers[0].driver_version_ttl);

    std::fs::write("out.json", serde_json::to_string_pretty(&metadata).unwrap()).unwrap();

    Ok(())
}