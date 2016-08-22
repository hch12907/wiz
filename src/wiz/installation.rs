use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

use constants;
use wiz::verification;
use wiz::extraction;


// TODO: Eliminate hardcoded strings and ints
fn find_package(name: &str) -> Result<String, &str>
{
    let mut list = match File::open(constants::PACKAGE_LIST) {
        Err(why) => panic!("An error occured. \n{}", why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(list);
    let mut buffer = String::new();

    //This is eventually going to be replaced with JSON files or other stuff
    for line in reader.lines() {
        let x = line.unwrap();
        let tokens: Vec<&str> = x.split(' ').collect();

        if(tokens.len() < 2) { 
            continue; 
        }

        if(tokens[0] == name) {
            buffer.push_str(tokens[1]);
            break;
        }
    }

    if(buffer.len() > 0) {
        Ok(buffer)
    }
    else {
        Err("Package not found")
    }
}

fn retrieve_package(url: &str) // -> Path
{
    /*
      This function retrieves the package, and
      puts them in a temporary folder.
    */
}

fn verify_package(package: &File, provided_checksum: u32) -> bool
{
    let checksum = verification::get_crc32(package);
    checksum == provided_checksum
}

fn copy_package(package: &File, destination: &Path)
{
    
}

fn setup_package()
{
    /*
      This function finalises the installation of
      the package, such as setting PATH and such.
    */
}



fn install()
{
    
}