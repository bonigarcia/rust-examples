fn main() {
    println!("Hello, world!");
}

#[test]
fn test_hello() {
    let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert().success().code(0).stdout("Hello, world!\n");
}