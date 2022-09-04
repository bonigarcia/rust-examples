use std::process::exit;
use clap::{arg, Command};

fn main() {
    let matches = Command::new("Selenium Manager")
        .version("1.0.0")
        .about("Automated driver management for Selenium")
        .arg(arg!(--browser <browserType>).required(false))
        .get_matches();

    if let Some(browser) = matches.get_one::<String>("browser") {
        println!("OK\t/path/to/{}driver", browser);
        exit(0);
    } else {
        eprintln!("ERROR\tNo browser specified");
        exit(1);
    }
}

#[test]
fn test_ok() {
    let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["--browser", "chrome"])
        .assert()
        .success()
        .code(0)
        .stdout("OK\t/path/to/chromedriver\n");
}

#[test]
fn test_err() {
    let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert()
        .failure()
        .code(1)
        .stderr("ERROR\tNo browser specified\n");
}