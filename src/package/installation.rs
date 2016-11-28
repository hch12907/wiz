use std::fs;
use std::fs::File;
use std::path::{ Path, PathBuf };

use download;
use extract;
use verify;
use find_package;
use find_package::Package;

fn _install_package(package: &Package, path: &Path) -> Result<bool, String> {
    let mut download_path = PathBuf::new();
        download_path.push(path);
        download_path.push(format!("download/{}", package.name));

    if !download_path.exists() {
        download::download_file(&package.url, download_path.as_path())?;

        let target = match File::open(download_path.as_path()) {
            Ok(x) => x,
            Err(why) => return Err(format!("{}", why))
        };

        if verify::verify_file_crc32(&target) != package.crc32 {
            fs::remove_file(download_path.as_path())
                .ok()
                .expect(
                    &format!(
                        "Failed to remove file: {}", 
                        download_path.as_path().to_str().unwrap_or("Invalid File")
                    )
                );
            return Err(String::from("The downloaded file is corrupted.\nwiz will delete the file now."))
        }
    }

    let mut tar_path = PathBuf::new();
        tar_path.push(path);
        tar_path.push(format!("download/{}.tar", package.name));
    
    if !tar_path.exists() {
        extract::extract_gz(download_path.as_path(), tar_path.as_path())?;
    }

    let mut install_path = PathBuf::new();
        tar_path.push(path);
        tar_path.push(format!("install/{}", package.name));

    if !install_path.exists() {
        extract::extract_tar(tar_path.as_path(), install_path.as_path())?;
    } else {
        return Ok(false)
    }

    return Ok(true)
}

pub fn install_package(name: &str, path: &Path) -> Result<bool, String> {
    let package = find_package::select_package(name, path)?;
    for dep in &package.dependencies {
        install_package(&dep.name, path)?;
    }
    return _install_package(&package, path)
}
