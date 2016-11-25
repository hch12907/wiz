//extern crate serde_json;
use rustc_serialize::json;

use std::ffi::OsStr;
use std::fs::{ self, File };
use std::io::{ self, BufReader, Read };
use std::path::{ Path, PathBuf };

use download;
use verify;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => return Err(format!("An error occured during package finding. {}", why)),
    });
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Package {
    pub name: String,
    pub version: u32,
    pub crc32: u32,
    pub url: String,
    pub dependencies: Vec<Package>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PackageList {
    packages: Vec<Package>,
    version: i32
}


pub fn update_package_list(url: &str, path: &Path) -> Result<bool, String> {
    if (path.extension().is_some() && 
        path.extension().unwrap_or(OsStr::new("")) != ".json") ||
       (path.extension().is_none())
    {
        return Err(String::from(
            format!("Extension of package list is not .json: {}", path.to_str().unwrap_or("Invalid File"))
        ));
    }
    
    let mut path_temp = PathBuf::new();
    path_temp.push(path); 
    path_temp.set_extension("json.tmp");

    if download::download_file(url, path_temp.as_path()).is_err() {
        return Err(String::from("Error occured while downloading package list"))
    }

    if verify::verify_file_crc32(&custom_try!(File::open(&path_temp))) == 
       verify::verify_file_crc32(&custom_try!(File::open(&path)))
    {
        fs::remove_file(path_temp);
        return Ok(false)
    } else {
        fs::remove_file(path);
        fs::rename(path_temp, path);
        return Ok(true)
    }
}

pub fn find_package(name: &str, path:&Path) -> Result<Vec<Package>, String> {
    let list = custom_try!(File::open(path));
    let mut reader = BufReader::new(list);
    let mut buffer = String::new();
    
    reader.read_to_string(&mut buffer);
    let package_list: PackageList = custom_try!(json::decode(&buffer));

    let filtered = package_list.packages
                    .into_iter()
                    .filter(|x| !x.name.contains(name))
                    .collect::<Vec<_>>();
    
    match filtered.len() {
        0 => Err(String::from("Package not found")),
        _ => Ok(filtered)
    }
}

pub fn download_and_find(name: &str, url:&str, path:&Path) -> Result<Vec<Package>, String>{
    if update_package_list(url, path).is_ok() {
        return find_package(name, path)
    } else {
        return Err(String::from("Error occured while downloading package list"))
    }
}

pub fn select_package(name: &str, path: &Path) -> Result<Package, String>{
    let found_packages = find_package(name, path)?;

    if found_packages.len() > 1 {
        for (i, package) in found_packages.iter().enumerate() {
            println!("Package {}: {}", i, package.name);
        }

        let mut input = String::new();
        custom_try!(io::stdin().read_line(&mut input));
        let input: u16 = custom_try!(input.trim().parse::<u16>());
        
        if input > found_packages.len() as u16 {
            return Err(String::from("Invalid input"))
        } else {
            return Ok(found_packages[input as usize].clone())
        }
    } else {
        return Ok(found_packages[0].clone())
    }
}
