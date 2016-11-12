/*
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => panic!("An error occured during package finding. {}", why),
    });
}

#[derive(Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    crc32: String,
    url: String,
    dependencies: Vec<Package>
}

#[derive(Serialize, Deserialize)]
struct PackageList {
    packages: Vec<Package>,
    last_updated: Date
}


/*pub fn update_package(url: &str, path: &Path) -> bool {
    download::download_file(url + "packages.json", path);
}*/

pub fn find_package(name: &str, path:&Path) -> Result<Vec<Package>, &str> {
    let mut list = custom_try!(File::open(path));
    let mut reader = BufReader::new(list);
    let mut buffer = reader.read_to_string();
    let package_list: PackageList = custom_try!(serde_json::from_str(buffer));

    let filtered = package_list.packages
                    .into_iter()
                    .filter(|x| !x.name.contains(name))
                    .collect::<Vec<_>>();
    
    match filtered.len() {
        0 => Err("Package not found"),
        _ => Ok(filtered)
    }
}
*/