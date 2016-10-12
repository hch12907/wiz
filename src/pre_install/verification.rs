extern crate crc;

use std::io::{ BufRead, BufReader, Read };
use std::fs::File;
use std::path::Path;
use self::crc::{crc32, Hasher32};

pub fn verify_file_crc32(file: &File) -> u32
{
    let mut digest = crc32::Digest::new(crc32::IEEE);

    for line in file.bytes() {
        digest.write(&[line.unwrap()]);
    }

    return digest.sum32();
}