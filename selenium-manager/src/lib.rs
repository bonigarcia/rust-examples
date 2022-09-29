use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;
use std::path::PathBuf;

use tempfile::{Builder, TempDir};
use zip::ZipArchive;

pub trait BrowserManager {
    fn get_browser_name(&self) -> &str;

    fn get_driver_name(&self) -> &str;

    fn get_browser_version(&self, os: &str) -> String;

    fn get_driver_url(&self, driver_version: &String, os: &str, arch: &str) -> String;

    fn get_driver_version(&self, browser_version: &String) -> Result<String, Box<dyn Error>>;

    fn download_driver(&self, driver_version: &String, os: &str, arch: &str) -> Result<(), Box<dyn Error>>;

    fn get_cache_path(&self, driver_name: &str, driver_version: &String, os: &str, arch: &str) -> PathBuf;
}

pub fn get_m1_prefix(arch: &str) -> &str {
    match arch {
        "aarch64" => "_m1",
        _ => "",
    }
}

#[tokio::main]
pub async fn download_file(url: String) -> Result<(TempDir, String), Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix("selenium-manager").tempdir()?;
    log::debug!("Downloading {} to temporal folder {:?}", url, tmp_dir.path());

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

pub fn unzip(zip_file: String, target: PathBuf) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => target.join(path.to_owned()),
            None => continue,
        };
        if (file.name()).ends_with('/') {
            fs::create_dir_all(&out_path).unwrap();
        } else {
            log::debug!("File extracted to {} ({} bytes)", out_path.display(), file.size());
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&out_path).unwrap();

            log::info!("{}", out_path.display());
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}