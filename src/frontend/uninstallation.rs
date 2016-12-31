use std::error::Error;
use std::fs;
use std::path::Path;

use package::{ find, version, pkg };
use package::pkg::{ Package, Version };
use operation::selection;
use paths::{ self, PathAppend };

pub fn uninstall_package(input: &Package, base_path: &Path) -> Result<bool, String> {
    let installation_path = base_path.to_path_buf().append(&(paths::PACKAGE_INSTALL_DIR.to_string() + &input.name + "/"));
    if installation_path.exists() {
        match fs::remove_dir_all(&installation_path) {
            Ok(x) => x,
            Err(why) => return Err(why.description().to_string())
        }
        version::remove_from_list(&input.name, &base_path.to_path_buf().append(paths::INSTALLED_PACKAGE_LIST))?;
        return Ok(true)
    }
    return Ok(false)
}

pub fn uninstall(input: &str, base_path: &Path) -> Result<bool, String> {
    let installed_package = try!(version::get_list(&base_path.to_path_buf().append(paths::INSTALLED_PACKAGE_LIST)));
    for pkg in installed_package {
        if &pkg.name == &input {
            return uninstall_package(&pkg, base_path)
        }
    }
    Err("Specified package not installed".to_string())
}