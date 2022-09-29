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

fn main() -> Result<()> {
    let now = OffsetDateTime::now_utc();
    //let dt: DateTime<Utc> = now.clone().into();

    println!("Now is {} ", now.to_string());

    let data = r#"
{
  "browsers": [
    {
      "browser_name": "chrome",
      "browser_version": "105",
      "browser_version_ttl": 1546300800,
      "driver_name": "chromedriver",
      "driver_version": "105.0.5195.52",
      "driver_version_ttl": 1586300800
    },
    {
      "browser_name": "firefox",
      "browser_version": "104",
      "browser_version_ttl": 1546300800,
      "driver_name": "geckodriver",
      "driver_version": "0.31.0",
      "driver_version_ttl": 1586300800
    }
  ]
}
"#;

    let metadata: Browsers = serde_json::from_str(data)?;

    println!("{} {} {}", metadata.browsers[0].browser_name, metadata.browsers[0].browser_version, metadata.browsers[0].browser_version_ttl);
    println!("{} {} {}", metadata.browsers[0].driver_name, metadata.browsers[0].driver_version, metadata.browsers[0].driver_version_ttl);

    Ok(())
}