use std::str;
use assert_cmd::Command;
use rstest::rstest;

#[rstest]
#[case("105", "105.0.5195.52")]
#[case("106", "106.0.5249.21")]
fn test_browser_manager(#[case] browser_version: String, #[case] driver_version: String) {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--browser", "chrome", "--version", &browser_version])
        .assert()
        .success()
        .code(0);

    let stdout = &cmd.unwrap().stdout;
    let output = match str::from_utf8(stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", output);

    assert!(output.contains(&driver_version));
}