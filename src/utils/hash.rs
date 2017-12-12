use sha2::{Sha512, Digest};
use std::fmt;
use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;

use error::PackageError;

/// This struct represents the hash result.
pub struct Hash(Box<[u8]>);

impl Hash {
    /// Calculates the SHA-512 of a file.
    /// Returns `Box<[u8]>` if there are no errors during the process of
    /// calculation.
    fn sha2<P: AsRef<Path>>(path: P) -> Result<Hash, PackageError> {    
        // Create a SHA512 hasher.
        let mut hasher = Sha512::default();
        // Read the file from `path`
        let file = File::open(path)?;
        let file = BufReader::new(file);
        // Hash the bytes read from the file
        for byte in file.bytes() { hasher.input(&[byte?]) };
        // Return the result (it needs to copy 64 bytes though, ouch)
        Ok(Hash(Box::from(hasher.result().as_slice())))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Hash(ref hash) = self;
        let mut content = String::new();
        for &h in hash.as_ref() {
            content += &format!("{:x}", h)
        };
        write!(f, "{}", content)
    }
}

impl<'a> From<&'a str> for Hash {
    fn from(s: &'a str) -> Self {
        // A simple function which converts hexadecimal chars into `u8`s.
        let hex_to_dec = |x| match x { 
            b'0'...b'9' => x - b'0', 
            y @ b'a'...b'f' => 10 + y - b'a',
            _ => panic!("unable to convert hexadecimal string into decimals"),
        };
        
        let mut result = Vec::with_capacity(64);
        for x in 1 .. 64+1 {
            let (front, _) = s.split_at(2 * x);
            result.push(
                hex_to_dec(front.as_bytes()[0]) * 16 +
                hex_to_dec(front.as_bytes()[1])
            );
        }

        Hash(result.into_boxed_slice())
    }
}