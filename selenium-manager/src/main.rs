use std::env::consts::{ARCH, OS};
use std::error::Error;
use std::io::Write;

use clap::Parser;
use env_logger::fmt::Color;
use env_logger::Target::Stdout;
use log::Level;
use log::LevelFilter::{Debug, Info, Trace};

use selenium_manager::BrowserManager;

use crate::chrome::ChromeManager;

mod chrome;

/// Automated driver management for Selenium
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None, disable_version_flag = true, help_template = "\
{name} {version}
{about-with-newline}
{usage-heading} {usage}
{all-args}")]
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
    let browser_type: String = String::from(cli.browser);
    let os = OS;
    let arch = ARCH;
    let browser_manager: Box<dyn BrowserManager>;

    if browser_type.eq_ignore_ascii_case("chrome") {
        browser_manager = ChromeManager::new();
    } else {
        return Err(format!("Browser {} not supported", browser_type))?;
    }

    let mut browser_version = cli.version;
    if browser_version.is_empty() {
        browser_version = browser_manager.get_browser_version(&os)?;
        log::debug!("The major version of your local {} is {}", browser_type, browser_version);
    }
    let driver_version = browser_manager.get_driver_version(&browser_version)?;
    log::debug!("You need to use {} {}", browser_manager.get_driver_name(), driver_version);

    let mut driver_path = browser_manager.get_driver_path_in_cache(&driver_version, &os, &arch);
    if driver_path.exists() {
        log::debug!("{} {} is already in the cache", browser_manager.get_driver_name(), driver_version);
    } else {
        driver_path = browser_manager.download_driver(&driver_version, &os, &arch)?;
    }
    log::info!("{}", driver_path.display());

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