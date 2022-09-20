use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let chrome_version = "106";
    let chromedriver_version = get_chromedriver_version(chrome_version)?;
    println!("You need to use chromedriver {} for controlling Chrome {} with Selenium", chromedriver_version, chrome_version);
    Ok(())
}


#[tokio::main]
async fn get_chromedriver_version(chrome_version: &str) -> Result<String, Box<dyn Error>> {
    let chromedriver_url = format!("https://chromedriver.storage.googleapis.com/LATEST_RELEASE_{}", chrome_version);
    let chromedriver_version = reqwest::get(chromedriver_url)
        .await?.text().await?;

    Ok(chromedriver_version)
}