use std::error::Error;
use std::fs;
use std::path::Path;

use package::find;
use operation::selection;
use paths::{ self, PathAppend };

pub fn uninstall(input: &str, base_path: &Path) -> Result<bool, String> {
    let package_list_dir = get!(fs::read_dir(base_path.to_path_buf().append(paths::PACKAGE_LIST_DIR)), "");
    for list_path in package_list_dir {
        let result = try!(find::select_package(input, list_path.unwrap().path().as_path(), selection::select_package));
        let installation_path = base_path.to_path_buf().append(&(paths::PACKAGE_INSTALL_DIR.to_string() + &result.name + "/"));
        if installation_path.exists() {
            match fs::remove_dir_all(&installation_path) {
                Ok(x) => x,
                Err(why) => return Err(why.description().to_string())
            }
            return Ok(true)
        } else {
            continue;
        }
    }
    return Ok(false);
}
