use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use crc::{crc32, Hasher32};

/// Verifies the file in `path`, and returns a
/// Result, which returns the CRC32 of the file if
/// the file verification succeeds, or a error message
/// if the file verification failed.
pub fn verify_file_crc32(path: &Path) -> Result<u32, String> {
    let file = get!(File::open(path), "An error occured while opening file. (verify_file)");
    let mut digest = crc32::Digest::new(crc32::IEEE);
    
    for line in file.bytes() {
        digest.write(&[line.unwrap()]);
    }

    Ok(digest.sum32())
}
