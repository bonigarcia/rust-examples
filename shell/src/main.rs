use std::env::consts::OS;
use std::process::Command;


fn main() {
    let command = match OS {
        "windows" => "cmd",
        _ => "sh"
    };
    let args = match OS {
        "windows" => ["/C", "dir /a"],
        _ => ["-c", "ls -l"]
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