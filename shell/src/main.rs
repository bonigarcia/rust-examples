use std::env::consts::OS;
use std::process::Command;
use regex::Regex;

fn main() {
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
    let browser_version = re.replace_all(&*stdout, "");

    println!("status: {}", output.status);
    println!("stdout: {}", stdout);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("browser_version: {}", browser_version);

    assert!(output.status.success());
}