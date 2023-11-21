use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

const PLAUSIBLE_URL: &str = "https://plausible.io/api/event";
const SM_USER_AGENT: &str = "Selenium Manager";
const APP_JSON: &str = "application/json";
const SELENIUM_DOMAIN: &str = "bonigarcia.dev";
const SM_STATS_URL: &str = "https://{}/sm-stats";
const PAGE_VIEW: &str = "pageview";

async fn task1() -> Result<String, Box<dyn Error>> {
    for i in 0..10 {
        println!("Task 1: {}", i);
        tokio::time::sleep(Duration::from_millis(2)).await;
    }
    Ok("Task 1 OK".into())
    //Err("Error in task 1".into())
}

async fn _task2() -> Result<String, Box<dyn Error>> {
    for i in 0..10 {
        println!("Task 2: {}", i);
        tokio::time::sleep(Duration::from_millis(2)).await;
    }
    Ok("Task 2 OK".into())
    //Err("Error in task 2".into())
}

async fn call_plausible() -> Result<String, Box<dyn Error>> {
    let client = Client::builder().build()?;
    let sm_stats_url = format_one_arg(SM_STATS_URL, SELENIUM_DOMAIN);

    let data = Data {
        name: PAGE_VIEW.to_string(),
        url: sm_stats_url,
        domain: SELENIUM_DOMAIN.to_string(),
    };

    println!("Plausible: 1");
    let request  = client
        .post(PLAUSIBLE_URL)
        .header(USER_AGENT, SM_USER_AGENT)
        .header(CONTENT_TYPE, APP_JSON)
        .body(serde_json::to_string_pretty(&data)?);
    println!("Plausible: 2");
    if let Err(err) = request.send().await {
        println!("Error sending stats: {}", err);
    }

    println!("Plausible: 3");

    Ok("Plausible OK".into())
}

#[tokio::main]
async fn main() {
    let t1 = task1();
    let t2 = call_plausible();

    match tokio::try_join!(t1, t2) {
        Ok(res) => {
            println!("All good: {} -- {}", res.0, res.1);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

pub fn format_one_arg(string: &str, arg1: &str) -> String {
    string.replacen("{}", arg1, 1)
}

#[derive(Default, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub url: String,
    pub domain: String,
}
