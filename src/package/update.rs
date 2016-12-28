use rustc_serialize::json;

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{ Path, PathBuf };

use backend::{ download, verify };
use package;

///
/// Downloads the package list from `url`, and
/// outputs it to `path`. After the download of
/// package list is complete, invokes `after_download`.
///
fn update_list<F>(url: &str, path: &Path, after_download: F) -> Result<bool, String>
    where F: Fn(u64) {
    let mut download_to = PathBuf::new();
        download_to.push(path);
        download_to.push(get_option!(url.split('/').last(), "An error occured while getting package list filename."));
        
    if download_to.exists() {
        let mut temporary_download_to = PathBuf::new();
            temporary_download_to.push(download_to.as_path());
            temporary_download_to.set_extension(".tmp");
        download::download_file_and(url, temporary_download_to.as_path(), after_download)?;
        
        if verify::verify_file_crc32(download_to.as_path())? == verify::verify_file_crc32(temporary_download_to.as_path())? {
            get!(fs::remove_file(temporary_download_to.as_path()), "An error occured while removing unneeded package list.");
            Ok(false)
        } else {
            get!(fs::remove_file(download_to.as_path()), "An error occured while removing unneeded package list.");
            get!(fs::rename(temporary_download_to.as_path(), download_to.as_path()), "An error occured while renaming package list.");
            Ok(true)
        }
    // Indeed, this function is a giant clusterfuck. Apologies for code-gore.
    } else {
        download::download_file_and(url, download_to.as_path(), after_download)?;
        Ok(true)
    }
}
