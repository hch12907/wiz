use std::fs::File;
use std::path::{ Path, PathBuf };

use download;
use extract;
use verify;
use find_package::Package;

fn _install_package(package: Package, path: &Path) -> Result<bool, String> {
    let install_path = PathBuf::new();
        install_path.push(path);
        install_path.push(format!("installed/{}", package.name));
}

pub fn install_package(name: &str, path: &Path) -> Result<bool, String> {
    let found_packages = try!(find_package::find_package(name, path));
    
    if(found_packages.count() > 1) {
        for (i, package) in found_packages.iter().enumerate() {
            let install_path = PathBuf::new();
                install_path.push(path);
                install_path.push(format!("installed/{}", package.name));

            if(!install_path.exists()) {
                println!("Package {}: {}", i, package);
            }
        }

        //TODO: Accept user input and install user-chosen package.
    } else {
        return _install_package(package)
    }
}
