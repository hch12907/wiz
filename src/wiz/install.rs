extern crate tar;
extern crate flate2;

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use self::tar::Archive;
use self::flate2::read::GzDecoder;

fn extract_tar(input: &Path, output: &Path) {
    let file = File::open(input).unwrap();
    let mut archive = Archive::new(file);
    archive.unpack(output).unwrap();
}

fn extract_gz(input: &Path, output: &Path) {
    let file = File::open(input).unwrap();
    let buffer = BufReader::new(file);
    let archive = GzDecoder::new(buffer).unwrap();

    let mut buffer = File::create(output).unwrap();
    for byte in archive.bytes() {
        buffer.write(&[byte.unwrap()]);
    }
}

