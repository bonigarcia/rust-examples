use std::env::consts::OS;
use std::os::windows::process::CommandExt;
use std::process::Command;

fn main() {
    let command = match OS {
        "windows" => "cmd",
        _ => "sh"
    };
    let args = match OS {
        "windows" => r#"cmd.exe /C wmic datafile where name="%PROGRAMFILES:\=\\%\\Google\\Chrome\\Application\\chrome.exe" get Version /value"#,
        "mac" => "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome --version",
        _ => "google-chrome --version",
    };

    let output = Command::new(command)
        .raw_arg(args)
        .output()
        .expect("command failed to start");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}