use std::error::Error;
use std::fs;
use std::path::Path;

use operation::{ installation, selection, uninstallation };
use package::{ find, update, version };
use paths::{ self, PathAppend };

fn upgrade_package(input: &str, base_path: &Path) -> Result<(), String> {
    for list_path in get!(fs::read_dir(base_path.to_path_buf().append(paths::PACKAGE_LIST_DIR)), "") {
        let latest = try!(find::select_package(input, list_path.unwrap().path().as_path(), selection::select_package));
        for pkg in try!(version::get_list(&base_path.to_path_buf().append(paths::INSTALLED_PACKAGE_LIST))) {
            if latest.name == pkg.name && latest.version > pkg.version {
                try!(uninstallation::uninstall_package(&pkg, base_path));
                try!(installation::install_package(&latest, base_path));
                return Ok(())
            }
        }
    }

    return Err("Specified package not installed.".to_string())
}