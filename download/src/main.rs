use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::Cursor;

use tempfile::Builder;
use zip::ZipArchive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let url = "https://chromedriver.storage.googleapis.com/106.0.5249.21/chromedriver_linux64.zip";
    //let url = "https://msedgewebdriverstorage.blob.core.windows.net/edgewebdriver/105.0.1343.34/edgedriver_arm64.zip";
    let response = reqwest::get(url).await?;
    let target_path;
    let mut tmp_file = {
        let target_name = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File to download: {}", target_name);
        let target_name = tmp_dir.path().join(target_name);
        target_path = String::from(target_name.to_str().unwrap());

        println!("It will be located under: {}", target_path);
        File::create(target_name)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut tmp_file)?;
    unzip(target_path);
    Ok(())
}

fn unzip(zip_file: String) {
    let file = File::open(zip_file).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if (file.name()).ends_with('/') {
            println!("File {} extracted to {}", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} extracted to {} ({} bytes)",
                i,
                out_path.display(),
                file.size()
            );
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&out_path).unwrap();
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
