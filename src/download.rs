use reqwest::{ self, header };
use std::fs::File;
use std::io::{ BufWriter, Read, Write };
use std::path::Path;

use error::PackageError;

/// Enum for representing the state of the package downloads.
pub enum PackageDownload<'a> {
    /// The state is switched to IsFinished when the download of the
    /// package is finished.
    IsFinished {
        /// The path where the downloaded package is located at.
        location: &'a Path,
        /// The total file size of the package.
        file_size: u32,
    },

    /// The state is switched to InProgress when the download of the
    /// package is started.
    InProgress {
        /// The path where the half-downloaded package is located at.
        location: &'a Path,
        /// The progress of the current download process.
        progress: u32,
        /// The total file size of the package.
        file_size: u32,
    },

    /// This is the initial state of PackageDownload, where the package is yet
    /// to be downloaded.
    NotStarted {
        /// The path where the package is going to be located at.
        location: &'a Path,
        /// The URL of where the package is going to be downloaded from.
        url: &'a str,
    }
}

impl<'a> PackageDownload<'a> {
    /// Create a new PackageDownload.
    fn new(url: &'a str, location: &'a Path) -> Self {
        PackageDownload::NotStarted { location, url }
    }

    /// A bool to determine whether the package is downloaded.
    fn is_downloaded(&self) -> bool {
        match self {
            &PackageDownload::IsFinished { .. } => true,
            _ => false
        }
    }

    /// Returns the path of where the package is (or is going to be) located
    /// at.
    fn store_path(&self) -> &'a Path {
        match self {
            &PackageDownload::IsFinished { location: path, .. } => path,
            &PackageDownload::InProgress { location: path, .. } => path,
            &PackageDownload::NotStarted { location: path, .. } => path,
        }
    }
}

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