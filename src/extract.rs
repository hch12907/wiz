extern crate tar;
extern crate flate2;

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use self::tar::Archive;
use self::flate2::read::GzDecoder;

macro_rules! custom_try {
    ($x:expr) => (match $x {
        Ok(x) => x,
        Err(why) => panic!("An error occured during extraction. {}", why),
    });
}

pub fn extract_tar(input: &Path, output: &Path) {
    let file = custom_try!(File::open(input));
    let mut archive = Archive::new(file);
    
    archive.unpack(output).ok();
}

pub fn extract_gz(input: &Path, output: &Path) {
    let file = custom_try!(File::open(input));
    let archive = custom_try!(GzDecoder::new(BufReader::new(file)));
    let mut target = custom_try!(File::create(output));

    for byte in archive.bytes() {
        target.write(&[byte.unwrap()]);
    }
}

