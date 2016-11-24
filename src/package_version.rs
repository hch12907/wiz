use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

fn parse_list(path: &Path) -> Result<Vec<(String, u32)>, String> {
    let list = match File::open(path) {
        Ok(x) => x,
        Err(why) => return Err(format!("{}", why))
    };
    let list = BufReader::new(list);
    
    let parsed_packages = Vec::new(); 
    for line in list.lines() {
        let split: Vec<&str> = line.unwrap_or(String::from("")).trim().split('=').collect();
        let package_name = String::from(split[0]);
        let package_version = split[1].parse::<u32>().unwrap_or(0); // For this reason, version of packages MUST NOT BE 0

        if !package_name.is_empty() || package_version != 0 {
            parsed_packages.push((package_name, package_version));
        }
    }

    if parsed_packages.len() > 0 {
        return Ok(parsed_packages)
    } else {
        return Err(String::from("No package found."))
    }
}

pub fn get_version(name: &str, path: &Path) -> Option<u32> {
    //TODO:
    //  Gets the version of the package
    let list = match parse_list(path) {
        Ok(x) => x,
        Err(why) => return None
    };

    for package in list {
        if package.0 == name {
            return Some(package.1)
        }
    }

    return None
}

pub fn update_version() -> Result<bool, String>{
    //TODO:
    // Sets/Updates the version of the package
}