use assert_cmd::Command;

#[test]
fn test_cli_hello() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert()
        .success()
        .code(0)
        .stdout("Hello, world!\n");
}