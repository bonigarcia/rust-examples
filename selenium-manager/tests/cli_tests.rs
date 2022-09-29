use std::str;
use assert_cmd::Command;
use rstest::rstest;

#[rstest]
#[case("chrome", "", "")]
#[case("chrome", "105", "105.0.5195.52")]
#[case("chrome", "106", "106.0.5249.61")]
fn cli_test(#[case] browser: String, #[case] browser_version: String, #[case] driver_version: String) {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--browser", &browser, "--version", &browser_version])
        .assert()
        .success()
        .code(0);

    let stdout = &cmd.unwrap().stdout;
    let output = match str::from_utf8(stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", output);

    assert!(output.contains(&browser));
    if !browser_version.is_empty() {
        assert!(output.contains(&driver_version));
    }
}