use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use crc::{crc32, Hasher32};

pub fn verify_file_crc32(file: &Path) -> Result<u32, String> {
    let file = get!(File::open(file), "An error occured while opening file. (verify_file)");
    let mut digest = crc32::Digest::new(crc32::IEEE);
    
    for line in file.bytes() {
        digest.write(&[line.unwrap()]);
    }

    Ok(digest.sum32())
}
