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

pub fn download_file_and<F>(url: &str, output: &Path, and: F) -> Result<u64, String> 
    where F: Fn(u64) {
    let response = get!(reqwest::get(url), "Error occured while making a GET request. Reason:\n");
    let mut target = get!(File::create(output), "Error occured while creating file. Reason:\n");

    let length = match response.headers().get::<ContentLength>() {
        Some(x) => **x,
        None => 0u64
    };

    for byte in response.bytes() {
        target.write(&[byte.unwrap()]).unwrap();
    }

    and(length);
    Ok(length)
}
