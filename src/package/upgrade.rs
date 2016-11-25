use std::path::Path;

use find_package;
use package::{ installation, uninstallation };
use package_version;

pub fn upgrade_package(name: &str, path: &Path) -> Result<bool, String> {
    let selected_package = find_package::select_package(name, path)?;
    let version = match package_version::get_version(&selected_package.name, path) {
        Some(x) => x,
        None => return Err("The package is not available.".to_string())
    };

    if selected_package.version > version {
        uninstallation::uninstall(name, path)?;
        installation::install_package(name, path)?;
        package_version::update_version(name, selected_package.version, path)?;
        return Ok(true)
    } else {
        return Ok(false)
    }
}