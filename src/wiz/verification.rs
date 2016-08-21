extern crate crc;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::crc::{crc32, Hasher32};

pub fn get_crc32(file: &File) -> u32
{
    let mut digest = crc32::Digest::new(crc32::IEEE);

    for line in file.bytes() {
            digest.write(&[line.unwrap()]);
    }

    digest.sum32()
}


