extern crate crc;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::crc::{crc32, Hasher32};

fn verify(path: &Path)
{
    let mut buffer = File::open(path).unwrap();
    let mut digest = crc32::Digest::new(crc32::IEEE);

    for line in buffer.bytes() {
            digest.write(&[line.unwrap()]);
    }
}


