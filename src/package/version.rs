use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

use package::pkg::{ Package, Version };

const SEPERATION_SYMBOL: &'static str = "=";

pub fn get_installed_version(pkg: Package, path: &Path) -> Result<Version, String> {
    let file = get!(File::open(path), "An error occured while opening version list.");
    let file = BufReader::new(file);
    
    for line in file.lines() {
        let line: String = get!(line, "An error occured while trying to unwrap line.");
        if line.contains(&pkg.name) {
            let ver: Vec<&str> = line.trim().split(SEPERATION_SYMBOL).collect();
            if ver.len() == 2 {
                return Ok(try!(Version::from_str(ver[1])))
            } else {
                return Err("Wrong version".to_string())
            }
        }
    }

    return Err("Specified package not found".to_string())
}

pub fn update_installed_version(pkg: Package, path: &Path, version: Version) -> Result<bool, String> {
    
}