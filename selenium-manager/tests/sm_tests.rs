use assert_cmd::Command;
use std::str;

#[test]
fn test_browser_manager() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--browser", "chrome", "--version", "105"])
        .assert()
        .success()
        .code(0);

    let stdout = &cmd.unwrap().stdout;
    let output = match str::from_utf8(stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("--> OUTPUT: {}", output);

    assert!(output.contains("105.0.5195.52"));
}