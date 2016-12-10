use std::fs::File;
use std::io::BufReader;

use package::pkg::{ Package, Version };

pub fn get_installed_version(pkg: Package, path: &Path) -> Result<Version, String> {
    let file = get!(File::open(path), "An error occured while opening version list.");
    let file = BufReader::new(file);
    
    for line in file.lines() {
        let line: String = get!(line, "An error occured while trying to unwrap line.");
        if line.contains(&pkg.name) {
            return Ok(pkg.version)
        }
    }

    return Err("Specified package not found")
}

pub fn update_installed_version(pkg: Package, path: &Path, version: Version) -> Result<bool, String> {
    let file = get!(File::open(path), "An error occured while opening version list.");
    let file = BufReader::new(file);
    let result = get_installed_version(pkg, path);

    return match result {
        Ok(version) => Ok(true),
        Ok(x) => {},
        Err("Specified package not found") => {},
        Err(_) => result
    }   
}