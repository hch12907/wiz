use std::error::Error;
use std::fs::File;
use std::io::{ Read, Write };
use std::path::Path;

use reqwest;
use reqwest::header::{ ContentLength, Header, HeaderFormat};

pub fn download_file_and<F>(url: &str, output: &Path, and: F) -> Result<u64, String> 
    where F: Fn(u64)
{
    let response = match reqwest::get(url) {
        Ok(x) => x,
        Err(why) => return Err("Error occured while making a GET request. Reason:".to_string() + why.description())
    };

    let mut target = match File::create(output) {
        Ok(x) => x,
        Err(why) => return Err("Error occured while creating file. Reason:".to_string() + why.description())
    };

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