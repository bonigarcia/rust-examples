use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;

use directories::BaseDirs;
use regex::Regex;
use zip::ZipArchive;

const CACHE_FOLDER: &str = ".cache/selenium";

pub fn create_path_if_not_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
}

pub fn unzip(zip_file: String, target: PathBuf) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if (file.name()).ends_with('/') {
            continue;
        } else {
            log::debug!("File extracted to {} ({} bytes)", target.display(), file.size());
            if let Some(p) = target.parent() {
                create_path_if_not_exists(p);
            }

            let mut outfile = File::create(&target).unwrap();

            // Set permissions in Unix-like systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&target, fs::Permissions::from_mode(mode)).unwrap();
                }
            }

            io::copy(&mut file, &mut outfile).unwrap();

            break;
        }
    }
}

pub fn get_cache_folder() -> PathBuf {
    let cache_path = Path::new(BaseDirs::new().unwrap().home_dir())
        .join(String::from(CACHE_FOLDER).replace('/', &MAIN_SEPARATOR.to_string()));
    create_path_if_not_exists(&cache_path);
    cache_path
}

pub fn compose_driver_path_in_cache(driver_name: &str, os: &str, arch_folder: &str, driver_version: &str) -> PathBuf {
    get_cache_folder()
        .join(driver_name)
        .join(arch_folder)
        .join(driver_version)
        .join(get_driver_filename(driver_name, os))
}

pub fn get_driver_filename(driver_name: &str, os: &str) -> String {
    format!("{}{}", driver_name, get_binary_extension(os))
}

pub fn get_binary_extension(os: &str) -> &str {
    match os {
        "windows" => ".exe",
        _ => "",
    }
}

pub fn parse_version(version_text: String) -> String {
    let re = Regex::new(r"[^\d^.]").unwrap();
    re.replace_all(&*version_text, "").to_string()
}