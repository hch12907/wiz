use rustc_serialize::{ json, Encodable };

use std::error::Error;
use std::fs::File;
use std::io::{ Read, Write };
use std::path::Path;

use package::pkg::{ Package, Version };

///
/// Parse the list from `path` and
/// returns the parsed list.
///
pub fn get_list(path: &Path) -> Result<Vec<Package>, String> {
    let mut buffer = String::new();
    let mut file = get!(File::open(path), "An error occured while opening file");
    
    get!(file.read_to_string(&mut buffer), "An error occured while reading file");
    return Ok(get!(json::decode(&buffer), "An error occured while decoding JSON"))
}

fn raw_update<T: Encodable>(object: &T, path: &Path) -> Result<(), String> {
    let mut file = get!(File::open(path), "An error occured while opening file");
    return match file.write(get!(json::encode(&object), "An error occured while encoding list").as_bytes()) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.description().to_string())
    }
}

///
/// Updates the list. If the version in the list
/// is older than the one in `package`, it will
/// not be updated.
///  
pub fn update_list(package: &Package, path: &Path) -> Result<(), String> {
    let mut list: Vec<Package> = get_list(path)?;
    for x in 0..list.len() {
        if &list[x].name == &package.name {
            list[x].version = package.version.clone();
            return raw_update(&list, path)
        }
    };

    list.push(package.clone());
    return raw_update(&list, path)
}

pub fn remove_from_list(package: &str, path: &Path) -> Result<(), String> {
    let mut list: Vec<Package> = get_list(path)?;
    for x in 0..list.len() {
        if &list[x].name == &package {
            list.remove(x);
        }
    }

    return raw_update(&list, path)
}