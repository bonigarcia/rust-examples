use std::env::consts::OS;
use std::io::Write;
use std::process::Command;

use regex::Regex;

use clap::Parser;
use env_logger::fmt::Color;
use log::Level;
use log::LevelFilter::{Debug, Info, Trace};

/// Selenium-Manager prototype
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Browser for resolving its driver (e.g., chrome, firefox, edge)
    #[clap(short, long, value_parser)]
    browser: String,

    /// Display DEBUG messages
    #[clap(short, long)]
    debug: bool,

    /// Display TRACE messages
    #[clap(short, long)]
    trace: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut filter = match cli.debug {
        true => Debug,
        false => Info,
    };
    if cli.trace {
        filter = Trace
    }

    env_logger::Builder::new()
        .filter_level(filter)
        .format(|buf, record| {
            let mut level_style = buf.style();
            match record.level() {
                Level::Trace => level_style.set_color(Color::Cyan),
                Level::Debug => level_style.set_color(Color::Blue),
                Level::Info => level_style.set_color(Color::Green),
                Level::Warn => level_style.set_color(Color::Yellow),
                Level::Error => level_style.set_color(Color::Red).set_bold(true),
            };
            writeln!(
                buf,
                "{}\t{}",
                level_style.value(record.level()),
                record.args()
            )
        })
        .init();

    let browser_type: String = String::from(cli.browser).to_lowercase();
    if browser_type.eq("chrome") {
        let browser_version = get_browser_version();
        log::info!("The version of your {} is {}", browser_type, browser_version);
    } else {
        log::error!("{} is not supported", browser_type);
    }
}


fn get_browser_version() -> String {
    let command = match OS {
        "windows" => "cmd",
        _ => "sh"
    };
    let args = match OS {
        "windows" => ["/C", r#"wmic datafile where name='%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#],
        "macos" => ["-c", r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --version"#],
        _ => ["-c", "google-chrome --version"],
    };
    log::trace!("Running shell command: {:?}", args);

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("command failed to start");
    log::trace!("Output: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"[^\d^\.]").unwrap();

    let browser_version = re.replace_all(&*stdout, "").to_string();
    log::debug!("Your browser version is {}", browser_version);

    let browser_version_vec: Vec<&str> = browser_version.split(".").collect();
    browser_version_vec.get(0).unwrap().to_string()
}