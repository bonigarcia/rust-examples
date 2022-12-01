use assert_cmd::Command;

#[test]
fn test_cli_ok() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--browser", "chrome"])
        .assert()
        .success()
        .code(0)
        .stdout("OK\t/path/to/chromedriver\n");
}

#[test]
fn test_cli_err() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert()
        .failure()
        .code(65)
        .stderr("ERROR\tNo browser specified\n");
}
