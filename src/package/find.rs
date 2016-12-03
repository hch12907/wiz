use rustc_serialize::json;

use std::path::{ Path, PathBuf };

use backend::download;
use package;

fn update_list<F>(url: &str, path: &Path, and: F) -> Result<bool, String>
    where F: Fn(u64) {
    let mut download_to = PathBuf::new();
        download_to.push(path);
        download_to.push(match url.split('/').last(){
            Some(x) => x,
            None => return Err("An error occured while getting package list filename.".to_string()) 
        });

    try!(download::download_file_and(url, path, and));
    Ok(true) // TODO: Return false if the package list is not updated.
}