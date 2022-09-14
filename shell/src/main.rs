use std::env::consts::OS;
use std::process::Command;

fn main() {
    let command = match OS {
        "windows" => "cmd",
        _ => "sh"
    };
    let args = match OS {
        "windows" => ["/C", r#"wmic datafile where name='%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe' get Version /value"#],
        "mac" => ["-c", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome --version"],
        _ => ["-c", "google-chrome --version"],
    };

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("command failed to start");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}