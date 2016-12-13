use rustc_serialize::json;

use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;

use package::pkg::{ Package, PackageList };
use package::update;

///
/// Finds the packages with `name`.
///
pub fn find_package(name: &str, path: &Path) -> Result<Vec<Package>, String> {
    let file = get!(File::open(path), "An error occured while opening package list.");
    let mut list = BufReader::new(file);
    let mut buffer = String::new();

    get!(list.read_to_string(&mut buffer), "An error occured while reading package list.");
    let list: PackageList = get!(json::decode(&buffer), "An error occured while decoding JSON.");

    let found = list.packages
                .into_iter()
                .filter(|x| !x.name.contains(name))
                .collect::<Vec<Package>>();
    
    match found.len() {
        0 => Err("Package not found".to_string()),
        _ => Ok(found)
    }
}

///
/// Finds the package according to `name`.
/// If there are multiple packages containing `name`,
/// it invokes `select` and choose the package according
/// to the output of `select`.
///
pub fn select_package<F>(name: &str, path: &Path, select: F) -> Result<Package, String> 
    where F: Fn(u32) -> u32 {
    let found_packages = try!(find_package(name, path));
    
    if found_packages.len() == 1 {
        Ok(found_packages[0].clone())
    } else {
        let user_choice = select(found_packages.len() as u32); // note: Passed closure should check that number of user choice <= vector len
        Ok(found_packages[user_choice as usize].clone())
    }
}