use std::fs;
use std::path::{ Path, PathBuf };

use find_package;

fn uninstall(name: &str, path: &Path) -> Result<bool, String> {
    let package = find_package::select_package(name, path)?;
    
    let mut installation_path = PathBuf::new();
            installation_path.push(path);
            installation_path.push(format!("install/{}", package.name));

    if installation_path.exists() {
        match fs::remove_dir_all(&path) {
            Ok(x) => x,
            Err(why) => return Err(format!("{}", why))
        }
        return Ok(true)
    } else {
        return Ok(false)
    }
}

fn full_uninstall(name: &str, path: &Path) -> Result<bool, String> {
    let package = find_package::select_package(name, path)?;
    uninstall(name, path)?;

    let mut download_path = PathBuf::new();
            download_path.push(path);
            download_path.push(format!("download/{}", package.name));
    match fs::remove_file(download_path) {
        Ok(x) => x,
        Err(why) => return Err(format!("{}", why))
    };

    let mut tar_path = PathBuf::new();
            tar_path.push(path);
            tar_path.push(format!("download/{}.tar", package.name));
    match fs::remove_file(tar_path) {
        Ok(x) => x,
        Err(why) => return Err(format!("{}", why))
    };


    return Ok(true)
}
