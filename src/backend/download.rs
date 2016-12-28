use std::error::Error;
use std::fs::File;
use std::io::{ BufWriter, Read, Write };
use std::path::Path;

use reqwest;
use reqwest::header::ContentLength;

///
/// Downloads file from specified URL, and output it
/// to the path specified in `output`. While downloading
/// the file, for each byte downloaded, it invokes `and`.
/// When the download is complete, it returns a Result that
/// returns file size if download succeeds, or returns error
/// message if download fails.
/// 
pub fn download_file_while<F>(url: &str, output: &Path, and: F) -> Result<u64, String> 
    where F: Fn(u64, &u64) {
        
    // Abandoning this code, as the filename is going to be unpredictable in this case.
    //let mut output_to = PathBuf::new();
    //        output_to.push(output);
    //        output_to.push(url.split('/').last().unwrap_or("_error_"));*/ 

    let response = get!(reqwest::get(url), "Error occured while making a GET request. Reason:\n");
    let target = get!(File::create(output), "Error occured while creating file. Reason:\n");
    let mut target = BufWriter::new(target);

    let length = match response.headers().get::<ContentLength>() {
        Some(x) => **x,
        None => 0u64
    };

    let mut bytes_completed = 0u64;
    for byte in response.bytes() {
        target.write(&[byte.unwrap()]).unwrap();
        bytes_completed += 1;
        and(length, &bytes_completed)
    }

    Ok(length)
}

///
/// Downloads file from specified URL, and output it
/// to the path specified in `output`. After the file is
/// downloaded, it invokes `and`.
/// When the download is complete, it returns a Result that
/// returns file size if download succeeds, or returns error
/// message if download fails. 
///
pub fn download_file_and<F>(url: &str, output: &Path, and: F) -> Result<u64, String> 
    where F: Fn(u64) {
    let length = download_file_while(url, output, |_, _| { })?;
    and(length);
    Ok(length)
}

///
/// Downloads file from specified URL, and output it to
/// the path specified in `output`.
/// When the download is complete, it returns a Result that
/// returns file size if download succeeds, or returns error
/// message if download fails. 
///
pub fn download_file(url: &str, output: &Path) -> Result<u64, String> {
    download_file_and(url, output, |_| { }) // Do nothing for and. 
}
