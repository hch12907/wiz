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

fn find_package(name: &str, list:&Path) -> Result<String, &str>
{
    let mut list = custom_try!(match File::open(list));
    let mut reader = BufReader::new(list);
    let mut buffer = String::new();

    //TODO: Use JSON.
}