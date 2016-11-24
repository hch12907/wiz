use std::fs::{ File, OpenOptions };
use std::io::{ BufRead, BufReader, Write };
use std::path::Path;

fn parse_list(path: &Path) -> Result<Vec<(String, u32)>, String> {
    let list = match File::open(path) {
        Ok(x) => x,
        Err(why) => return Err(format!("{}", why))
    };
    let list = BufReader::new(list);
    
    let mut parsed_packages = Vec::new(); 
    for line in list.lines() {
        let line = line.unwrap_or("".to_string());
        let split: Vec<&str> = line.trim().split('=').collect();
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

pub fn update_version(name: &str, version: u32, path: &Path) -> Result<bool, String>{
    let mut list = match parse_list(path) {
        Ok(x) => x,
        Err(why) => return Err(format!("{}", why))
    };

    list.retain(|ref x| x.0 == name);

    if list.len() == 0 {
        let mut file = match OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open("installed_packages") {
            Ok(x) => x,
            Err(why) => return Err(format!("{}", why))
        };
        
        writeln!(file, "{}={}\n", name, version);
        return Ok(true);
    } else if list.len() == 1 {
        if list[0].1 == version {
            return Ok(false)
        } else {
            let mut updated_list = String::new();
            {
                let file = match File::open("installed_packages") {
                    Ok(x) => x,
                    Err(why) => return Err(format!("{}", why))
                };

                let file = BufReader::new(file);
                for line in file.lines() {
                    let current_line = line.unwrap();
                    if current_line.contains(name) {
                        updated_list += name;
                        updated_list += "=";
                        updated_list += &(version.to_string());
                        updated_list += "\n"; 
                    } else {
                        updated_list += current_line.trim();
                    }
                }
            }

            {
                let mut file = match File::open("installed_packages") {
                    Ok(x) => x,
                    Err(why) => return Err(format!("{}", why))
                };

                file.write_all(updated_list.as_bytes());
            }
            return Ok(true);
        }
    } else {
        return Err("There are two or more packages with the same name, this shouldn't happen!".to_string());
    }
}
