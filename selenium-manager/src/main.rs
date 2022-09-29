use std::env::consts::{ARCH, OS};
use std::error::Error;
use std::io::Write;

use clap::Parser;
use env_logger::fmt::Color;
use env_logger::Target::Stdout;
use log::Level;
use log::LevelFilter::{Debug, Info, Trace};

use selenium_manager::BrowserManager;

mod chrome;
use crate::chrome::ChromeManager;

/// Selenium-Manager: Automated driver management for Selenium
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Browser type (e.g., chrome, firefox, edge)
    #[clap(short, long, value_parser)]
    browser: String,

    /// Major browser version (e.g., 105, 106, etc.)
    #[clap(short, long, value_parser, default_value = "")]
    version: String,

    /// Display DEBUG messages
    #[clap(short, long)]
    debug: bool,

    /// Display TRACE messages
    #[clap(short, long)]
    trace: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    setup_logging(&cli);

    let browser_type: String = String::from(cli.browser).to_lowercase();
    let os = OS;
    let arch = ARCH;
    let browser_manager : Box<dyn BrowserManager>;

    if browser_type.eq("chrome") {
        browser_manager = ChromeManager::new();
    } else {
        log::error!("{} is not unknown", browser_type);
        return Err("Browser not supported")?;
    }

    let mut browser_version = cli.version;
    if browser_version.is_empty() {
        browser_version = browser_manager.get_browser_version(&os);
        log::debug!("The version of your local {} is {}", browser_type, browser_version);
    }
    let driver_version = browser_manager.get_driver_version(&browser_version)?;
    log::debug!("You need to use {} {}", browser_manager.get_driver_name(), driver_version);

    browser_manager.download_driver(&driver_version, &os, &arch)?;
    Ok(())

}


fn setup_logging(cli: &Cli) {
    let mut filter = match cli.debug {
        true => Debug,
        false => Info,
    };
    if cli.trace {
        filter = Trace
    }

    env_logger::Builder::new()
        .filter_level(filter)
        .target(Stdout)
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
}