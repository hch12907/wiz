use constants;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;


// TODO: Eliminate hardcoded strings and ints
fn find_package(name: &str) -> String
{
    let mut list = match File::open(constants::PACKAGE_LIST) {
        Err(why) => panic!("An error occured:\n{}", why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(list);
    let mut buffer = String::new();

    for line in reader.lines() {
        let x = line.unwrap();
        let tokens: Vec<&str> = x.split(' ').collect();

        if tokens.len() < 2 {
            buffer.push_str("NOT FOUND");
        }
        else {   
            if tokens[0] == name {
                buffer.push_str(tokens[1]);
            } 
            else {
                buffer.push_str("NOT FOUND"); // this is only temporary!
            }
        }
    }

    buffer
}

fn retrieve_package(url: &str) // -> Path
{
    /*
      This function retrieves the package, and
      puts them in a temporary folder.
    */
}

fn verify_package(package: &File, provided: &str)
{
    /*
      This function verifies the package(CRC32 IEEE)
      retrieved by retrieve_package, and returns boolean indicating 
      that the file is not corrupted.
    */
}

fn copy_package()
{
    /*
      This function retrieves the package, and put them
      to specified locations accordingly.
    */
}

fn process_package()
{
    /*
      This function finalises the installation of
      the package, such as setting PATH and such.
    */
}

fn install()
{
    
}