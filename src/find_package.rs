//extern crate serde_json;
use rustc_serialize::json;

use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, BufRead, Read };
use std::path::Path;
use std::str::FromStr;

use download;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => return Err(format!("An error occured during package finding. {}", why)),
    });
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Package {
    name: String,
    version: String,
    crc32: String,
    url: String,
    dependencies: Vec<Package>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PackageList {
    packages: Vec<Package>,
    version: i32
}


pub fn update_package_list(url: &str, path: &Path) -> Result<bool, String> {
    let download_complete = download::download_file(url, path);
    if(download_complete.is_ok()) {
        //TODO: Return true when the package list is updated, false if not
    }
}

pub fn find_package(name: &str, path:&Path) -> Result<Vec<Package>, String> {
    let mut list = custom_try!(File::open(path));
    let mut reader = BufReader::new(list);
    let mut buffer = String::new();
    
    reader.read_to_string(&mut buffer);
    let package_list: PackageList = custom_try!(json::decode(&buffer));

    let filtered = package_list.packages
                    .into_iter()
                    .filter(|x| !x.name.contains(name))
                    .collect::<Vec<_>>();
    
    match filtered.len() {
        0 => Err(custom_try!(String::from_str("Package not found"))),
        _ => Ok(filtered)
    }
}
