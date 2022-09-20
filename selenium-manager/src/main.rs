use std::env::consts::OS;
use std::process::Command;
use regex::Regex;

fn main() {
    let browser_version = get_browser_version();
    println!("Browser version {}", browser_version);
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

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("command failed to start");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"[^\d^\.]").unwrap();

    re.replace_all(&*stdout, "").to_string()
}