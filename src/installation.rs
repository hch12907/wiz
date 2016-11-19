use std::fs::File;
use std::io;
use std::path::{ Path, PathBuf };

use download;
use extract;
use verify;
use find_package;
use find_package::Package;

fn _install_package(package: &Package, path: &Path) -> Result<bool, String> {
    let mut download_path = PathBuf::new();
        download_path.push(path);
        download_path.push(format!("download/{}", package.name));

    if !download_path.exists() {
        download::download_file(&package.url, download_path.as_path())?;
    }

    let mut tar_path = PathBuf::new();
        tar_path.push(path);
        tar_path.push(format!("download/{}.tar", package.name));
    
    if !tar_path.exists() {
        extract::extract_gz(download_path.as_path(), tar_path.as_path())?;
    }

    let mut install_path = PathBuf::new();
        tar_path.push(path);
        tar_path.push(format!("install/{}", package.name));

    if !install_path.exists() {
        extract::extract_tar(tar_path.as_path(), install_path.as_path())?;
    } else {
        return Ok(false)
    }

    return Ok(true)
}

pub fn install_package(name: &str, path: &Path) -> Result<bool, String> {
    let found_packages = find_package::find_package(name, path)?;
    
    if !found_packages.is_empty() && found_packages.len() > 1 {
        for (i, package) in found_packages.iter().enumerate() {
            let mut install_path = PathBuf::new();
                install_path.push(path);
                install_path.push(format!("installed/{}", package.name));

            if !install_path.exists() {
                println!("Package {}: {}", i, package.name);
            }
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input: u16 = input.trim().parse()?;
        
        if input > found_packages.len() as u16 {
            return Err(String::from("Invalid input"))
        } else {
            return _install_package(&found_packages[input as usize], path)
        }        

        return _install_package(&found_packages[0], path)
    } else {
        return _install_package(&found_packages[0], path)
    }
}
