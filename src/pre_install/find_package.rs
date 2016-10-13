use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => panic!("An error occured during package finding.", why),
    });
}

#[derive(Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    crc32: String,
    dependencies: Vec<Package>
}

#[derive(Serialize, Deserialize)]
struct PackageList {
    packages: Vec<Package>,
    last_updated: Date
}

fn find_package(name: &str, path:&Path) -> Result<String, &str>
{
    let mut list = custom_try!(match File::open(path));
    let mut reader = BufReader::new(list);
    let mut buffer = reader.read_to_string();
    let packages: PackageList = custom_try!(serde_json::from_str(buffer));
}