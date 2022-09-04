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