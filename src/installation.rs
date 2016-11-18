use std::fs::File;
use std::path::{ Path, PathBuf };

use download;
use extract;
use verify;
use find_package::Package;

fn _install_package(package: Package, path: &Path) -> Result<bool, String> {
    let download_path = PathBuf::new();
        download_path.push(path);
        download_path.push(format!("download/{}", package.name));

    download::download_file(package.url, download_path);

    let tar_path = PathBuf::new();
        tar_path.push(path);
        tar_path.push(format!("download/{}.tar", package.name));

    extract::extract_gz(download_path, tar_path);

    let install_path = PathBuf::new();
        tar_path.push(path);
        tar_path.push(format!("install/{}", package.name));

    extract::extract_tar(tar_path, install_path);
}

pub fn install_package(name: &str, path: &Path) -> Result<bool, String> {
    let found_packages = try!(find_package::find_package(name, path));
    
    if(!found_packages.is_empty() && found_packages.count() > 1) {
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
        return _install_package(found_packages[0], path)
    }
}
