use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Browser_path {
    os: String,
    channel: String,
}

impl Browser_path {
    fn new(os: &str, channel: &str) -> Browser_path {
        Browser_path {
            os: os.to_string(),
            channel: channel.to_string(),
        }
    }
}

fn main() {
    let chrome_map = HashMap::from([
        (Browser_path::new("Linux", "stable"), "google-chrome"),
        (Browser_path::new("Linux", "beta"), "google-chrome-beta"),
        (Browser_path::new("Linux", "dev"), "google-chrome-unstable"),
    ]);

    for (chrome_distro, path) in &chrome_map {
        println!("{chrome_distro:?} is in {path}");
    }

    match chrome_map.get(&Browser_path::new("Linux", "stable")) {
        Some(p) => println!("Chrome stable in linux is in {p}"),
        _ => println!("Chrome stable in linux is not supported")
    }

    match chrome_map.get(&Browser_path::new("Linux", "unstable")) {
        Some(p) => println!("Chrome unstable in linux is in {p}"),
        _ => println!("Chrome unstable in linux is not supported")
    }
}
