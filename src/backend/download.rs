use std::error::Error;
use std::fs::File;
use std::io::{ Read, Write };
use std::path::Path;

use reqwest;
use reqwest::header::{ ContentLength, Header, HeaderFormat};

macro_rules! get {
    ($x:expr, $y:expr) => (match $x {
        Ok(x) => x,
        Err(why) => return Err($y.to_string() + why.description()),
    });
}

pub fn download_file_while<F>(url: &str, output: &Path, and: F) -> Result<u64, String> 
    where F: Fn(u64, &u64) {
    let response = get!(reqwest::get(url), "Error occured while making a GET request. Reason:\n");
    let mut target = get!(File::create(output), "Error occured while creating file. Reason:\n");

    let length = match response.headers().get::<ContentLength>() {
        Some(x) => **x,
        None => 0u64
    };

    let mut bytes_completed = 0u64;
    for byte in response.bytes() {
        target.write(&[byte.unwrap()]).unwrap();
        bytes_completed += 1;
        and(length, &bytes_completed)
    }

    Ok(length)
}

pub fn download_file_and<F>(url: &str, output: &Path, and: F) -> Result<u64, String> 
    where F: Fn(u64) {
    let length = try!(download_file_while(url, output, |_, _| { }));
    and(length);
    Ok(length)
}

pub fn download_file(url: &str, output: &Path) -> Result<u64, String> {
    download_file_and(url, output, |_| { }) // Do nothing for and. 
}
