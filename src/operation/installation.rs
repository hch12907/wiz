use std::error::Error;
use std::fs;
use std::io::stdin;
use std::path::Path;

use backend::download;
use backend::extract;
use operation::selection::select_package;
use package::{ find, version };
use package::pkg::Package;
use paths::{ self, PathAppend };  

fn install_package(package: &Package, base_path: &Path) -> Result<(), String> {
    let mut base_pathbuf = base_path.to_path_buf();
    let tarball_path = base_pathbuf.append(&(paths::DOWNLOAD_DIR.to_string() + get_option!(package.url.split('/').last(), "")));
    let gz_path = base_pathbuf.append(&(paths::DOWNLOAD_DIR.to_string() + &package.name + "/"));
    let install_path = base_pathbuf.append(&(paths::PACKAGE_INSTALL_DIR.to_string() + &package.name + "/"));
    
    try!(download::download_file_while(
            &package.url, 
            &tarball_path, 
            |all, current|{
                println!("Downloaded {} bytes, out of {} bytes.", current, all);
            }
        )
    );

    try!(extract::extract_tar(&tarball_path, &gz_path));
    try!(extract::extract_gz(&gz_path, &install_path));
    try!(version::update_list(result, &base_path.to_path_buf().append(paths::INSTALLED_PACKAGE_LIST)));
    
    Ok(())
}

pub fn install(input: &str, base_path: &Path) -> Result<(), String> {
    let package_list_dir = get!(fs::read_dir(base_path.to_path_buf().append(paths::PACKAGE_LIST_DIR)), "");
    for list_path in package_list_dir {
        let result = find::select_package(input, list_path.unwrap().path().as_path(), select_package); 
        if result.is_ok() {
            let result = result.unwrap();
            for dep in &result.dependencies {
                try!(install_package(&dep, base_path));
            }
            return install_package(&result, base_path)
        }
    }
    Err("Specified package not found".to_string())
}