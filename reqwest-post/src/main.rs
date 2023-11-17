use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const PLAUSIBLE_URL: &str = "https://plausible.io/api/event";
const UA_CHROME_WINDOWS: &str = r#"Mozilla/5.0 (Windows; x64) Chrome/{}"#;
const APP_JSON: &str = "application/json";
const SELENIUM_DOMAIN: &str = "bonigarcia.dev"; // TODO change
const SM_STATS_URL: &str = "https://{}/sm-stats";
const PAGE_VIEW: &str = "pageview";

fn main() -> Result<(), Box<dyn Error>> {
    let props = Props {
        browser: "chrome".to_string(),
        browser_version: "119".to_string(),
        os: "win".to_string(),
        arch: "x86".to_string(),
        lang: "java".to_string(),
        selenium_version: "4.16.0".to_string(),
    };
    call_plausible(props)?;
    println!("Ok");
    Ok(())
}

#[tokio::main]
async fn call_plausible(props: Props) -> Result<(), Box<dyn Error>> {
    let client = Client::builder().build()?;
    let user_agent = format_one_arg(UA_CHROME_WINDOWS, &props.browser_version);
    let sm_stats_url = format_one_arg(SM_STATS_URL, SELENIUM_DOMAIN);

    let data = Data {
        name: PAGE_VIEW.to_string(),
        url: sm_stats_url,
        domain: SELENIUM_DOMAIN.to_string(),
        props,
    };

    client
        .post(PLAUSIBLE_URL)
        .header(USER_AGENT, user_agent)
        .header(CONTENT_TYPE, APP_JSON)
        .body(serde_json::to_string_pretty(&data)?)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub fn format_one_arg(string: &str, arg1: &str) -> String {
    string.replacen("{}", arg1, 1)
}

pub fn format_two_args(string: &str, arg1: &str, arg2: &str) -> String {
    string.replacen("{}", arg1, 1).replacen("{}", arg2, 1)
}

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub url: String,
    pub domain: String,
    pub props: Props,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Props {
    pub browser: String,
    pub browser_version: String,
    pub os: String,
    pub arch: String,
    pub lang: String,
    pub selenium_version: String,
}
