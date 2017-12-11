use sha2::{Sha512, Digest};
use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;

use error::PackageError;

/// Calculates the SHA-512 of a file.
/// Returns `Box<[u8]>` if there are no errors during the process of
/// calculation.
fn sha2<P: AsRef<Path>>(path: P) -> Result<Box<[u8]>, PackageError> {    
    // Create a SHA512 hasher.
    let mut hasher = Sha512::default();
    // Read the file from `path`
    let file = File::open(path)?;
    let file = BufReader::new(file);
    // Hash the bytes read from the file
    for byte in file.bytes() { hasher.input(&[byte?]) };
    // Return the result (it needs to copy 64 bytes though, ouch)
    Ok(Box::from(hasher.result().as_slice()))
}