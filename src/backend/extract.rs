use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

/// Extracts the tarball found in the path specified in `input`,
/// and outputs it into path specified in `output`.
/// If the extract process failed, it returns a String containing
/// the error message. 
pub fn extract_tar(input: &Path, output: &Path) -> Result<(), String> {
    let file = get!(File::open(input), "An error occured while opening file. (extract_tar)");
    let mut archive = Archive::new(file);
    archive.unpack(output).ok();
    return Ok(())
}

/// Extracts the gzip found in the path specified in `input`,
/// and outputs it into path specified in `output`.
/// If the extract process failed, it returns a String containing
/// the error message. 
pub fn extract_gz(input: &Path, output: &Path) -> Result<(), String> {
    let file = get!(File::open(input), "An error occured while opening file. (extract_gz) Reason:\n");
    let archive = get!(GzDecoder::new(BufReader::new(file)), "An error occured while creating GzDecoder. Reason:\n");
    let target = get!(File::create(output), "An error occured while creating file. (extract_tar) Reason:\n");
    let mut target = BufWriter::new(target);

    for byte in archive.bytes() {
        target.write(&[byte.unwrap()]).unwrap();
    }

    return Ok(())
}