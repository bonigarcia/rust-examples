use directories::BaseDirs;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static CACHE_FOLDER: &str = ".cache/selenium";

fn main() {
    if let Some(base_dirs) = BaseDirs::new() {
        let home = base_dirs.home_dir();
        println!("Your home folder is {}", home.display());

        let cache = base_dirs.cache_dir();
        println!("Your cache folder is {}", cache.display());

        create_file(home, "lorem_ipsum.txt", "Lorem ipsum dolor sit amet.");
    }
}

fn create_file(parent: &Path, filename: &str, content: &str) {
    let cache = Path::new(parent).join(CACHE_FOLDER);
    fs::create_dir_all(&cache).unwrap();

    let path = cache.join(filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("ERROR\tcouldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `content` string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("ERROR\tcouldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
