use std::env::consts::OS;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::io::Write;
use std::process::Command;

use clap::Parser;
use env_logger::fmt::Color;
use log::Level;
use log::LevelFilter::{Debug, Info, Trace};
use regex::Regex;
use tempfile::Builder;
use zip::ZipArchive;

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

fn main() -> Result<(), Box<dyn Error>> {
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

        download_chromedriver(browser_version)?;
        Ok(())
    } else {
        log::error!("{} is not unknown", browser_type);
        Err("Browser not supported")?
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

#[tokio::main]
async fn download_chromedriver(chrome_version: String) -> Result<(), Box<dyn Error>> {
    log::debug!("Downloading chromedriver for Chrome {}", chrome_version);

    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let url = "https://chromedriver.storage.googleapis.com/106.0.5249.21/chromedriver_linux64.zip";
    //let url = "https://msedgewebdriverstorage.blob.core.windows.net/edgewebdriver/105.0.1343.34/edgedriver_arm64.zip";
    let response = reqwest::get(url).await?;
    let target_path;
    let mut tmp_file = {
        let target_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        log::debug!("File to be downloaded: {}", target_name);
        let target_name = tmp_dir.path().join(target_name);
        target_path = String::from(target_name.to_str().unwrap());

        log::debug!("It will be located under: {}", target_path);
        File::create(target_name)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut tmp_file)?;
    unzip(target_path);
    Ok(())
}

fn unzip(zip_file: String) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (file.name()).ends_with('/') {
            log::trace!("File {} extracted to {}", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            log::trace!("File {} extracted to {} ({} bytes)", i, out_path.display(), file.size());
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&out_path).unwrap();
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