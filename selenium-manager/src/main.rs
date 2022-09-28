use std::env::consts::{ARCH, OS};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;
use std::process::Command;

use clap::Parser;
use directories::BaseDirs;
use env_logger::fmt::Color;
use env_logger::Target::Stdout;
use log::Level;
use log::LevelFilter::{Debug, Info, Trace};
use regex::Regex;
use tempfile::{Builder, TempDir};
use zip::ZipArchive;

const CHROMEDRIVER: &str = "chromedriver";
const CHROMEDRIVER_URL: &str = "https://chromedriver.storage.googleapis.com/";
const CACHE_FOLDER: &str = ".cache/selenium";

/// Selenium-Manager prototype
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

    if browser_type.eq("chrome") {
        let mut browser_version = cli.version;
        if browser_version.is_empty() {
            browser_version = get_browser_version();
            log::debug!("The version of your local {} is {}", browser_type, browser_version);
        }
        let driver_version = get_driver_version(&browser_version)?;
        log::debug!("You need to use chromedriver {} for controlling Chrome {} with Selenium", driver_version, browser_version);

        download_driver(&CHROMEDRIVER, &driver_version, &os, &arch)?;
        Ok(())
    } else {
        log::error!("{} is not unknown", browser_type);
        Err("Browser not supported")?
    }
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
    log::debug!("Running shell command: {:?}", args);

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("command failed to start");
    log::debug!("{:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"[^\d^.]").unwrap();

    let browser_version = re.replace_all(&*stdout, "").to_string();
    log::debug!("Your browser version is {}", browser_version);

    let browser_version_vec: Vec<&str> = browser_version.split(".").collect();
    browser_version_vec.get(0).unwrap().to_string()
}


fn get_m1_prefix(arch: &str) -> &str {
    match arch {
        "aarch64" => "_m1",
        _ => "",
    }
}

fn get_driver_url(driver_name: &str, driver_version: &String, os: &str, arch: &str) -> String {
    let m1 = get_m1_prefix(&arch);
    match os {
        "windows" => format!("{}{}/{}_win32.zip", CHROMEDRIVER_URL, driver_version, driver_name),
        "macos" => format!("{}{}/{}_mac64{}.zip", CHROMEDRIVER_URL, driver_version, driver_name, m1),
        _ => format!("{}{}/{}_linux64.zip", CHROMEDRIVER_URL, driver_version, driver_name),
    }
}

#[tokio::main]
async fn download_file(url: String) -> Result<(TempDir, String), Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix("selenium-manager").tempdir()?;
    log::debug!("Downloading {} to temporal folder {:?}", url, tmp_dir.path());

    let response = reqwest::get(url).await?;
    let target_path;
    let mut tmp_file = {
        let target_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        log::trace!("File to be downloaded: {}", target_name);
        let target_name = tmp_dir.path().join(target_name);
        target_path = String::from(target_name.to_str().unwrap());

        log::trace!("Temporal folder for driver package: {}", target_path);
        File::create(target_name)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut tmp_file)?;

    Ok((tmp_dir, target_path))
}

fn download_driver(driver_name: &str, driver_version: &String, os: &str, arch: &str) -> Result<(), Box<dyn Error>> {
    let url = get_driver_url(&driver_name, &driver_version, os, arch);
    let (_tmp_dir, target_path) = download_file(url)?;

    let m1 = get_m1_prefix(&arch);
    let arch_folder = match os {
        "windows" => String::from("win32"),
        "macos" => format!("mac64{}", m1),
        _ => String::from("linux64")
    };

    let cache_folder = String::from(CACHE_FOLDER).replace("/", &*String::from(MAIN_SEPARATOR));
    let base_dirs = BaseDirs::new().unwrap();
    let cache = Path::new(base_dirs.home_dir())
        .join(cache_folder)
        .join(driver_name)
        .join(arch_folder)
        .join(driver_version);
    unzip(target_path, cache);

    Ok(())
}

fn unzip(zip_file: String, target: PathBuf) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => target.join(path.to_owned()),
            None => continue,
        };
        if (file.name()).ends_with('/') {
            fs::create_dir_all(&out_path).unwrap();
        } else {
            log::debug!("File extracted to {} ({} bytes)", out_path.display(), file.size());
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&out_path).unwrap();

            log::info!("{}", out_path.display());
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}

#[tokio::main]
async fn get_driver_version(browser_version: &String) -> Result<String, Box<dyn Error>> {
    let driver_url = format!("{}LATEST_RELEASE_{}", CHROMEDRIVER_URL, browser_version);
    let driver_version = reqwest::get(driver_url).await?.text().await?;

    Ok(driver_version)
}