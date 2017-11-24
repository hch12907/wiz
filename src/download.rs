use reqwest::{ self, header };
use std::fs::File;
use std::io::{ BufWriter, Read, Write };
use std::path::Path;

use error::PackageError;

fn download_file<F>(url: &str, to_path: &Path, callback: F) -> Result<u64, PackageError>
    where F: Fn(u64, u64) 
{
    // GET the URL, returning an error if necessary.
    let response = reqwest::get(url)?;
    
    // Try to retrieve the content length.
    let content_length = 
        response
        .headers()
        .get::<header::ContentLength>()
        // If the Response contains no content-length, default to 0.
        .map_or(0u64, |x| {
            let header::ContentLength(len) = *x;
            len
        });


    // For every `BUFFER_SIZE` bytes, call the callback once.
    const BUFFER_SIZE: u64 = 65536;

    // Obviously, the target file to be written to by the BufWriter.
    let mut target = BufWriter::new(File::create(to_path)?);
    
    for (index, byte) in response.bytes().enumerate() {
        // Write to the target, returns an error if necessary.
        target.write(&[byte?]);

        // Checks whether it's time to call the callback, and call it if it is.
        // Plus one to `index` as `index` itself is zero-indexed.
        if index as u64 % BUFFER_SIZE == 0 {
            callback((index + 1) as u64, content_length);
        };
    }

    // In case the callback never got a chance to be called due to the package
    // itself being too small.
    callback(content_length, content_length);

    Ok(content_length)
}