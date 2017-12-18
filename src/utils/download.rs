use reqwest::{ self, header };
use std::fs::File;
use std::io::{ BufWriter, Read, Write };
use std::path::Path;

use config::Config;
use error::PackageError;

fn get_content_length(url: &str) -> Result<u64, PackageError> {
    // GET the URL, returning an error if necessary.
    let response = reqwest::get(url)?;
    
    // Try to retrieve the content length, and return it.
    Ok(response
    .headers()
    .get::<header::ContentLength>()
    // If the Response contains no content-length, default to 0.
    .map_or(0u64, |x| {
        let header::ContentLength(len) = *x;
        len
    }))
}

pub fn download_file<F, P>(url: &str, to_path: P, callback: F) -> Result<u64, PackageError>
    where F: Fn(u64, u64), P: AsRef<Path> 
{

    // GET the URL, returning an error if necessary.
    let response = reqwest::get(url)?;
    
    // Try to retrieve the content length.
    let content_length = get_content_length(url)?;

    // For every `buffer_size` bytes, call the callback once.
    // `buffer_size` decides how frequent the function calls the callback by
    // checking whether (downloaded bytes) % `buffer_size` is 0.
    let Config { buffer_size, .. } = Config::default();
    let buffer_size = buffer_size.unwrap_or(65536);

    // Obviously, the target file to be written to by the BufWriter.
    let mut target = BufWriter::new(File::create(to_path)?);
    
    for (index, byte) in response.bytes().enumerate() {
        // Write to the target, returns an error if necessary.
        target.write(&[byte?])?;

        // Checks whether it's time to call the callback, and call it if it is.
        // Plus one to `index` as `index` itself is zero-indexed.
        if index as u64 % buffer_size == 0 {
            callback((index + 1) as u64, content_length);
        };
    }

    // In case the callback never got a chance to be called due to the package
    // itself being too small.
    callback(content_length, content_length);

    Ok(content_length)
}

pub fn download_buf(url: &str) -> Result<Vec<u8>, PackageError> {
    // GET the URL, returning an error if necessary.
    let mut response = reqwest::get(url)?;

    // Try to read the Response into memory
    let mut content = Vec::new();
    response.read_to_end(&mut content)?;

    Ok(content)
}