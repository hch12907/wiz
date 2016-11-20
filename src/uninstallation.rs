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