use std::error::Error;
use std::fs;
use std::path::Path;

use package::{ find, version };
use operation::selection;
use paths::{ self, PathAppend };

fn uninstall_package(input: Package, base_path: &Path) -> Result<bool, String> {
    let installation_path = base_path.to_path_buf().append(&(paths::PACKAGE_INSTALL_DIR.to_string() + &input.name + "/"));
    if installation_path.exists() {
        match fs::remove_dir_all(&installation_path) {
            Ok(x) => x,
            Err(why) => return Err(why.description().to_string())
        }
        return Ok(true)
    }
    return Ok(false);
}

pub fn uninstall(input: &str, base_path: &Path) -> Result<bool, String> {
    let package_list_dir = get!(fs::read_dir(base_path.to_path_buf().append(paths::PACKAGE_LIST_DIR)), "");
    for list_path in package_list_dir {
        let result = find::select_package(input, list_path.unwrap().path().as_path(), select_package); 
        if result.is_ok() {
            let result = result.unwrap();
            return install_package(&result, base_path)
        }
    }
    Err("Specified package not found".to_string())
}