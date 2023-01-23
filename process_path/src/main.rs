use regex::Regex;
use std::env::consts::OS;
use std::process::Command;

const DRIVER_VERSION_COMMAND: &str = "chromedriver --version";
//const DRIVER_VERSION_COMMAND: &str = "geckodriver --version";
//const DRIVER_VERSION_COMMAND: &str = "msedgedriver --version";
//const DRIVER_VERSION_COMMAND: &str = "IEDriverServer --version";

const DRIVER_PATH_COMMAND: &str = "where chromedriver";
//const DRIVER_PATH_COMMAND: &str = "which chromedriver";

fn run_command(command: &str, flag: &str, args: String) -> String {
    let output = Command::new(command)
        .args([flag, args.as_str()])
        .output()
        .expect("command failed to start");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn parse_version(string: String) -> String {
    let re_versions = Regex::new(r"(?:(\d+)\.)?(?:(\d+)\.)?(?:(\d+)\.\d+)").unwrap();
    for token in string.split(" ") {
        if re_versions.is_match(token) {
            return token.to_owned();
        }
    }
    "".to_string()
}

fn main() {
    let command = match OS {
        "windows" => "cmd",
        _ => "sh",
    };
    let flag = match OS {
        "windows" => "/C",
        _ => "-c",
    };

    let stdout_1 = run_command(command, flag, DRIVER_VERSION_COMMAND.to_string());
    let driver_version = parse_version(stdout_1);
    println!("driver_version: {}", driver_version);

    let driver_path = run_command(command, flag, DRIVER_PATH_COMMAND.to_string());
    println!("driver_path: {}", driver_path);
}
