use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct BrowserPath {
    os: String,
    channel: String,
}

impl BrowserPath {
    fn new(os: &str, channel: &str) -> BrowserPath {
        BrowserPath {
            os: os.to_string(),
            channel: channel.to_string(),
        }
    }
}

fn main() {
    let chrome_map = HashMap::from([
        (BrowserPath::new("Linux", "stable"), "google-chrome"),
        (BrowserPath::new("Linux", "beta"), "google-chrome-beta"),
        (BrowserPath::new("Linux", "dev"), "google-chrome-unstable"),
    ]);

    for (chrome_distro, path) in &chrome_map {
        println!("{chrome_distro:?} is in {path}");
    }

    match chrome_map.get(&BrowserPath::new("Linux", "stable")) {
        Some(p) => println!("Chrome stable in linux is in {p}"),
        _ => println!("Chrome stable in linux is not supported"),
    }

    match chrome_map.get(&BrowserPath::new("Linux", "unstable")) {
        Some(p) => println!("Chrome unstable in linux is in {p}"),
        _ => println!("Chrome unstable in linux is not supported"),
    }
}
