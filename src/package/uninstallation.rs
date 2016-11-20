use std::fs;
use std::path::{ Path, PathBuf };

use find_package;

fn uninstall(name: &str, path: &Path) -> Result<bool, String> {
    let package = find_package::select_package(name, path);
    
    let mut installation_path = PathBuf::new();
            installation_path.push(path);
            installation_path.push(format!("install/{}", package.name));

    if installation_path.exists() {
        fs::remove_dir_all(&path)?;
        return Ok(true)
    } else {
        return Ok(false)
    }
}

fn full_uninstall(name: &str, path: &Path) -> Result<bool, String> {
    uninstall(name, path)?;

    let mut download_path = PathBuf::new();
            download_path.push(path);
            download_path.push(format!("download/{}", package.name));
    fs::remove_file(tar_path)?;

    let mut tar_path = PathBuf::new();
            tar_path.push(path);
            tar_path.push(format!("download/{}.tar", package.name));
    fs::remove_file(tar_path)?;

    return Ok(true)
}
