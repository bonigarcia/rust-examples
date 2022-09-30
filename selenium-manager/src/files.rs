use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::path::MAIN_SEPARATOR;

use directories::BaseDirs;
use tempfile::{Builder, TempDir};
use zip::ZipArchive;

const CACHE_FOLDER: &str = ".cache/selenium";

#[tokio::main]
pub async fn download_driver_to_tmp_folder(url: String) -> Result<(TempDir, String), Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix("selenium-manager").tempdir()?;
    log::trace!("Downloading {} to temporal folder {:?}", url, tmp_dir.path());

    let response = reqwest::get(url).await?;
    let target_path;
    let mut tmp_file = {
        let target_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        log::trace!("File to be downloaded: {}", target_name);
        let target_name = tmp_dir.path().join(target_name);
        target_path = String::from(target_name.to_str().unwrap());

        log::trace!("Temporal folder for driver package: {}", target_path);
        File::create(target_name)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut tmp_file)?;

    Ok((tmp_dir, target_path))
}

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
            fs::create_dir_all(&target).unwrap();
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