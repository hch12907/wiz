/*
    TODO:
    1. Copy downloaded files to other places. (Completed)
    2. Verify the downloaded package.
    3. Make it support repos someday.
*/
extern crate hyper;

use self::hyper::Client;
use std::fs::File;
use std::io::{ Read, Write };
use std::path::Path;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => panic!("An error occured during package downloading. {}", why),
    });
}

pub fn download_file(url: &str, output: &Path) {
    let client = Client::new();
    let response = custom_try!(client.get(url).send());
    let mut target = custom_try!(File::create(output));

    for byte in response.bytes() {
        target.write(&[byte.unwrap()]);
    }
}